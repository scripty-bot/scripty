use std::{
	collections::VecDeque,
	net::SocketAddr,
	sync::{
		Arc,
		Mutex,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	time::Duration,
};

use byteorder::NetworkEndian;
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use scripty_common::stt_transport_models::{
	ClientToServerMessage,
	ServerToClientMessage,
	StatusConnectionData,
	StatusConnectionOpen,
};
use scripty_config::SttServiceDefinition;
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::{
		TcpStream,
		lookup_host,
		tcp::{OwnedReadHalf, OwnedWriteHalf},
	},
	sync::broadcast::{Receiver, Sender},
};

use crate::{ModelError, NUM_STT_SERVICE_TRIES, Stream};

/// Maximum number of workers to queue up.
///
/// Takes roughly 60ms after TCP RTT to establish a connection to a server.
/// Scripty's analytics put all time peak usage at 11 simultaneous streams.
///
/// 11 concurrent streams * 60ms TCP RTT = 660ms / 20ms packet length = 33 queue slots,
/// rounded down to 32.
const MAXIMUM_QUEUE_SIZE: usize = 32;

pub static LOAD_BALANCER: OnceCell<LoadBalancer> = OnceCell::new();

/// Round-robin load balancer that equally loads all tasks,
/// until one notes that it is overloaded, at which point it is removed from the pool.
///
/// If it notifies the master that it is no longer overloaded, it is re-added.
#[derive(Clone)]
pub struct LoadBalancer {
	/// The current worker index.
	current_index:  Arc<AtomicUsize>,
	/// A list of all workers.
	workers:        Arc<DashMap<usize, LoadBalancedStream>>,
	/// Queued-up workers ready for use.
	///
	/// This is used to prevent dropping a few hundred milliseconds of audio at the very start of a stream.
	/// If a worker is queued up, it is ready to be used immediately.
	queued_workers: Arc<Mutex<VecDeque<Stream>>>,
	/// Channel to request a new worker be queued up.
	///
	/// Allows avoiding busy waiting in the background task.
	new_worker_tx:  flume::Sender<()>,
}

impl LoadBalancer {
	pub async fn new() -> Result<Self, ModelError> {
		let stt_services = scripty_config::get_config().stt_services.clone();
		let mut peer_addresses: Vec<SocketAddr> = Vec::new();
		for service in stt_services {
			match service {
				SttServiceDefinition::HostString(host) => peer_addresses.extend(
					lookup_host(host)
						.await
						.expect("Could not resolve stt hostname"),
				),
				SttServiceDefinition::IPTuple(addr, port) => peer_addresses.push(SocketAddr::new(
					addr.parse()
						.expect("Could not parse IP address for stt server"),
					port,
				)),
			}
		}

		let workers = Arc::new(DashMap::new());
		let (purge_tx, purge_rx) = flume::bounded(1);
		for (n, addr) in peer_addresses.into_iter().enumerate() {
			workers.insert(n, LoadBalancedStream::new(addr, purge_tx.clone()).await?);
		}
		let (new_worker_tx, new_worker_rx) = flume::unbounded();
		let this = Self {
			current_index: Arc::new(AtomicUsize::new(0)),
			workers,
			queued_workers: Arc::new(Mutex::new(VecDeque::with_capacity(MAXIMUM_QUEUE_SIZE))),
			new_worker_tx,
		};
		let t2 = this.clone();
		tokio::spawn(t2.new_worker_background_task(new_worker_rx));
		let t3 = this.clone();
		tokio::spawn(async move {
			loop {
				if purge_rx.recv_async().await.is_ok() {
					t3.queued_workers
						.lock()
						.unwrap_or_else(|poisoned| {
							warn!("queued workers lock was poisoned!");
							poisoned.into_inner()
						})
						.clear();
					// request the queue be refilled
					if t3.new_worker_tx.send_async(()).await.is_err() {
						break error!(
							"error sending new worker request: all client queues dropped"
						);
					}
				}
			}
		});
		Ok(this)
	}

