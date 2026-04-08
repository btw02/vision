//! Disk view
//!
//! Disk usage and storage information.

use egui::{Ui, Context, RichText};
use crate::models::{metrics::SystemMetrics, state::AppState};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the disk view
pub fn render(_ctx: &Context, ui: &mut Ui, metrics: &SystemMetrics, _state: &AppState) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("Disk Storage")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    if metrics.disks.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(Spacing::XXLARGE);
            ui.label(RichText::new("💾 No disk information available")
                .size(Typography::SUBHEADING_SIZE)
                .color(ColorPalette::TEXT_MUTED));
            ui.add_space(Spacing::XXLARGE);
        });
        return;
    }
    
    // Disk summary cards with improved visual design
    for disk in &metrics.disks {
        ui.group(|ui| {
            // Header with mount point and filesystem type
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("💾 {}", &disk.mount_point))
                    .size(Typography::SUBHEADING_SIZE)
                    .strong());
                ui.add_space(Spacing::SMALL);
                ui.label(RichText::new(&disk.fs_type)
                    .size(Typography::SMALL_SIZE)
                    .color(ColorPalette::TEXT_MUTED));
            });
            
            ui.add_space(Spacing::SMALL);
            
            // Device info
            ui.horizontal(|ui| {
                ui.label(RichText::new("Device:")
                    .size(Typography::BODY_SIZE)
                    .color(ColorPalette::TEXT_SECONDARY));
                ui.add_space(Spacing::TINY);
                ui.label(RichText::new(&disk.device)
                    .size(Typography::BODY_SIZE));
            });
            
            ui.add_space(Spacing::SMALL);
            
            // Usage bar with better styling
            let usage_color = ColorPalette::usage_color(disk.usage_percent);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Usage:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::SMALL);
                ui.add(egui::ProgressBar::new(disk.usage_percent / 100.0)
                    .text(format!("{:.1}%", disk.usage_percent))
                    .fill(usage_color)
                    .desired_width(350.0)
                    .desired_height(22.0));
            });
            
            ui.add_space(Spacing::SMALL);
            
            // Space information with color coding
            ui.horizontal(|ui| {
                ui.label(RichText::new("Used:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::TINY);
                ui.label(RichText::new(format_bytes(disk.used_bytes))
                    .size(Typography::BODY_SIZE)
                    .strong()
                    .color(ColorPalette::WARNING));
                
                ui.add_space(Spacing::MEDIUM);
                ui.separator();
                ui.add_space(Spacing::MEDIUM);
                
                ui.label(RichText::new("Available:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::TINY);
                ui.label(RichText::new(format_bytes(disk.available_bytes))
                    .size(Typography::BODY_SIZE)
                    .strong()
                    .color(ColorPalette::SUCCESS));
                
                ui.add_space(Spacing::MEDIUM);
                ui.separator();
                ui.add_space(Spacing::MEDIUM);
                
                ui.label(RichText::new("Total:")
                    .size(Typography::BODY_SIZE));
                ui.add_space(Spacing::TINY);
                ui.label(RichText::new(format_bytes(disk.total_bytes))
                    .size(Typography::BODY_SIZE)
                    .strong()
                    .color(ColorPalette::INFO));
            });
            
            // I/O speeds (if available)
            if disk.read_speed_bps > 0 || disk.write_speed_bps > 0 {
                ui.add_space(Spacing::SMALL);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📖 Read Speed:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::TINY);
                    ui.label(RichText::new(format!("{}/s", format_bytes(disk.read_speed_bps)))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::CHART_PRIMARY));
                    
                    ui.add_space(Spacing::MEDIUM);
                    ui.separator();
                    ui.add_space(Spacing::MEDIUM);
                    
                    ui.label(RichText::new("✍ Write Speed:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::TINY);
                    ui.label(RichText::new(format!("{}/s", format_bytes(disk.write_speed_bps)))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::CHART_SECONDARY));
                });
            }
        });
        
        ui.add_space(Spacing::MEDIUM);
    }
    
    ui.add_space(Spacing::SMALL);
    
    // Detailed table with improved design
    ui.label(RichText::new("📋 Detailed Information")
        .size(Typography::SUBHEADING_SIZE)
        .strong());
    ui.add_space(Spacing::SMALL);
    
    use egui_extras::{TableBuilder, Column};
    
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(150.0))  // Mount point
        .column(Column::exact(150.0))  // Device
        .column(Column::exact(80.0))   // FS Type
        .column(Column::exact(110.0))  // Total
        .column(Column::exact(110.0))  // Used
        .column(Column::exact(110.0))  // Available
        .column(Column::exact(90.0))   // Usage %
        .header(28.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Mount Point")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Device")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Type")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Total")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Used")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Available")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Usage")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
        })
        .body(|mut body| {
            for disk in &metrics.disks {
                body.row(24.0, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(&disk.mount_point)
                            .size(Typography::BODY_SIZE)
                            .strong());
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(&disk.device)
                            .size(Typography::SMALL_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(&disk.fs_type)
                            .size(Typography::SMALL_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(disk.total_bytes))
                            .size(Typography::BODY_SIZE));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(disk.used_bytes))
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::WARNING));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(disk.available_bytes))
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::SUCCESS));
                    });
                    row.col(|ui| {
                        let color = ColorPalette::usage_color(disk.usage_percent);
                        ui.label(RichText::new(format!("{:.1}%", disk.usage_percent))
                            .color(color)
                            .size(Typography::BODY_SIZE)
                            .strong());
                    });
                });
            }
        });
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    const PB: u64 = TB * 1024;
    
    if bytes >= PB {
        format!("{:.2} PB", bytes as f64 / PB as f64)
    } else if bytes >= TB {
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

