//! Alert manager
//!
//! Manages alert conditions and triggers actions.

use anyhow::Result;
use crate::models::{
    config::{AlertConfig, AlertCondition, AlertAction},
    metrics::SystemMetrics,
    state::{AppState, ActiveAlert, AlertSeverity},
};

/// Alert manager
pub struct AlertManager {
    configs: Vec<AlertConfig>,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new(configs: Vec<AlertConfig>) -> Self {
        Self { configs }
    }

    /// Check metrics against alert conditions
    pub fn check_alerts(&self, metrics: &SystemMetrics, state: &mut AppState) -> Result<()> {
        for config in &self.configs {
            if !config.enabled {
                continue;
            }

            // Evaluate condition and get current value
            if let Some(current_value) = self.evaluate_condition(&config.condition, metrics, config.threshold) {
                self.trigger_alert(config, current_value, state)?;
            }
        }

        Ok(())
    }

    /// Evaluate an alert condition
    ///
    /// Returns Some(current_value) if the condition is met, None otherwise
    fn evaluate_condition(
        &self,
        condition: &AlertCondition,
        metrics: &SystemMetrics,
        threshold: f64,
    ) -> Option<f64> {
        let current_value = match condition {
            AlertCondition::CpuUsageAbove => metrics.cpu.usage_percent as f64,
            AlertCondition::MemoryUsageAbove => metrics.memory.usage_percent as f64,
            AlertCondition::DiskUsageAbove => {
                metrics.disks.iter()
                    .map(|d| d.usage_percent as f64)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0)
            }
            AlertCondition::TemperatureAbove => {
                metrics.temperatures.iter()
                    .map(|t| t.temperature_celsius as f64)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0)
            }
            AlertCondition::NetworkSpeedAbove => {
                (metrics.network.rx_speed_bps + metrics.network.tx_speed_bps) as f64
            }
            AlertCondition::ProcessCpuAbove { process_name } => {
                metrics.processes.iter()
                    .filter(|p| p.name.contains(process_name))
                    .map(|p| p.cpu_percent as f64)
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0)
            }
        };

        if current_value > threshold {
            Some(current_value)
        } else {
            None
        }
    }

    /// Trigger alert actions
    fn trigger_alert(
        &self,
        config: &AlertConfig,
        current_value: f64,
        state: &mut AppState,
    ) -> Result<()> {
        // Determine severity based on how far over threshold
        let severity = if current_value > config.threshold * 1.5 {
            AlertSeverity::Critical
        } else if current_value > config.threshold * 1.2 {
            AlertSeverity::Warning
        } else {
            AlertSeverity::Info
        };

        // Create active alert
        let alert = ActiveAlert {
            id: config.id.clone(),
            name: config.name.clone(),
            message: config.message.clone().unwrap_or_else(|| {
                format!("{} exceeded threshold", config.name)
            }),
            severity,
            triggered_at: chrono::Utc::now(),
            current_value,
            threshold: config.threshold,
        };

        // Add to active alerts if not already present
        if !state.active_alerts.iter().any(|a| a.id == alert.id) {
            state.add_alert(alert);

            // Execute alert actions
            for action in &config.actions {
                if let Err(e) = self.execute_action(action) {
                    tracing::warn!("Failed to execute alert action: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Execute an alert action
    fn execute_action(&self, action: &AlertAction) -> Result<()> {
        match action {
            AlertAction::Notification { title, message } => {
                // Log notification (desktop notifications not implemented)
                tracing::info!("Alert notification: {} - {}", title, message);
            }
            AlertAction::Command { command } => {
                // Log command (command execution not implemented for security)
                tracing::info!("Alert would execute command: {}", command);
            }
            AlertAction::Log { level } => {
                tracing::info!("Alert logged at level: {}", level);
            }
        }

        Ok(())
    }
}
