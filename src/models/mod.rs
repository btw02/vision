//! Data models for SystemVision
//!
//! This module contains all data structures used throughout the application.

pub mod config;
pub mod metrics;
pub mod state;

// Re-export commonly used types
pub use config::AppConfig;
pub use metrics::SystemMetrics;
pub use state::AppState;
