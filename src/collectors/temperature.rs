//! Temperature metrics collector
//!
//! Collects system temperature information from various sensors.

use anyhow::Result;
use sysinfo::Components;
use crate::models::metrics::TemperatureMetrics;
use super::Collector;

/// Temperature metrics collector
pub struct TemperatureCollector {
    components: Components,
    metrics: Vec<TemperatureMetrics>,
}

impl TemperatureCollector {
    /// Create a new temperature collector
    pub fn new() -> Result<Self> {
        let components = Components::new_with_refreshed_list();

        Ok(Self {
            components,
            metrics: Vec::new(),
        })
    }

    /// Get the current metrics
    pub fn get_metrics(&self) -> Vec<TemperatureMetrics> {
        self.metrics.clone()
    }
}

impl Collector for TemperatureCollector {
    fn collect(&mut self) -> Result<()> {
        // Refresh component information
        self.components.refresh();

        // Collect temperature metrics
        self.metrics = self.components
            .iter()
            .map(|component| {
                TemperatureMetrics {
                    label: component.label().to_string(),
                    temperature_celsius: component.temperature(),
                    critical_celsius: component.critical(),
                    max_celsius: Some(component.max()),
                }
            })
            .collect();

        Ok(())
    }
}
