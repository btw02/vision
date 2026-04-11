//! SystemVision Library
//!
//! Core library exposing the main modules and functionality of SystemVision.
//! This allows the application to be used as a library if needed.

pub mod app;
pub mod collectors;
pub mod models;
pub mod ui;
pub mod storage;
pub mod alerts;
pub mod utils;

// Re-export commonly used types
pub use app::SystemVisionApp;
pub use models::{
    config::AppConfig,
    metrics::SystemMetrics,
    state::AppState,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");
