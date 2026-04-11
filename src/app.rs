//! Main application structure and state management
//!
//! This module contains the core application logic and state.

use egui::{Context, CentralPanel};
use crate::models::{config::AppConfig, state::AppState};
use crate::collectors::MetricsCollector;
use crate::alerts::manager::AlertManager;
use std::time::{Duration, Instant};

/// Main application structure
pub struct SystemVisionApp {
    /// Application configuration
    config: AppConfig,

    /// Application state
    state: AppState,

    /// Current selected tab
    selected_tab: Tab,

    /// Metrics collector
    collector: MetricsCollector,

    /// Alert manager
    alert_manager: AlertManager,

    /// Last update time
    last_update: Instant,

    /// Update interval in milliseconds
    update_interval_ms: u64,
}

/// Available tabs in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Dashboard,
    Processes,
    Network,
    Disk,
    Gpu,
    Alerts,
    Settings,
}

impl SystemVisionApp {
    /// Create a new SystemVision application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui style
        configure_style(&cc.egui_ctx);

        // Initialize metrics collector
        let collector = MetricsCollector::new()
            .expect("Failed to initialize metrics collector");

        let config = AppConfig::default();
        let update_interval_ms = config.general.refresh_interval_ms;

        // Initialize alert manager with configured alerts
        let alert_manager = AlertManager::new(config.alerts.clone());

        Self {
            config,
            state: AppState::default(),
            selected_tab: Tab::Dashboard,
            collector,
            alert_manager,
            last_update: Instant::now(),
            update_interval_ms,
        }
    }
}

impl eframe::App for SystemVisionApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Update metrics if not paused and enough time has elapsed
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        if !self.state.paused && elapsed >= Duration::from_millis(self.update_interval_ms) {
            // Collect metrics
            if let Ok(metrics) = self.collector.collect_all() {
                // Check alerts
                if let Err(e) = self.alert_manager.check_alerts(&metrics, &mut self.state) {
                    tracing::warn!("Alert check failed: {}", e);
                }

                self.state.add_metrics(metrics);
            }
            self.last_update = now;
        }

        // Top panel with tabs and controls
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SystemVision");
                ui.separator();

                // Tab selection
                ui.selectable_value(&mut self.selected_tab, Tab::Dashboard, "📊 Dashboard");
                ui.selectable_value(&mut self.selected_tab, Tab::Processes, "⚙️ Processes");
                ui.selectable_value(&mut self.selected_tab, Tab::Network, "🌐 Network");
                ui.selectable_value(&mut self.selected_tab, Tab::Disk, "💾 Disk");
                ui.selectable_value(&mut self.selected_tab, Tab::Gpu, "🎮 GPU");
                ui.selectable_value(&mut self.selected_tab, Tab::Alerts, "🔔 Alerts");
                ui.separator();
                ui.selectable_value(&mut self.selected_tab, Tab::Settings, "⚙️ Settings");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Pause/Resume button
                    let pause_text = if self.state.paused { "▶ Resume" } else { "⏸ Pause" };
                    if ui.button(pause_text).clicked() {
                        self.state.paused = !self.state.paused;
                    }

                    ui.separator();

                    // Refresh interval slider
                    ui.label("Refresh:");
                    ui.add(egui::Slider::new(&mut self.update_interval_ms, 100..=5000)
                        .suffix(" ms")
                        .text("Interval"));
                });
            });
        });

        // Main content area
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.selected_tab {
                    Tab::Dashboard => {
                        crate::ui::dashboard::render(ctx, ui, &self.state.current_metrics, &self.state);
                    }
                    Tab::Processes => {
                        let metrics = self.state.current_metrics.clone();
                        crate::ui::process_view::render(ctx, ui, &metrics, &mut self.state);
                    }
                    Tab::Network => {
                        crate::ui::network_view::render(ctx, ui, &self.state.current_metrics, &self.state);
                    }
                    Tab::Disk => {
                        crate::ui::disk_view::render(ctx, ui, &self.state.current_metrics, &self.state);
                    }
                    Tab::Gpu => {
                        crate::ui::gpu_view::render(ctx, ui, &self.state.current_metrics, &self.state);
                    }
                    Tab::Alerts => {
                        crate::ui::alerts::render(ctx, ui, &mut self.state);
                    }
                    Tab::Settings => {
                        crate::ui::settings::render(ctx, ui, &mut self.config);
                    }
                }
            });
        });

        // Request repaint based on refresh interval
        ctx.request_repaint_after(Duration::from_millis(self.update_interval_ms));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Cleanup on exit
        tracing::info!("SystemVision shutting down");
    }
}

/// Configure the egui style
fn configure_style(ctx: &Context) {
    // Use default dark theme
    ctx.set_visuals(egui::Visuals::dark());
}
