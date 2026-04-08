//! User interface modules
//!
//! Contains all UI components and views for the application.

pub mod dashboard;
pub mod process_view;
pub mod network_view;
pub mod disk_view;
pub mod gpu_view;
pub mod settings;
pub mod alerts;
pub mod theme;

// Re-export commonly used types
pub use theme::Theme;

