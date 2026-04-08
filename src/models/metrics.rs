//! System metrics data structures
//!
//! Defines all metric types collected by the system monitors.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Complete system metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub processes: Vec<ProcessMetrics>,
    pub network: NetworkMetrics,
    pub disks: Vec<DiskMetrics>,
    pub gpu: Option<GpuMetrics>,
    pub temperatures: Vec<TemperatureMetrics>,
    pub power: Option<PowerMetrics>,
}

/// CPU metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Overall CPU usage percentage (0-100)
    pub usage_percent: f32,
    /// Per-core usage percentages
    pub per_core_usage: Vec<f32>,
    /// CPU frequency in MHz
    pub frequency_mhz: u64,
    /// Number of logical cores
    pub core_count: usize,
    /// Load averages (1, 5, 15 minutes)
    pub load_average: (f64, f64, f64),
}

/// Memory metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total RAM in bytes
    pub total_bytes: u64,
    /// Used RAM in bytes
    pub used_bytes: u64,
    /// Available RAM in bytes
    pub available_bytes: u64,
    /// Usage percentage (0-100)
    pub usage_percent: f32,
    /// Total swap in bytes
    pub swap_total_bytes: u64,
    /// Used swap in bytes
    pub swap_used_bytes: u64,
    /// Cached memory in bytes
    pub cached_bytes: u64,
    /// Buffers in bytes
    pub buffers_bytes: u64,
}

/// Process metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Memory usage percentage
    pub memory_percent: f32,
    /// Process status
    pub status: String,
    /// Parent process ID
    pub parent_pid: Option<u32>,
    /// Number of threads
    pub thread_count: usize,
    /// Process start time
    pub start_time: DateTime<Utc>,
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Network interfaces
    pub interfaces: Vec<NetworkInterface>,
    /// Total bytes received
    pub total_rx_bytes: u64,
    /// Total bytes transmitted
    pub total_tx_bytes: u64,
    /// Current download speed (bytes/sec)
    pub rx_speed_bps: u64,
    /// Current upload speed (bytes/sec)
    pub tx_speed_bps: u64,
}

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub is_up: bool,
}

/// Disk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Mount point
    pub mount_point: String,
    /// Device name
    pub device: String,
    /// File system type
    pub fs_type: String,
    /// Total space in bytes
    pub total_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Usage percentage (0-100)
    pub usage_percent: f32,
    /// Read speed (bytes/sec)
    pub read_speed_bps: u64,
    /// Write speed (bytes/sec)
    pub write_speed_bps: u64,
}

/// GPU metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMetrics {
    /// GPU name
    pub name: String,
    /// GPU utilization percentage (0-100)
    pub usage_percent: f32,
    /// Memory total in bytes
    pub memory_total_bytes: u64,
    /// Memory used in bytes
    pub memory_used_bytes: u64,
    /// Memory usage percentage (0-100)
    pub memory_percent: f32,
    /// GPU temperature in Celsius
    pub temperature_celsius: f32,
    /// Power usage in watts
    pub power_watts: f32,
    /// Fan speed percentage (0-100)
    pub fan_speed_percent: f32,
}

/// Temperature metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureMetrics {
    /// Sensor label
    pub label: String,
    /// Temperature in Celsius
    pub temperature_celsius: f32,
    /// Critical temperature threshold
    pub critical_celsius: Option<f32>,
    /// Maximum temperature threshold
    pub max_celsius: Option<f32>,
}

/// Power metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMetrics {
    /// Battery present
    pub battery_present: bool,
    /// Battery percentage (0-100)
    pub battery_percent: Option<f32>,
    /// Battery status: "Charging", "Discharging", "Full"
    pub battery_status: Option<String>,
    /// Time remaining (seconds)
    pub time_remaining_seconds: Option<u64>,
    /// Power consumption in watts
    pub power_consumption_watts: Option<f32>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                usage_percent: 0.0,
                per_core_usage: Vec::new(),
                frequency_mhz: 0,
                core_count: 0,
                load_average: (0.0, 0.0, 0.0),
            },
            memory: MemoryMetrics {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                usage_percent: 0.0,
                swap_total_bytes: 0,
                swap_used_bytes: 0,
                cached_bytes: 0,
                buffers_bytes: 0,
            },
            processes: Vec::new(),
            network: NetworkMetrics {
                interfaces: Vec::new(),
                total_rx_bytes: 0,
                total_tx_bytes: 0,
                rx_speed_bps: 0,
                tx_speed_bps: 0,
            },
            disks: Vec::new(),
            gpu: None,
            temperatures: Vec::new(),
            power: None,
        }
    }
}

