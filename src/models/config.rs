//! Configuration data structures
//!
//! Defines the application configuration model based on the technical specification.

use serde::{Deserialize, Serialize};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub monitoring: MonitoringConfig,
    pub alerts: Vec<AlertConfig>,
    pub ui: UiConfig,
    pub export: ExportConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            monitoring: MonitoringConfig::default(),
            alerts: Vec::new(),
            ui: UiConfig::default(),
            export: ExportConfig::default(),
        }
    }
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Refresh interval in milliseconds (default: 1000)
    pub refresh_interval_ms: u64,
    /// Start application minimized
    pub start_minimized: bool,
    /// Minimize to system tray instead of taskbar
    pub minimize_to_tray: bool,
    /// Start application on system boot
    pub start_on_boot: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            refresh_interval_ms: 1000,
            start_minimized: false,
            minimize_to_tray: true,
            start_on_boot: false,
        }
    }
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_cpu: bool,
    pub enable_memory: bool,
    pub enable_disk: bool,
    pub enable_network: bool,
    pub enable_gpu: bool,
    pub enable_temperature: bool,
    pub enable_power: bool,
    /// Duration to keep historical data (hours)
    pub history_duration_hours: u32,
    /// Maximum storage size for metrics database (MB)
    pub max_storage_mb: u32,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_cpu: true,
            enable_memory: true,
            enable_disk: true,
            enable_network: true,
            enable_gpu: true,
            enable_temperature: true,
            enable_power: true,
            history_duration_hours: 24,
            max_storage_mb: 100,
        }
    }
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub id: String,
    pub enabled: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub condition: AlertCondition,
    pub threshold: f64,
    /// Alert must be sustained for this duration (seconds)
    pub duration_seconds: u64,
    pub actions: Vec<AlertAction>,
}

/// Alert condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    CpuUsageAbove,
    MemoryUsageAbove,
    DiskUsageAbove,
    TemperatureAbove,
    NetworkSpeedAbove,
    ProcessCpuAbove { process_name: String },
}

/// Alert action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertAction {
    Notification { title: String, message: String },
    Command { command: String },
    Log { level: String },
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme: "Dark" or "Light"
    pub theme: String,
    /// Font size
    pub font_size: f32,
    /// Show graphs in dashboard
    pub show_graphs: bool,
    /// Graph history duration (seconds)
    pub graph_history_seconds: u32,
    /// Window width
    pub window_width: f32,
    /// Window height
    pub window_height: f32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "Dark".to_string(),
            font_size: 14.0,
            show_graphs: true,
            graph_history_seconds: 60,
            window_width: 1200.0,
            window_height: 800.0,
        }
    }
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Default export format: "CSV", "JSON"
    pub default_format: String,
    /// Default export directory
    pub default_directory: String,
    /// Include timestamps in export
    pub include_timestamps: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            default_format: "CSV".to_string(),
            default_directory: "~/Documents/SystemVision".to_string(),
            include_timestamps: true,
        }
    }
}

