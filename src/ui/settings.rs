//! Settings view
//!
//! Application configuration and preferences.

use egui::{Ui, Context, RichText};
use crate::models::config::AppConfig;
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the settings view
pub fn render(_ctx: &Context, ui: &mut Ui, config: &mut AppConfig) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("Settings")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        // General Settings with improved design
        ui.group(|ui| {
            ui.label(RichText::new("⚙️ General")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Refresh Interval:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::Slider::new(&mut config.general.refresh_interval_ms, 100..=10000)
                    .suffix(" ms")
                    .text("Update Rate"));
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.checkbox(&mut config.general.start_minimized,
                RichText::new("Start minimized").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.general.minimize_to_tray,
                RichText::new("Minimize to system tray").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.general.start_on_boot,
                RichText::new("Start on system boot").size(Typography::BODY_SIZE));
        });
        
        ui.add_space(Spacing::MEDIUM);
        
        // UI Settings with improved layout
        ui.group(|ui| {
            ui.label(RichText::new("🎨 Display")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Theme:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                egui::ComboBox::from_id_source("theme_selector")
                    .selected_text(&config.ui.theme)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.ui.theme, "Dark".to_string(), "Dark");
                        ui.selectable_value(&mut config.ui.theme, "Light".to_string(), "Light");
                    });
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Font Size:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::Slider::new(&mut config.ui.font_size, 10.0..=20.0)
                    .suffix(" pt")
                    .text("Size"));
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.checkbox(&mut config.ui.show_graphs,
                RichText::new("Show graphs on dashboard").size(Typography::BODY_SIZE));
            
            ui.add_space(Spacing::TINY);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Graph History:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::Slider::new(&mut config.ui.graph_history_seconds, 10..=300)
                    .suffix(" s")
                    .text("Duration"));
            });
        });
        
        ui.add_space(Spacing::MEDIUM);
        
        // Monitoring Settings with better organization
        ui.group(|ui| {
            ui.label(RichText::new("📊 Monitoring")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            ui.label(RichText::new("Enable Monitoring For:")
                .size(Typography::BODY_SIZE)
                .strong());
            ui.add_space(Spacing::TINY);
            
            ui.checkbox(&mut config.monitoring.enable_cpu,
                RichText::new("CPU").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_memory,
                RichText::new("Memory").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_disk,
                RichText::new("Disks").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_network,
                RichText::new("Network").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_gpu,
                RichText::new("GPU (if available)").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_temperature,
                RichText::new("Temperatures").size(Typography::BODY_SIZE));
            ui.checkbox(&mut config.monitoring.enable_power,
                RichText::new("Power/Battery").size(Typography::BODY_SIZE));
            
            ui.add_space(Spacing::SMALL);
            ui.separator();
            ui.add_space(Spacing::SMALL);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("History Duration:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::Slider::new(&mut config.monitoring.history_duration_hours, 1..=72)
                    .suffix(" hours")
                    .text("Keep Data"));
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Max Storage:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::Slider::new(&mut config.monitoring.max_storage_mb, 10..=1000)
                    .suffix(" MB")
                    .text("Database Size"));
            });
        });
        
        ui.add_space(Spacing::MEDIUM);
        
        // Export Settings with improved design
        ui.group(|ui| {
            ui.label(RichText::new("💾 Data Export")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Default Format:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                egui::ComboBox::from_id_source("export_format")
                    .selected_text(&config.export.default_format)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.export.default_format, "CSV".to_string(), "CSV");
                        ui.selectable_value(&mut config.export.default_format, "JSON".to_string(), "JSON");
                    });
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Export Directory:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.text_edit_singleline(&mut config.export.default_directory);
            });
            
            ui.add_space(Spacing::TINY);
            
            ui.checkbox(&mut config.export.include_timestamps,
                RichText::new("Include timestamps in exports").size(Typography::BODY_SIZE));
        });
        
        ui.add_space(Spacing::MEDIUM);
        
        // Alert Configuration Info with better styling
        ui.group(|ui| {
            ui.label(RichText::new("🔔 Alerts")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            ui.label(RichText::new(format!("Configured Alerts: {}", config.alerts.len()))
                .size(Typography::BODY_SIZE)
                .strong());
            ui.add_space(Spacing::TINY);
            ui.label(RichText::new("Alert configuration is managed through the configuration file.")
                .size(Typography::SMALL_SIZE)
                .color(ColorPalette::TEXT_SECONDARY));
            ui.label(RichText::new("See documentation for alert setup instructions.")
                .size(Typography::SMALL_SIZE)
                .color(ColorPalette::TEXT_SECONDARY));
        });
        
        ui.add_space(Spacing::LARGE);
        
        // Action buttons with improved styling
        ui.separator();
        ui.add_space(Spacing::SMALL);
        
        ui.horizontal(|ui| {
            if ui.button(RichText::new("↺ Reset to Defaults")
                .size(Typography::BODY_SIZE)).clicked() {
                *config = AppConfig::default();
            }
        });
        
        ui.add_space(Spacing::SMALL);
        ui.label(RichText::new("⚠ Note: Settings are not persisted to disk yet.")
            .color(ColorPalette::WARNING)
            .size(Typography::SMALL_SIZE));
        
        ui.add_space(Spacing::MEDIUM);
        
        // Info section with better design
        ui.separator();
        ui.add_space(Spacing::SMALL);
        ui.label(RichText::new("ℹ️ About SystemVision")
            .size(Typography::SUBHEADING_SIZE)
            .strong());
        ui.add_space(Spacing::TINY);
        ui.label(RichText::new(format!("Version: {}", env!("CARGO_PKG_VERSION")))
            .size(Typography::BODY_SIZE));
        ui.label(RichText::new("A comprehensive system monitoring tool for Linux")
            .size(Typography::SMALL_SIZE)
            .color(ColorPalette::TEXT_SECONDARY));
        ui.add_space(Spacing::TINY);
        ui.hyperlink_to("🔗 GitHub Repository", "https://github.com/yourusername/system_vision");
    });
}