	fn get_next_worker_idx(&self) -> usize {
		self.current_index
			.fetch_update(Ordering::Release, Ordering::Acquire, |x| {
				if x == self.workers.len() {
					Some(0)
				} else {
					Some(x + 1)
				}
			})
			.expect("get_next_worker_idx::{closure} should never return None")
	}

	fn find_worker(&self) -> Result<usize, ModelError> {
		let mut idx = self.get_next_worker_idx();
		let mut iter_count: usize = 0;
		let mut allow_overload = false;

		loop {
			if let Some(worker) = self.workers.get(&idx) {
				// if we're allowing overloading, or this worker isn't overloaded and isn't in error
				if (allow_overload && worker.can_overload)
					|| !worker.is_overloaded() && !worker.is_in_error()
				{
					// usually this is going to be the fast path, and it will immediately return this worker.
					// if it isn't, this is still decently fast, an O(2n) operation worst case.
					// given there's very likely never going to be more than 255 workers, this is fine
					return Ok(idx);
				}
			}

			idx = self.get_next_worker_idx();

			// are we back at the start?
			if !allow_overload && iter_count > self.workers.len() {
				// we've looped through all workers, and none are available:
				// try again, but this time, allow overloading
				allow_overload = true;
			}

			iter_count += 1;

			if iter_count > NUM_STT_SERVICE_TRIES {
				// failed to find any available workers
				// give up and return an error
				scripty_metrics::get_metrics()
					.stt_server_fetch_failure
					.inc_by(1);
				error!(
					"no available STT servers after {} tries",
					NUM_STT_SERVICE_TRIES
				);
				return Err(ModelError::NoAvailableServers);
			}
		}
	}

	async fn spawn_new_stream(&self) -> Result<Stream, ModelError> {
		let worker_id = self.find_worker()?;
		let worker = self.workers.get(&worker_id).expect("worker should exist");

		let metrics = scripty_metrics::get_metrics();
		match worker.open_connection().await {
			Ok(s) => {
				metrics.stt_server_fetch_success.inc_by(1);
				Ok(s)
			}
			Err(e) => {
				metrics.stt_server_fetch_failure.inc_by(1);
				Err(e)
			}
		}
	}

	async fn new_worker_background_task(self, new_worker_rx: flume::Receiver<()>) {
		loop {
			{
				// check if we have reached the maximum queue size
				if self
					.queued_workers
					.lock()
					.unwrap_or_else(|poisoned| {
						warn!("queued workers lock was poisoned!");
						poisoned.into_inner()
					})
					.len() >= MAXIMUM_QUEUE_SIZE
				{
					// wait for a new worker to be requested
					if new_worker_rx.recv_async().await.is_err() {
						error!("all clients disconnected (should never happen)");
						return;
					};
				}
				debug!(
					"got request for new worker, {} queued",
					self.queued_workers
						.lock()
						.unwrap_or_else(|poisoned| {
							warn!("queued workers lock was poisoned!");
							poisoned.into_inner()
						})
						.len()
				);
			}

			// spawn a new worker
			let new_worker = match self.spawn_new_stream().await {
				Ok(s) => s,
				Err(e) => {
					error!("failed to spawn new worker: {}", e);
					continue;
				}
			};
			self.queued_workers
				.lock()
				.unwrap_or_else(|poisoned| {
					warn!("queued workers lock was poisoned!");
					poisoned.into_inner()
				})
				.push_back(new_worker);
		}
	}

	pub async fn get_stream(&self) -> Result<Stream, ModelError> {
		// check if we have any queued workers
		{
			let mut queued_workers = self.queued_workers.lock().unwrap_or_else(|poisoned| {
				warn!("queued workers lock was poisoned!");
				poisoned.into_inner()
			});
			if let Some(worker) = queued_workers.pop_front() {
				// request a new worker to be queued up
				let new_worker_queue = self.new_worker_tx.clone();
				tokio::spawn(async move { new_worker_queue.send_async(()).await });

				// return the one we got
				return Ok(worker);
			}
		}

		// spawn a new worker
		let new_worker = match self.spawn_new_stream().await {
			Ok(s) => s,
			Err(e) => {
				error!("failed to spawn new worker: {}", e);
				return Err(e);
			}
		};
		Ok(new_worker)
	}
}

