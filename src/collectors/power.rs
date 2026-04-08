//! Power metrics collector
//!
//! Collects battery and power consumption information.

use anyhow::Result;
use crate::models::metrics::PowerMetrics;
use super::Collector;
use std::fs;
use std::path::Path;

/// Power metrics collector
pub struct PowerCollector {
    metrics: Option<PowerMetrics>,
}

impl PowerCollector {
    /// Create a new power collector
    pub fn new() -> Result<Self> {
        Ok(Self {
            metrics: None,
        })
    }
    
    /// Get the current metrics
    pub fn get_metrics(&self) -> Option<PowerMetrics> {
        self.metrics.clone()
    }
    
    /// Read battery information from sysfs (Linux)
    fn read_battery_info() -> Option<PowerMetrics> {
        let power_supply_path = Path::new("/sys/class/power_supply");
        
        if !power_supply_path.exists() {
            return None;
        }
        
        // Find battery directory (usually BAT0, BAT1, etc.)
        let battery_dir = fs::read_dir(power_supply_path).ok()?
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                entry.file_name().to_string_lossy().starts_with("BAT")
            })?;
        
        let battery_path = battery_dir.path();
        
        // Read battery capacity (percentage)
        let capacity = fs::read_to_string(battery_path.join("capacity"))
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok());
        
        // Read battery status
        let status = fs::read_to_string(battery_path.join("status"))
            .ok()
            .map(|s| s.trim().to_string());
        
        // Read power consumption (in microwatts, convert to watts)
        let power_now = fs::read_to_string(battery_path.join("power_now"))
            .ok()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|p| p / 1_000_000.0);
        
        // Calculate time remaining (rough estimate)
        let time_remaining = if let (Some(_cap), Some(power), Some(ref stat)) =
            (capacity, power_now, &status) {
            if power > 0.0 {
                // Read energy_full and energy_now to calculate time
                let energy_full = fs::read_to_string(battery_path.join("energy_full"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok())
                    .map(|e| e / 1_000_000.0); // Convert to Wh
                
                let energy_now = fs::read_to_string(battery_path.join("energy_now"))
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok())
                    .map(|e| e / 1_000_000.0); // Convert to Wh
                
                if let (Some(full), Some(now)) = (energy_full, energy_now) {
                    let hours = if stat == "Discharging" {
                        now / power
                    } else if stat == "Charging" {
                        (full - now) / power
                    } else {
                        0.0
                    };
                    Some((hours * 3600.0) as u64) // Convert to seconds
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        Some(PowerMetrics {
            battery_present: true,
            battery_percent: capacity,
            battery_status: status,
            time_remaining_seconds: time_remaining,
            power_consumption_watts: power_now,
        })
    }
}

impl Collector for PowerCollector {
    fn collect(&mut self) -> Result<()> {
        // Try to read battery information
        self.metrics = Self::read_battery_info();
        
        // If no battery found, set default values
        if self.metrics.is_none() {
            self.metrics = Some(PowerMetrics {
                battery_present: false,
                battery_percent: None,
                battery_status: None,
                time_remaining_seconds: None,
                power_consumption_watts: None,
            });
        }
        
        Ok(())
    }
}

