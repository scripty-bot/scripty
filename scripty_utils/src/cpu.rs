use std::io;
use std::time::Duration;
use systemstat::{BlockDeviceStats, ByteSize, CPULoad, Platform, System};

#[derive(Default, Debug)]
pub struct SystemInformation {
    /// Current CPU temperature in degrees Celsius
    cpu_temp: Option<f32>,

    /// Block device statistics
    ///
    /// Summed total of all block devices on the system
    block_dev_stats: BlockDevStats,

    /// CPU load statistics
    ///
    /// Average over all CPU cores
    cpu_load: CpuLoadStats,

    /// CPU load average statistics
    load_average: CpuLoadAvg,

    /// Memory usage statistics
    memory_usage: MemoryStats,

    /// Network usage statistics
    network_usage: NetworkStats,

    /// Socket statistics
    socket_stats: SocketStats,

    /// System uptime in nanoseconds
    uptime: Option<u128>,
}

impl SystemInformation {
    pub async fn get_stats() -> Self {
        let sys = systemstat::System::new();
        let mut sys_info = SystemInformation::default();

        sys_info.cpu_temp = match sys.cpu_temp() {
            Ok(temp) => Some(temp),
            Err(e) => {
                warn!("Failed to get CPU temperature: {}", e);
                None
            }
        };

        sys_info.block_dev_stats = match BlockDevStats::get_stats(Some(&sys)) {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to get block device stats: {}", e);
                BlockDevStats::default()
            }
        };

        sys_info.cpu_load = match CpuLoadStats::get_stats_async(Some(&sys)).await {
            Ok(load) => load,
            Err(e) => {
                warn!("Failed to get CPU load: {}", e);
                CpuLoadStats::default()
            }
        };

        sys_info.memory_usage = match MemoryStats::get_stats(Some(&sys)) {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to get memory stats: {}", e);
                MemoryStats::default()
            }
        };

        sys_info.network_usage = match NetworkStats::get_stats(Some(&sys)) {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to get network stats: {}", e);
                NetworkStats::default()
            }
        };

        sys_info.socket_stats = match SocketStats::get_stats(Some(&sys)) {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to get socket stats: {}", e);
                SocketStats::default()
            }
        };

        sys_info.uptime = match sys.uptime() {
            Ok(uptime) => Some(uptime.as_nanos()),
            Err(e) => {
                warn!("Failed to get uptime: {}", e);
                None
            }
        };

        sys_info
    }
}

#[derive(Default, Debug)]
pub struct BlockDevStats {
    pub name: String,
    pub read_ios: usize,
    pub read_merges: usize,
    pub read_sectors: usize,
    pub read_ticks: usize,
    pub write_ios: usize,
    pub write_merges: usize,
    pub write_sectors: usize,
    pub write_ticks: usize,
    pub in_flight: usize,
    pub io_ticks: usize,
    pub time_in_queue: usize,
}

impl BlockDevStats {
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };

        let block_dev_stats = sys.block_device_statistics()?;

        debug!("Found {} block devices", block_dev_stats.len());

        if block_dev_stats.len() == 0 {
            warn!("No block devices found? There should be at least one.");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No block devices found",
            ));
        }

        let mut this = Self::default();

        for (dev, stats) in block_dev_stats {
            debug!("Logging stats for {}", dev);

            this.read_ios += stats.read_ios;
            this.read_merges += stats.read_merges;
            this.read_sectors += stats.read_sectors;
            this.read_ticks += stats.read_ticks;
            this.write_ios += stats.write_ios;
            this.write_merges += stats.write_merges;
            this.write_sectors += stats.write_sectors;
            this.write_ticks += stats.write_ticks;
            this.in_flight += stats.in_flight;
            this.io_ticks += stats.io_ticks;
            this.time_in_queue += stats.time_in_queue;
        }

        Ok(this)
    }
}

#[derive(Default, Debug)]
pub struct CpuLoadStats {
    user: f32,
    nice: f32,
    system: f32,
    interrupt: f32,
    idle: f32,
    io_wait: f32,
}

