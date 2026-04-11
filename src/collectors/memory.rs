//! Memory metrics collector
//!
//! Collects RAM and swap usage information.

use anyhow::Result;
use sysinfo::System;
use crate::models::metrics::MemoryMetrics;
use super::Collector;

/// Memory metrics collector
pub struct MemoryCollector {
    system: System,
    metrics: MemoryMetrics,
}

impl MemoryCollector {
    /// Create a new memory collector
    pub fn new() -> Result<Self> {
        let system = System::new_all();

        Ok(Self {
            system,
            metrics: MemoryMetrics {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                usage_percent: 0.0,
                swap_total_bytes: 0,
                swap_used_bytes: 0,
                cached_bytes: 0,
                buffers_bytes: 0,
            },
        })
    }

    /// Get the current metrics
    pub fn get_metrics(&self) -> MemoryMetrics {
        self.metrics.clone()
    }
}

impl Collector for MemoryCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh memory information
        self.system.refresh_memory();

        // Get memory metrics
        self.metrics.total_bytes = self.system.total_memory();
        self.metrics.used_bytes = self.system.used_memory();
        self.metrics.available_bytes = self.system.available_memory();

        // Calculate usage percentage
        if self.metrics.total_bytes > 0 {
            self.metrics.usage_percent =
                (self.metrics.used_bytes as f32 / self.metrics.total_bytes as f32) * 100.0;
        }

        // Get swap metrics
        self.metrics.swap_total_bytes = self.system.total_swap();
        self.metrics.swap_used_bytes = self.system.used_swap();

        // Note: cached and buffers are not separately tracked by sysinfo
        // They are included in the used_memory calculation

        Ok(())
    }
}
