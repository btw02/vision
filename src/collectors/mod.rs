//! System data collectors
//!
//! This module contains all the data collection implementations for various system metrics.

pub mod cpu;
pub mod memory;
pub mod process;
pub mod network;
pub mod disk;
pub mod gpu;
pub mod temperature;
pub mod power;

use anyhow::Result;
use crate::models::metrics::SystemMetrics;

/// Trait for all metric collectors
pub trait Collector {
    /// Collect current metrics
    fn collect(&mut self) -> Result<()>;
}

/// Main collector that aggregates all individual collectors
pub struct MetricsCollector {
    pub cpu: cpu::CpuCollector,
    pub memory: memory::MemoryCollector,
    pub process: process::ProcessCollector,
    pub network: network::NetworkCollector,
    pub disk: disk::DiskCollector,
    pub gpu: Option<gpu::GpuCollector>,
    pub temperature: temperature::TemperatureCollector,
    pub power: Option<power::PowerCollector>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Result<Self> {
        Ok(Self {
            cpu: cpu::CpuCollector::new()?,
            memory: memory::MemoryCollector::new()?,
            process: process::ProcessCollector::new()?,
            network: network::NetworkCollector::new()?,
            disk: disk::DiskCollector::new()?,
            gpu: gpu::GpuCollector::new().ok(),
            temperature: temperature::TemperatureCollector::new()?,
            power: power::PowerCollector::new().ok(),
        })
    }
    
    /// Collect all metrics and return a complete snapshot
    pub fn collect_all(&mut self) -> Result<SystemMetrics> {
        // Collect metrics sequentially
        self.cpu.collect()?;
        self.memory.collect()?;
        self.process.collect()?;
        self.network.collect()?;
        self.disk.collect()?;
        
        // Optional collectors (may not be available on all systems)
        if let Some(ref mut gpu) = self.gpu {
            let _ = gpu.collect();
        }
        
        self.temperature.collect()?;
        
        if let Some(ref mut power) = self.power {
            let _ = power.collect();
        }
        
        // Build the complete metrics snapshot
        Ok(SystemMetrics {
            timestamp: chrono::Utc::now(),
            cpu: self.cpu.get_metrics(),
            memory: self.memory.get_metrics(),
            processes: self.process.get_metrics(),
            network: self.network.get_metrics(),
            disks: self.disk.get_metrics(),
            gpu: self.gpu.as_ref().and_then(|g| g.get_metrics()),
            temperatures: self.temperature.get_metrics(),
            power: self.power.as_ref().and_then(|p| p.get_metrics()),
        })
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create metrics collector")
    }
}