pub struct LoadBalancedStream {
	peer_address:           SocketAddr,
	is_overloaded:          Arc<AtomicBool>,
	can_overload:           bool,
	waiting_for_new_stream: Arc<AtomicBool>,
	is_errored:             Arc<AtomicBool>,

	msg_tx:                 Sender<ClientToServerMessage>,
	msg_rx_transmit_handle: Sender<ServerToClientMessage>,
	// keep this field that way there's always one receiver
	_msg_rx:                Receiver<ServerToClientMessage>,

	purge_tx: flume::Sender<()>,
}

impl LoadBalancedStream {
	pub fn is_overloaded(&self) -> bool {
		self.is_overloaded.load(Ordering::Relaxed)
	}

	pub fn is_in_error(&self) -> bool {
		self.waiting_for_new_stream.load(Ordering::Relaxed)
			|| self.is_errored.load(Ordering::Relaxed)
	}

	pub async fn open_connection(&self) -> Result<Stream, ModelError> {
		if !self.can_overload && self.is_overloaded() {
			return Err(ModelError::OverloadedRemote);
		}

		let res = Stream::new(
			self.peer_address,
			self.msg_tx.clone(),
			self.msg_rx_transmit_handle.subscribe(),
			self.purge_tx.clone(),
		)
		.await;
		self.is_errored.store(res.is_err(), Ordering::Relaxed);
		res
	}

