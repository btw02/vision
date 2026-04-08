//! CPU metrics collector
//!
//! Collects CPU usage, frequency, and load average information.

use anyhow::Result;
use sysinfo::System;
use crate::models::metrics::CpuMetrics;
use super::Collector;

/// CPU metrics collector
pub struct CpuCollector {
    system: System,
    metrics: CpuMetrics,
}

impl CpuCollector {
    /// Create a new CPU collector
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_cpu();
        
        Ok(Self {
            system,
            metrics: CpuMetrics {
                usage_percent: 0.0,
                per_core_usage: Vec::new(),
                frequency_mhz: 0,
                core_count: num_cpus::get(),
                load_average: (0.0, 0.0, 0.0),
            },
        })
    }
    
    /// Get the current metrics
    pub fn get_metrics(&self) -> CpuMetrics {
        self.metrics.clone()
    }
}

impl Collector for CpuCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh CPU information
        self.system.refresh_cpu();
        
        // Get global CPU usage
        self.metrics.usage_percent = self.system.global_cpu_info().cpu_usage();
        
        // Get per-core usage
        self.metrics.per_core_usage = self.system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();
        
        // Get CPU frequency (from first CPU)
        if let Some(cpu) = self.system.cpus().first() {
            self.metrics.frequency_mhz = cpu.frequency();
        }
        
        // Get load average
        let load_avg = System::load_average();
        self.metrics.load_average = (load_avg.one, load_avg.five, load_avg.fifteen);
        
        Ok(())
    }
}

