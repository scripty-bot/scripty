use std::{
	net::SocketAddr,
	sync::{
		atomic::{AtomicBool, AtomicUsize, Ordering},
		Arc,
	},
	time::Duration,
};

use dashmap::DashMap;
use once_cell::sync::OnceCell;
use tokio::{
	io,
	io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{ModelError, Stream};

pub static LOAD_BALANCER: OnceCell<LoadBalancer> = OnceCell::new();

/// Round-robin load balancer that equally loads all tasks,
/// until one notes that it is overloaded, at which point it is removed from the pool.
///
/// If it notifies the master that it is no longer overloaded, it is re-added.
pub struct LoadBalancer {
	/// The current worker index.
	current_index: AtomicUsize,
	/// A list of all workers.
	workers:       DashMap<usize, LoadBalancedStream>,
}

impl LoadBalancer {
	pub async fn new() -> io::Result<Self> {
		let peer_addresses = scripty_config::get_config()
			.stt_services
			.iter()
			.map(|(addr, port)| SocketAddr::new(addr.parse().unwrap(), *port))
			.enumerate();

		let workers = DashMap::new();
		for (n, addr) in peer_addresses {
			workers.insert(n, LoadBalancedStream::new(addr).await?);
		}
		Ok(Self {
			current_index: AtomicUsize::new(0),
			workers,
		})
	}

	pub async fn get_stream(&self, language: &str, verbose: bool) -> Result<Stream, ModelError> {
		let metrics = scripty_metrics::get_metrics();

		let get_next = || {
			self.current_index
				.fetch_update(Ordering::Release, Ordering::Acquire, |x| {
					if x == self.workers.len() {
						Some(0)
					} else {
						Some(x + 1)
					}
				})
				.expect("fetch_update::{closure} should never return None")
		};

		let mut idx = get_next();
		let mut iter_count: usize = 0;
		let mut do_overload: bool = false;
		let lbs = loop {
			if let Some(lbs) = self.workers.get(&idx) {
				if (do_overload && lbs.can_overload) || !lbs.is_overloaded() {
					// usually this is going to be the fast path and it will immediately return this worker
					// if it isn't, this is still decently fast, an O(2n) operation worst case
					// given there's very likely never going to be more than 255 workers, this is fine
					break lbs;
				}
			}

			idx = get_next();
			// the not op here might seem redundant, but it's added to save a few instructions at the assembly level
			if !do_overload && iter_count > self.workers.len() {
				do_overload = true;
			}
			iter_count += 1;
			if iter_count > 1024 {
				metrics.stt_server_fetch_failure.inc_by(1);
				return Err(ModelError::NoAvailableServers);
			}
		};

		match lbs.open_connection(language, verbose).await {
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
}

pub struct LoadBalancedStream {
	peer_address:  SocketAddr,
	is_overloaded: Arc<AtomicBool>,
	can_overload:  bool,
}

impl LoadBalancedStream {
	#[inline]
	pub fn is_overloaded(&self) -> bool {
		self.is_overloaded.load(Ordering::Relaxed)
	}

	pub(crate) async fn open_connection(
		&self,
		language: &str,
		verbose: bool,
	) -> Result<Stream, ModelError> {
		if !self.can_overload && self.is_overloaded() {
			return Err(ModelError::Io(io::Error::new(
				io::ErrorKind::Other,
				"remote is overloaded",
			)));
		}

		Stream::new(language, verbose, self.peer_address).await
	}

	pub async fn new(peer_address: SocketAddr) -> io::Result<Self> {
		// open a connection to the remote
		let mut peer_stream = tokio::net::TcpStream::connect(peer_address).await?;

		// convert this connection into a data-only connection (send 0x04)
		peer_stream.write_u8(0x04).await?;

		// wait for a response of 0x06 (status connection open, fields max_utilization: f64, can_overload: bool)
		if peer_stream.read_u8().await? != 0x06 {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				"unexpected response from server",
			));
		}

		// read the fields
		let max_utilization = peer_stream.read_f64().await?;
		let can_overload = peer_stream.read_u8().await? == 1;

		debug!(
			?max_utilization,
			?can_overload,
			?peer_address,
			"got data for new stream"
		);

		let is_overloaded = Arc::new(AtomicBool::new(false));
		let iso2 = Arc::clone(&is_overloaded);

		// spawn a background task that will monitor the connection, and if it reports being overloaded, sets the overloaded flag
		tokio::spawn(async move {
			let metrics = scripty_metrics::get_metrics();
			let mut peer_stream = peer_stream;
			loop {
				let data: u8 = tokio::select! {
					data_type = peer_stream.read_u8() => {
						match data_type {
							Ok(d) => d,
							Err(e) => {
								error!("error reading from peer: {}", e);
								// try to reconnect
								peer_stream = match tokio::net::TcpStream::connect(peer_address).await {
									Ok(s) => s,
									Err(e) => {
										error!("error reconnecting to peer: {}", e);
										metrics.stt_server_fetch_failure.inc_by(1);
										const ONE_SECOND: Duration = Duration::from_secs(1);
										tokio::time::sleep(ONE_SECOND).await;
										continue;
									}
								};
								continue;
							}
						}
					},
					_ = tokio::signal::ctrl_c() => {
						break
					}
				};

				assert_eq!(data, 0x07);
				metrics.stt_server_fetch_success.inc_by(1);

				// read payload (utilization: f64)
				let utilization = match peer_stream.read_f64().await {
					Ok(u) => u,
					Err(e) => {
						error!("error reading from peer: {}", e);
						// toss the error to the handler which will try to reconnect or exit
						continue;
					}
				};

				// if the utilization is above the threshold, set the overloaded flag
				iso2.store(utilization > max_utilization, Ordering::Relaxed);
			}
			// write 0x03 to the stream to close the connection
			if let Err(e) = peer_stream.write_u8(0x03).await {
				error!("error closing connection to {}: {}", peer_address, e);
			}
		});

		Ok(Self {
			peer_address,
			is_overloaded,
			can_overload,
		})
	}
}
