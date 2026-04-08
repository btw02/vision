//! Network metrics collector
//!
//! Collects network interface statistics and traffic information.

use anyhow::Result;
use sysinfo::Networks;
use crate::models::metrics::{NetworkMetrics, NetworkInterface};
use super::Collector;

/// Network metrics collector
pub struct NetworkCollector {
    networks: Networks,
    metrics: NetworkMetrics,
    last_rx_bytes: u64,
    last_tx_bytes: u64,
    last_update: std::time::Instant,
}

impl NetworkCollector {
    /// Create a new network collector
    pub fn new() -> Result<Self> {
        let networks = Networks::new_with_refreshed_list();
        
        Ok(Self {
            networks,
            metrics: NetworkMetrics {
                interfaces: Vec::new(),
                total_rx_bytes: 0,
                total_tx_bytes: 0,
                rx_speed_bps: 0,
                tx_speed_bps: 0,
            },
            last_rx_bytes: 0,
            last_tx_bytes: 0,
            last_update: std::time::Instant::now(),
        })
    }
    
    /// Get the current metrics
    pub fn get_metrics(&self) -> NetworkMetrics {
        self.metrics.clone()
    }
}

impl Collector for NetworkCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh network information
        self.networks.refresh();
        
        // Collect interface metrics
        self.metrics.interfaces = self.networks
            .iter()
            .map(|(name, data)| {
                NetworkInterface {
                    name: name.clone(),
                    rx_bytes: data.total_received(),
                    tx_bytes: data.total_transmitted(),
                    rx_packets: data.total_packets_received(),
                    tx_packets: data.total_packets_transmitted(),
                    rx_errors: data.total_errors_on_received(),
                    tx_errors: data.total_errors_on_transmitted(),
                    is_up: true, // Assume up if we can read it
                }
            })
            .collect();
        
        // Calculate totals
        let total_rx: u64 = self.metrics.interfaces.iter().map(|i| i.rx_bytes).sum();
        let total_tx: u64 = self.metrics.interfaces.iter().map(|i| i.tx_bytes).sum();
        
        // Calculate speeds (bytes per second)
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        
        if elapsed > 0.0 && self.last_rx_bytes > 0 {
            let rx_diff = total_rx.saturating_sub(self.last_rx_bytes);
            let tx_diff = total_tx.saturating_sub(self.last_tx_bytes);
            
            self.metrics.rx_speed_bps = (rx_diff as f64 / elapsed) as u64;
            self.metrics.tx_speed_bps = (tx_diff as f64 / elapsed) as u64;
        }
        
        self.metrics.total_rx_bytes = total_rx;
        self.metrics.total_tx_bytes = total_tx;
        self.last_rx_bytes = total_rx;
        self.last_tx_bytes = total_tx;
        self.last_update = now;
        
        Ok(())
    }
}