	pub async fn new(
		peer_address: SocketAddr,
		purge_tx: flume::Sender<()>,
	) -> Result<Self, ModelError> {
		// open a connection to the remote
		info!("trying to connect to STT service at {}", peer_address);
		let peer_stream = TcpStream::connect(peer_address).await?;
		let (mut stream_read, stream_write) = peer_stream.into_split();

		// wait for the server to send a StatusConnectionOpen message
		info!(%peer_address, "waiting for initialization");
		let ServerToClientMessage::StatusConnectionOpen(StatusConnectionOpen {
			max_utilization,
			can_overload,
		}) = read_socket_message(&mut stream_read).await?
		else {
			// got something other than a StatusConnectionOpen message
			// should never happen
			return Err(ModelError::PayloadOutOfOrder);
		};

		info!(
			?max_utilization,
			?can_overload,
			?peer_address,
			"got data for new stream"
		);

		// spawn background tx and rx tasks
		let (client_to_server_tx, client_to_server_rx) = tokio::sync::broadcast::channel(16384);
		let (server_to_client_tx, server_to_client_rx) = tokio::sync::broadcast::channel(16384);
		// error handling queue
		let (stream_error_tx, mut stream_error_rx) = tokio::sync::mpsc::channel(2);
		let (new_read_stream_tx, new_read_stream_rx) = tokio::sync::mpsc::channel(1);
		let (new_write_stream_tx, new_write_stream_rx) = tokio::sync::mpsc::channel(1);

		// read stream task
		struct ReadStreamTask {
			stream_read:         OwnedReadHalf,
			server_to_client_tx: tokio::sync::broadcast::Sender<ServerToClientMessage>,
			stream_error_tx:     tokio::sync::mpsc::Sender<ModelError>,
			new_read_stream_rx:  tokio::sync::mpsc::Receiver<OwnedReadHalf>,
		}
		let mut read_stream_task = ReadStreamTask {
			stream_read,
			server_to_client_tx: server_to_client_tx.clone(),
			stream_error_tx: stream_error_tx.clone(),
			new_read_stream_rx,
		};
		tokio::spawn(async move {
			'outer: loop {
				let error = 'inner: loop {
					let message = tokio::select! {
						biased;
						new_handle = read_stream_task.new_read_stream_rx.recv() => {
							// always swap handles before trying to send anything new
							match new_handle {
								Some(stream) => {
									read_stream_task.stream_read = stream;
									continue 'inner;
								}
								None => {
									error!(%peer_address,
										"error receiving new stream from error queue: error task exited early"
									);
									break 'outer;
								}
							}
						}
						message = read_socket_message(&mut read_stream_task.stream_read) => {
							message
						}
					};
					match message {
						Ok(message) => {
							debug!("got message: {:?}", message);
							if let Err(e) = read_stream_task.server_to_client_tx.send(message) {
								error!(%peer_address,
									"error sending message to client: no remaining receivers: {}",
									e
								);
								break 'outer; // no remaining receivers, thus we are done
							}
						}
						Err(e) => {
							error!(%peer_address,"error reading message from server: {}", e);
							break 'inner e;
						}
					}
				};

				// error with stream, send our stream to the error queue and wait for a new one back
				if let Err(e) = read_stream_task.stream_error_tx.send(error).await {
					error!(
						%peer_address,
						"error sending error to error queue: error task exited early: {}",
						e
					);
					break 'outer;
				}
				// wait for a new stream to be sent back
				match read_stream_task.new_read_stream_rx.recv().await {
					Some(stream) => read_stream_task.stream_read = stream,
					None => {
						error!(
							%peer_address,
							"error receiving new stream from error queue: error task exited early"
						);
						break 'outer;
					}
				}
			}
		});

		// write stream task
		struct WriteStreamTask {
			stream_write:        OwnedWriteHalf,
			client_to_server_rx: tokio::sync::broadcast::Receiver<ClientToServerMessage>,
			stream_error_tx:     tokio::sync::mpsc::Sender<ModelError>,
			new_write_stream_rx: tokio::sync::mpsc::Receiver<OwnedWriteHalf>,
		}
		let mut write_stream_task = WriteStreamTask {
			stream_write,
			client_to_server_rx,
			stream_error_tx,
			new_write_stream_rx,
		};
		tokio::spawn(async move {
			'outer: loop {
				let error = 'inner: loop {
					let message = tokio::select! {
						biased;
						new_handle = write_stream_task.new_write_stream_rx.recv() => {
							// always swap handles before trying to send anything new
							match new_handle {
								Some(stream) => {
									write_stream_task.stream_write = stream;
									continue 'inner;
								}
								None => {
									error!(%peer_address,
										"error receiving new stream from error queue: error task exited early"
									);
									break 'outer;
								}
							}
						}
						message = write_stream_task.client_to_server_rx.recv() => {
							message
						},
					};
					match message {
						Ok(message) => {
							if let Err(e) =
								write_socket_message(&mut write_stream_task.stream_write, &message)
									.await
							{
								error!(%peer_address, "error sending message to server: {}", e);
								break 'inner e;
							}
						}
						Err(e) => {
							error!(
								%peer_address,
								"error reading message from client: no remaining transmitters: {}",
								e
							);
							break 'outer; // no remaining transmitters, thus we are done
						}
					}
				};

				// error with stream, send our stream to the error queue and wait for a new one back
				if let Err(e) = write_stream_task.stream_error_tx.send(error).await {
					error!(%peer_address,
						"error sending error to error queue: error task exited early: {}",
						e
					);
					break 'outer;
				}

				// wait for a new stream to be sent back
				match write_stream_task.new_write_stream_rx.recv().await {
					Some(stream) => write_stream_task.stream_write = stream,
					None => {
						error!(
							%peer_address,
							"error receiving new stream from error queue: error task exited early",
						);
						break 'outer;
					}
				}
			}
		});

		let waiting_for_new_stream = Arc::new(AtomicBool::new(false));
		let wfns2 = Arc::clone(&waiting_for_new_stream);
		let purge_tx2 = purge_tx.clone();
		// error handling task
		tokio::spawn(async move {
			loop {
				let _error = stream_error_rx.recv().await;
				warn!("got error from stream pair");
				wfns2.store(true, Ordering::Relaxed);

				// immediately purge all queued workers as we have bad state
				if purge_tx2.send_async(()).await.is_err() {
					error!("error sending purge request: all client queues dropped");
					break;
				}

				// start a new connection to the server
				let mut peer_stream = None;
				for n in 0..=12 {
					// try 12 times to connect to the server with exponential backoff
					let maybe_stream = TcpStream::connect(peer_address).await;
					match maybe_stream {
						Ok(stream) => {
							peer_stream = Some(stream);
							break;
						}
						Err(e) => {
							error!(%peer_address, "error connecting to server: {}", e);
							tokio::time::sleep(Duration::from_secs(2_u64.pow(n))).await;
						}
					}
				}
				let peer_stream = match peer_stream {
					Some(stream) => stream,
					None => {
						error!(%peer_address, "failed to connect to server");
						break;
					}
				};
				let (stream_read, stream_write) = peer_stream.into_split();
				// send the new streams to the read and write tasks
				let _ = new_read_stream_tx.send(stream_read).await;
				let _ = new_write_stream_tx.send(stream_write).await;
				wfns2.store(false, Ordering::Relaxed);

				// purge again to ensure we clear out any queued workers that were on bad streams
				if purge_tx2.send_async(()).await.is_err() {
					error!("error sending purge request: all client queues dropped");
					break;
				}
			}
		});

		let is_overloaded = Arc::new(AtomicBool::new(false));
		let iso2 = Arc::clone(&is_overloaded);
		let mut server_to_client_rx2 = server_to_client_tx.subscribe();
		// monitoring task
		tokio::spawn(async move {
			loop {
				let Ok(res) = server_to_client_rx2.recv().await else {
					// error happened, we are never going to get any more messages
					break;
				};
				if let ServerToClientMessage::StatusConnectionData(StatusConnectionData {
					utilization,
				}) = res
				{
					iso2.store(utilization > max_utilization, Ordering::Relaxed);
				}
			}
		});

		let is_errored = Arc::new(AtomicBool::new(false));
		let ie2 = Arc::clone(&is_errored);
		let cts2 = client_to_server_tx.clone();
		let stc2 = server_to_client_tx.clone();
		let ptx2 = purge_tx.clone();
		// If in error state, clear out the queue
		// and also try creating a new worker every few seconds.
		// When one does succeed, unset the flag
		tokio::spawn(async move {
			loop {
				if ie2.load(Ordering::Relaxed) {
					// try fetching a new worker
					match Stream::new(peer_address, cts2.clone(), stc2.subscribe(), ptx2.clone())
						.await
					{
						Ok(_) => {
							ie2.store(false, Ordering::Relaxed);
						}
						Err(e) => {
							error!("error fetching new worker: {}", e);
						}
					}
				}
				tokio::time::sleep(Duration::from_secs(5)).await;
			}
		});

		Ok(Self {
			peer_address,
			is_overloaded,
			can_overload,
			waiting_for_new_stream,
			msg_tx: client_to_server_tx,
			msg_rx_transmit_handle: server_to_client_tx,
			_msg_rx: server_to_client_rx,
			purge_tx,
			is_errored,
		})
	}
}

