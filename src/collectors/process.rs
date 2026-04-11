//! Process metrics collector
//!
//! Collects information about running processes.

use anyhow::Result;
use sysinfo::System;
use crate::models::metrics::ProcessMetrics;
use super::Collector;

/// Process metrics collector
pub struct ProcessCollector {
    system: System,
    metrics: Vec<ProcessMetrics>,
}

impl ProcessCollector {
    /// Create a new process collector
    pub fn new() -> Result<Self> {
        let system = System::new_all();

        Ok(Self {
            system,
            metrics: Vec::new(),
        })
    }

    /// Get the current metrics
    pub fn get_metrics(&self) -> Vec<ProcessMetrics> {
        self.metrics.clone()
    }
}

impl Collector for ProcessCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh process information
        self.system.refresh_processes();

        // Get total memory for percentage calculation
        let total_memory = self.system.total_memory();

        // Collect metrics for all processes
        let mut processes: Vec<ProcessMetrics> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let memory_bytes = process.memory();
                let memory_percent = if total_memory > 0 {
                    (memory_bytes as f32 / total_memory as f32) * 100.0
                } else {
                    0.0
                };

                ProcessMetrics {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    cpu_percent: process.cpu_usage(),
                    memory_bytes,
                    memory_percent,
                    status: format!("{:?}", process.status()),
                    parent_pid: process.parent().map(|p| p.as_u32()),
                    thread_count: 0, // sysinfo doesn't provide thread count easily
                    start_time: chrono::Utc::now(), // sysinfo doesn't provide start time easily
                }
            })
            .collect();

        // Sort by CPU usage (highest first)
        processes.sort_by(|a, b| {
            b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal)
        });

        self.metrics = processes;

        Ok(())
    }
}
