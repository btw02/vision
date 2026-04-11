//! Application state management
//!
//! Manages the runtime state of the application.

use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use super::metrics::SystemMetrics;

/// Maximum number of historical data points to keep in memory
const MAX_HISTORY_POINTS: usize = 3600; // 1 hour at 1 second intervals

/// Application runtime state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current system metrics
    pub current_metrics: SystemMetrics,

    /// Historical metrics (ring buffer)
    pub metrics_history: VecDeque<SystemMetrics>,

    /// Active alerts
    pub active_alerts: Vec<ActiveAlert>,

    /// Application start time
    pub start_time: DateTime<Utc>,

    /// Last update time
    pub last_update: DateTime<Utc>,

    /// Is data collection paused
    pub paused: bool,

    /// Selected process filter
    pub process_filter: String,

    /// Sort column for process view
    pub process_sort_column: ProcessSortColumn,

    /// Sort direction
    pub process_sort_ascending: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            current_metrics: SystemMetrics::default(),
            metrics_history: VecDeque::with_capacity(MAX_HISTORY_POINTS),
            active_alerts: Vec::new(),
            start_time: now,
            last_update: now,
            paused: false,
            process_filter: String::new(),
            process_sort_column: ProcessSortColumn::Cpu,
            process_sort_ascending: false,
        }
    }
}

impl AppState {
    /// Add a new metrics snapshot to history
    pub fn add_metrics(&mut self, metrics: SystemMetrics) {
        self.current_metrics = metrics.clone();
        self.last_update = Utc::now();

        // Add to history, removing oldest if at capacity
        if self.metrics_history.len() >= MAX_HISTORY_POINTS {
            self.metrics_history.pop_front();
        }
        self.metrics_history.push_back(metrics);
    }

    /// Get metrics history for a specific duration (seconds)
    pub fn get_history(&self, duration_seconds: u64) -> Vec<&SystemMetrics> {
        let cutoff = Utc::now() - chrono::Duration::seconds(duration_seconds as i64);
        self.metrics_history
            .iter()
            .filter(|m| m.timestamp > cutoff)
            .collect()
    }

    /// Clear all historical data
    pub fn clear_history(&mut self) {
        self.metrics_history.clear();
    }

    /// Add an active alert
    pub fn add_alert(&mut self, alert: ActiveAlert) {
        self.active_alerts.push(alert);
    }

    /// Remove an alert by ID
    pub fn remove_alert(&mut self, id: &str) {
        self.active_alerts.retain(|a| a.id != id);
    }

    /// Clear all alerts
    pub fn clear_alerts(&mut self) {
        self.active_alerts.clear();
    }
}

/// Active alert information
#[derive(Debug, Clone)]
pub struct ActiveAlert {
    /// Alert ID
    pub id: String,

    /// Alert name
    pub name: String,

    /// Alert message
    pub message: String,

    /// Alert severity
    pub severity: AlertSeverity,

    /// When the alert was triggered
    pub triggered_at: DateTime<Utc>,

    /// Current value that triggered the alert
    pub current_value: f64,

    /// Threshold value
    pub threshold: f64,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Process table sort columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSortColumn {
    Pid,
    Name,
    Cpu,
    Memory,
    Status,
}
