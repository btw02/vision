//! Dashboard view
//!
//! Main dashboard showing system overview and real-time metrics.

use egui::{Ui, Context, RichText};
use crate::models::{metrics::SystemMetrics, state::AppState};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the dashboard view
pub fn render(_ctx: &Context, ui: &mut Ui, metrics: &SystemMetrics, state: &AppState) {
    // Page header with better typography
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("System Dashboard")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    // CPU Section with improved visual hierarchy
    ui.group(|ui| {
        ui.label(RichText::new("🖥️ CPU")
            .size(Typography::SUBHEADING_SIZE)
            .strong());
        ui.add_space(Spacing::SMALL);
        
        // Overall CPU usage with better layout
        ui.horizontal(|ui| {
            ui.label(RichText::new("Overall Usage")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::MEDIUM);
            
            let cpu_usage = metrics.cpu.usage_percent;
            let color = ColorPalette::usage_color(cpu_usage);
            
            ui.add(egui::ProgressBar::new(cpu_usage / 100.0)
                .text(format!("{:.1}%", cpu_usage))
                .fill(color)
                .desired_height(20.0));
        });
        
        ui.add_space(Spacing::SMALL);
        
        // CPU info with better spacing
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Cores: {}", metrics.cpu.core_count))
                .color(ColorPalette::TEXT_SECONDARY));
            ui.add_space(Spacing::SMALL);
            ui.separator();
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format!("Frequency: {} MHz", metrics.cpu.frequency_mhz))
                .color(ColorPalette::TEXT_SECONDARY));
            ui.add_space(Spacing::SMALL);
            ui.separator();
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format!("Load Avg: {:.2}, {:.2}, {:.2}",
                metrics.cpu.load_average.0,
                metrics.cpu.load_average.1,
                metrics.cpu.load_average.2
            )).color(ColorPalette::TEXT_SECONDARY));
        });
        
        ui.add_space(Spacing::SMALL);
        
        // Per-core usage with improved layout
        if !metrics.cpu.per_core_usage.is_empty() {
            ui.add_space(Spacing::TINY);
            ui.label(RichText::new("Per-Core Usage")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::TINY);
            
            // Display cores in a responsive grid
            let cores_per_row = 4;
            let mut core_index = 0;
            
            while core_index < metrics.cpu.per_core_usage.len() {
                ui.horizontal(|ui| {
                    for _ in 0..cores_per_row {
                        if core_index < metrics.cpu.per_core_usage.len() {
                            let usage = metrics.cpu.per_core_usage[core_index];
                            let color = ColorPalette::usage_color(usage);
                            
                            ui.vertical(|ui| {
                                ui.label(RichText::new(format!("Core {}", core_index))
                                    .size(Typography::SMALL_SIZE)
                                    .color(ColorPalette::TEXT_SECONDARY));
                                ui.add(egui::ProgressBar::new(usage / 100.0)
                                    .text(format!("{:.0}%", usage))
                                    .fill(color)
                                    .desired_width(85.0)
                                    .desired_height(18.0));
                            });
                            
                            core_index += 1;
                            if core_index < metrics.cpu.per_core_usage.len() {
                                ui.add_space(Spacing::SMALL);
                            }
                        }
                    }
                });
                ui.add_space(Spacing::TINY);
            }
        }
    });
    
    ui.add_space(Spacing::MEDIUM);
    
    // Memory Section with improved visual design
    ui.group(|ui| {
        ui.label(RichText::new("💾 Memory")
            .size(Typography::SUBHEADING_SIZE)
            .strong());
        ui.add_space(Spacing::SMALL);
        
        // RAM usage with better formatting
        ui.horizontal(|ui| {
            ui.label(RichText::new("RAM")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::MEDIUM);
            
            let mem_usage = metrics.memory.usage_percent;
            let color = ColorPalette::usage_color(mem_usage);
            
            ui.add(egui::ProgressBar::new(mem_usage / 100.0)
                .text(format!("{:.1}% ({} / {})",
                    mem_usage,
                    format_bytes(metrics.memory.used_bytes),
                    format_bytes(metrics.memory.total_bytes)
                ))
                .fill(color)
                .desired_height(20.0));
        });
        
        ui.add_space(Spacing::SMALL);
        
        // Swap usage with conditional display
        if metrics.memory.swap_total_bytes > 0 {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Swap")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::MEDIUM);
                
                let swap_usage = if metrics.memory.swap_total_bytes > 0 {
                    (metrics.memory.swap_used_bytes as f32 / metrics.memory.swap_total_bytes as f32) * 100.0
                } else {
                    0.0
                };
                let color = ColorPalette::usage_color(swap_usage);
                
                ui.add(egui::ProgressBar::new(swap_usage / 100.0)
                    .text(format!("{:.1}% ({} / {})",
                        swap_usage,
                        format_bytes(metrics.memory.swap_used_bytes),
                        format_bytes(metrics.memory.swap_total_bytes)
                    ))
                    .fill(color)
                    .desired_height(20.0));
            });
            ui.add_space(Spacing::SMALL);
        }
        
        // Memory details with better styling
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Available: {}", format_bytes(metrics.memory.available_bytes)))
                .color(ColorPalette::TEXT_SECONDARY)
                .size(Typography::SMALL_SIZE));
        });
    });
    
    ui.add_space(Spacing::MEDIUM);
    
    // Top Processes Section with improved table design
    ui.group(|ui| {
        ui.label(RichText::new("⚡ Top Processes (by CPU)")
            .size(Typography::SUBHEADING_SIZE)
            .strong());
        ui.add_space(Spacing::SMALL);
        
        use egui_extras::{TableBuilder, Column};
        
        let top_processes: Vec<_> = metrics.processes.iter().take(10).collect();
        
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(70.0))  // PID
            .column(Column::remainder().at_least(180.0))  // Name
            .column(Column::exact(90.0))  // CPU%
            .column(Column::exact(110.0))  // Memory
            .column(Column::exact(100.0))  // Status
            .header(24.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("PID")
                        .strong()
                        .size(Typography::BODY_SIZE));
                });
                header.col(|ui| {
                    ui.label(RichText::new("Name")
                        .strong()
                        .size(Typography::BODY_SIZE));
                });
                header.col(|ui| {
                    ui.label(RichText::new("CPU %")
                        .strong()
                        .size(Typography::BODY_SIZE));
                });
                header.col(|ui| {
                    ui.label(RichText::new("Memory")
                        .strong()
                        .size(Typography::BODY_SIZE));
                });
                header.col(|ui| {
                    ui.label(RichText::new("Status")
                        .strong()
                        .size(Typography::BODY_SIZE));
                });
            })
            .body(|mut body| {
                for process in top_processes {
                    body.row(22.0, |mut row| {
                        row.col(|ui| {
                            ui.label(RichText::new(process.pid.to_string())
                                .size(Typography::BODY_SIZE));
                        });
                        row.col(|ui| {
                            ui.label(RichText::new(&process.name)
                                .size(Typography::BODY_SIZE));
                        });
                        row.col(|ui| {
                            let color = if process.cpu_percent > 50.0 {
                                ColorPalette::ERROR
                            } else if process.cpu_percent > 20.0 {
                                ColorPalette::WARNING
                            } else {
                                ColorPalette::SUCCESS
                            };
                            ui.label(RichText::new(format!("{:.1}%", process.cpu_percent))
                                .color(color)
                                .size(Typography::BODY_SIZE));
                        });
                        row.col(|ui| {
                            ui.label(RichText::new(format_bytes(process.memory_bytes))
                                .size(Typography::BODY_SIZE));
                        });
                        row.col(|ui| {
                            ui.label(RichText::new(&process.status)
                                .size(Typography::SMALL_SIZE)
                                .color(ColorPalette::TEXT_SECONDARY));
                        });
                    });
                }
            });
    });
    
    ui.add_space(Spacing::MEDIUM);
    
    // Status bar with improved styling
    ui.separator();
    ui.add_space(Spacing::TINY);
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("🕐 Last Update: {}", metrics.timestamp.format("%H:%M:%S")))
            .size(Typography::SMALL_SIZE)
            .color(ColorPalette::TEXT_SECONDARY));
        ui.add_space(Spacing::SMALL);
        ui.separator();
        ui.add_space(Spacing::SMALL);
        
        let pause_color = if state.paused { ColorPalette::WARNING } else { ColorPalette::SUCCESS };
        ui.label(RichText::new(format!("⏸ Paused: {}", if state.paused { "Yes" } else { "No" }))
            .size(Typography::SMALL_SIZE)
            .color(pause_color));
        ui.add_space(Spacing::SMALL);
        ui.separator();
        ui.add_space(Spacing::SMALL);
        
        ui.label(RichText::new(format!("📊 Total Processes: {}", metrics.processes.len()))
            .size(Typography::SMALL_SIZE)
            .color(ColorPalette::TEXT_SECONDARY));
    });
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

