//! GPU view
//!
//! GPU metrics and information display.

use egui::{Ui, Context, RichText};
use crate::models::{metrics::SystemMetrics, state::AppState};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the GPU view
pub fn render(_ctx: &Context, ui: &mut Ui, metrics: &SystemMetrics, _state: &AppState) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("GPU Monitor")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    match &metrics.gpu {
        Some(gpu) => {
            // GPU information card with improved design
            ui.group(|ui| {
                ui.label(RichText::new(format!("🎮 {}", &gpu.name))
                    .size(Typography::SUBHEADING_SIZE)
                    .strong());
                ui.add_space(Spacing::MEDIUM);
                
                // GPU utilization with better styling
                ui.horizontal(|ui| {
                    ui.label(RichText::new("GPU Usage:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::SMALL);
                    let color = ColorPalette::usage_color(gpu.usage_percent);
                    ui.add(egui::ProgressBar::new(gpu.usage_percent / 100.0)
                        .text(format!("{:.1}%", gpu.usage_percent))
                        .fill(color)
                        .desired_width(350.0)
                        .desired_height(22.0));
                });
                
                ui.add_space(Spacing::SMALL);
                
                // Memory usage with improved formatting
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Memory Usage:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::SMALL);
                    let color = ColorPalette::usage_color(gpu.memory_percent);
                    ui.add(egui::ProgressBar::new(gpu.memory_percent / 100.0)
                        .text(format!("{:.1}% ({} / {})",
                            gpu.memory_percent,
                            format_bytes(gpu.memory_used_bytes),
                            format_bytes(gpu.memory_total_bytes)
                        ))
                        .fill(color)
                        .desired_width(350.0)
                        .desired_height(22.0));
                });
                
                ui.add_space(Spacing::MEDIUM);
                
                // Temperature and power with icons and better colors
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🌡 Temperature:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::TINY);
                    let temp_color = ColorPalette::temperature_color(gpu.temperature_celsius);
                    ui.label(RichText::new(format!("{:.1}°C", gpu.temperature_celsius))
                        .color(temp_color)
                        .size(Typography::BODY_SIZE)
                        .strong());
                    
                    ui.add_space(Spacing::MEDIUM);
                    ui.separator();
                    ui.add_space(Spacing::MEDIUM);
                    
                    ui.label(RichText::new("⚡ Power:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::TINY);
                    ui.label(RichText::new(format!("{:.1} W", gpu.power_watts))
                        .color(ColorPalette::WARNING)
                        .size(Typography::BODY_SIZE)
                        .strong());
                    
                    ui.add_space(Spacing::MEDIUM);
                    ui.separator();
                    ui.add_space(Spacing::MEDIUM);
                    
                    ui.label(RichText::new("🌀 Fan Speed:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::TINY);
                    ui.label(RichText::new(format!("{:.0}%", gpu.fan_speed_percent))
                        .color(ColorPalette::INFO)
                        .size(Typography::BODY_SIZE)
                        .strong());
                });
            });
            
            ui.add_space(Spacing::MEDIUM);
            
            // Detailed metrics table with improved design
            ui.label(RichText::new("📊 Detailed Metrics")
                .size(Typography::SUBHEADING_SIZE)
                .strong());
            ui.add_space(Spacing::SMALL);
            
            egui::Grid::new("gpu_metrics_grid")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("GPU Name:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(&gpu.name)
                        .size(Typography::BODY_SIZE)
                        .strong());
                    ui.end_row();
                    
                    ui.label(RichText::new("GPU Utilization:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("{:.1}%", gpu.usage_percent))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::usage_color(gpu.usage_percent)));
                    ui.end_row();
                    
                    ui.label(RichText::new("Memory Total:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format_bytes(gpu.memory_total_bytes))
                        .size(Typography::BODY_SIZE));
                    ui.end_row();
                    
                    ui.label(RichText::new("Memory Used:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format_bytes(gpu.memory_used_bytes))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::WARNING));
                    ui.end_row();
                    
                    ui.label(RichText::new("Memory Free:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format_bytes(gpu.memory_total_bytes.saturating_sub(gpu.memory_used_bytes)))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::SUCCESS));
                    ui.end_row();
                    
                    ui.label(RichText::new("Memory Usage:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("{:.1}%", gpu.memory_percent))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::usage_color(gpu.memory_percent)));
                    ui.end_row();
                    
                    ui.label(RichText::new("Temperature:")
                        .size(Typography::BODY_SIZE));
                    let temp_color = ColorPalette::temperature_color(gpu.temperature_celsius);
                    ui.label(RichText::new(format!("{:.1}°C", gpu.temperature_celsius))
                        .color(temp_color)
                        .size(Typography::BODY_SIZE)
                        .strong());
                    ui.end_row();
                    
                    ui.label(RichText::new("Power Consumption:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("{:.1} W", gpu.power_watts))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::WARNING));
                    ui.end_row();
                    
                    ui.label(RichText::new("Fan Speed:")
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("{:.0}%", gpu.fan_speed_percent))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::INFO));
                    ui.end_row();
                });
        }
        None => {
            // Improved empty state with better visual design
            ui.vertical_centered(|ui| {
                ui.add_space(Spacing::XXLARGE);
                
                ui.label(RichText::new("🎮")
                    .size(48.0));
                ui.add_space(Spacing::SMALL);
                
                ui.label(RichText::new("No GPU Detected")
                    .size(Typography::SUBHEADING_SIZE)
                    .strong()
                    .color(ColorPalette::TEXT_SECONDARY));
                
                ui.add_space(Spacing::MEDIUM);
                
                ui.label(RichText::new("GPU monitoring is not available")
                    .size(Typography::BODY_SIZE)
                    .color(ColorPalette::TEXT_MUTED));
                
                ui.add_space(Spacing::LARGE);
                
                ui.group(|ui| {
                    ui.set_max_width(400.0);
                    ui.label(RichText::new("Requirements:")
                        .size(Typography::BODY_SIZE)
                        .strong());
                    ui.add_space(Spacing::TINY);
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("•")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::INFO));
                        ui.label(RichText::new("NVIDIA GPU with CUDA support")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("•")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::INFO));
                        ui.label(RichText::new("nvidia-smi utility installed")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("•")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::INFO));
                        ui.label(RichText::new("Application built with 'gpu-nvidia' feature")
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                });
                
                ui.add_space(Spacing::XXLARGE);
            });
        }
    }
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

