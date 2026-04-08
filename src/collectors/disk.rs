//! Disk metrics collector
//!
//! Collects disk usage and I/O statistics.

use anyhow::Result;
use sysinfo::Disks;
use crate::models::metrics::DiskMetrics;
use super::Collector;

/// Disk metrics collector
pub struct DiskCollector {
    disks: Disks,
    metrics: Vec<DiskMetrics>,
}

impl DiskCollector {
    /// Create a new disk collector
    pub fn new() -> Result<Self> {
        let disks = Disks::new_with_refreshed_list();
        
        Ok(Self {
            disks,
            metrics: Vec::new(),
        })
    }
    
    /// Get the current metrics
    pub fn get_metrics(&self) -> Vec<DiskMetrics> {
        self.metrics.clone()
    }
}

impl Collector for DiskCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh disk information
        self.disks.refresh();
        
        // Collect disk metrics
        self.metrics = self.disks
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                let usage_percent = if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                };
                
                DiskMetrics {
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    device: disk.name().to_string_lossy().to_string(),
                    fs_type: disk.file_system().to_string_lossy().to_string(),
                    total_bytes: total,
                    used_bytes: used,
                    available_bytes: available,
                    usage_percent,
                    // I/O speed tracking not implemented (requires platform-specific APIs)
                    read_speed_bps: 0,
                    write_speed_bps: 0,
                }
            })
            .collect();
        
        Ok(())
    }
}

