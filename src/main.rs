//! SystemVision - Advanced Linux System Monitoring Tool
//!
//! Main entry point for the application.

use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod app;
mod collectors;
mod models;
mod ui;
mod storage;
mod alerts;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting SystemVision v{}", env!("CARGO_PKG_VERSION"));

    // TODO: Load configuration
    // TODO: Initialize database
    // TODO: Start data collectors
    // TODO: Launch GUI

    // Run the application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "SystemVision",
        native_options,
        Box::new(|cc| {
            // TODO: Initialize app with proper context
            Box::new(app::SystemVisionApp::new(cc))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))?;

    info!("SystemVision shutting down");
    Ok(())
}

/// Load application icon
fn load_icon() -> egui::IconData {
    // TODO: Load actual icon from assets
    // For now, return a default empty icon
    egui::IconData {
        rgba: vec![0; 32 * 32 * 4],
        width: 32,
        height: 32,
    }
}
