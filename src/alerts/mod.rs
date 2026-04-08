//! Alert system
//!
//! Monitors metrics and triggers alerts based on configured conditions.

pub mod manager;

// Re-export commonly used types
pub use manager::AlertManager;