async fn read_socket_message(
	socket: &mut OwnedReadHalf,
) -> Result<ServerToClientMessage, ModelError> {
	// read the magic bytes
	let mut magic = [0; 4];
	socket.read_exact(&mut magic).await?;
	if magic != scripty_common::MAGIC_BYTES {
		return Err(ModelError::InvalidMagicBytes(magic));
	}

	// read the data length
	let mut data_length_bytes = [0; 8];
	socket.read_exact(&mut data_length_bytes).await?;
	let data_length = {
		use byteorder::ByteOrder;
		NetworkEndian::read_u64(&data_length_bytes)
	};

	// read the data
	let mut data = vec![0; data_length as usize];
	socket.read_exact(&mut data).await?;

	// deserialize the data
	Ok(rmp_serde::from_slice(&data)?)
}

async fn write_socket_message(
	socket: &mut OwnedWriteHalf,
	message: &ClientToServerMessage,
) -> Result<(), ModelError> {
	// serialize the message
	let mut data = Vec::new();
	rmp_serde::encode::write(&mut data, message)?;

	// write the magic bytes
	socket.write_all(&scripty_common::MAGIC_BYTES).await?;

	// write the data length
	socket.write_u64(data.len() as u64).await?;

	// write the data
	socket.write_all(&data).await?;

	// flush the socket
	socket.flush().await?;

	Ok(())
}
