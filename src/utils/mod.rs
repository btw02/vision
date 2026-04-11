//! Utility functions
//!
//! Common utility functions used throughout the application.

pub mod system;
pub mod formatting;

// Re-export commonly used functions
pub use formatting::{format_bytes, format_duration, format_percentage};
pub use system::{get_system_info, get_uptime};