impl CpuLoadStats {
    /// Get CPU load information. This has a blocking call to `std::thread::sleep`. Do not use in async contexts.
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let cpu_load_measurement = sys.cpu_load_aggregate()?;
        std::thread::sleep(Duration::from_secs(1));
        let cpu_load = cpu_load_measurement.done()?;

        Ok(Self {
            user: cpu_load.user,
            nice: cpu_load.nice,
            system: cpu_load.system,
            interrupt: cpu_load.interrupt,
            idle: cpu_load.idle,
            io_wait: cpu_load.platform.iowait,
        })
    }

    /// Get CPU load information asynchronously. This has a non-blocking call to `tokio::time::sleep`.
    pub async fn get_stats_async(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let cpu_load_measurement = sys.cpu_load_aggregate()?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let cpu_load = cpu_load_measurement.done()?;

        Ok(Self {
            user: cpu_load.user,
            nice: cpu_load.nice,
            system: cpu_load.system,
            interrupt: cpu_load.interrupt,
            idle: cpu_load.idle,
            io_wait: cpu_load.platform.iowait,
        })
    }
}

#[derive(Default, Debug)]
pub struct CpuLoadAvg {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

impl CpuLoadAvg {
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let cpu_load = sys.load_average()?;

        Ok(Self {
            one: cpu_load.one,
            five: cpu_load.five,
            fifteen: cpu_load.fifteen,
        })
    }
}

#[derive(Default, Debug)]
pub struct MemoryStats {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub used: u64,
    pub buffers: u64,
    pub cached: u64,
}

impl MemoryStats {
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let mem_stats = sys.memory()?;

        let total = mem_stats.total.0;
        let free = mem_stats.free.0;
        let available = mem_stats
            .platform_memory
            .meminfo
            .get("MemAvailable")
            .unwrap_or(&ByteSize(0))
            .0;
        let used = mem_stats.total - mem_stats.free;
        let buffers = mem_stats
            .platform_memory
            .meminfo
            .get("Buffers")
            .unwrap_or(&ByteSize(0))
            .0;
        let cached = mem_stats
            .platform_memory
            .meminfo
            .get("Cached")
            .unwrap_or(&ByteSize(0))
            .0;

        Ok(Self {
            total,
            free,
            available,
            used,
            buffers,
            cached,
        })
    }
}

#[derive(Default, Debug)]
pub struct NetworkStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
    rx_errors: u64,
    tx_errors: u64,
}

impl NetworkStats {
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let ifaces = sys.networks()?;

        let mut this = Self::default();

        for iface in ifaces.values() {
            let iface_stats = sys.network_stats(iface.name)?;
            this.rx_bytes += iface_stats.rx_bytes.0;
            this.tx_bytes += iface_stats.tx_bytes.0;
            this.rx_packets += iface_stats.rx_packets;
            this.tx_packets += iface_stats.tx_packets;
            this.rx_errors += iface_stats.rx_errors;
            this.tx_errors += iface_stats.tx_errors;
        }

        Ok(this)
    }
}

#[derive(Default, Debug)]
pub struct SocketStats {
    pub tcp_sockets_in_use: usize,
    pub tcp_sockets_orphaned: usize,
    pub tcp6_sockets_in_use: usize,

    pub udp_sockets_in_use: usize,
    pub udp6_sockets_in_use: usize,
}

impl SocketStats {
    pub fn get_stats(sys: Option<&System>) -> io::Result<Self> {
        let sys = match sys {
            Some(sys) => sys,
            None => System::new(),
        };
        let socket_stats = sys.socket_stats()?;

        Ok(Self {
            tcp_sockets_in_use: socket_stats.tcp_sockets_in_use,
            tcp_sockets_orphaned: socket_stats.tcp_sockets_orphaned,
            tcp6_sockets_in_use: socket_stats.tcp6_sockets_in_use,
            udp_sockets_in_use: socket_stats.udp_sockets_in_use,
            udp6_sockets_in_use: socket_stats.udp6_sockets_in_use,
        })
    }
}
