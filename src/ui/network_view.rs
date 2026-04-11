//! Network view
//!
//! Network interface statistics and traffic monitoring.

use egui::{Ui, Context, RichText};
use crate::models::{metrics::SystemMetrics, state::AppState};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the network view
pub fn render(_ctx: &Context, ui: &mut Ui, metrics: &SystemMetrics, _state: &AppState) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("Network Monitor")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);

    // Overall network statistics with improved visual design
    ui.group(|ui| {
        ui.label(RichText::new("📊 Overall Statistics")
            .size(Typography::SUBHEADING_SIZE)
            .strong());
        ui.add_space(Spacing::SMALL);

        // Total data transferred
        ui.horizontal(|ui| {
            ui.label(RichText::new("⬇ Total Download:")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format_bytes(metrics.network.total_rx_bytes))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::NETWORK_RX));

            ui.add_space(Spacing::MEDIUM);
            ui.separator();
            ui.add_space(Spacing::MEDIUM);

            ui.label(RichText::new("⬆ Total Upload:")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format_bytes(metrics.network.total_tx_bytes))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::NETWORK_TX));
        });

        ui.add_space(Spacing::SMALL);

        // Current speeds with visual emphasis
        ui.horizontal(|ui| {
            ui.label(RichText::new("⚡ Download Speed:")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format!("{}/s", format_bytes(metrics.network.rx_speed_bps)))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::SUCCESS));

            ui.add_space(Spacing::MEDIUM);
            ui.separator();
            ui.add_space(Spacing::MEDIUM);

            ui.label(RichText::new("⚡ Upload Speed:")
                .size(Typography::BODY_SIZE));
            ui.add_space(Spacing::SMALL);
            ui.label(RichText::new(format!("{}/s", format_bytes(metrics.network.tx_speed_bps)))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::SUCCESS));
        });
    });

    ui.add_space(Spacing::MEDIUM);

    // Network interfaces with improved table design
    ui.label(RichText::new("🌐 Network Interfaces")
        .size(Typography::SUBHEADING_SIZE)
        .strong());
    ui.add_space(Spacing::SMALL);

    if metrics.network.interfaces.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(Spacing::XLARGE);
            ui.label(RichText::new("No network interfaces found")
                .size(Typography::BODY_SIZE)
                .color(ColorPalette::TEXT_MUTED));
            ui.add_space(Spacing::XLARGE);
        });
        return;
    }

    use egui_extras::{TableBuilder, Column};

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(150.0))  // Interface name
        .column(Column::exact(100.0))  // Status
        .column(Column::exact(130.0))  // RX Bytes
        .column(Column::exact(130.0))  // TX Bytes
        .column(Column::exact(110.0))  // RX Packets
        .column(Column::exact(110.0))  // TX Packets
        .column(Column::exact(100.0))  // RX Errors
        .column(Column::exact(100.0))  // TX Errors
        .header(28.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Interface")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("Status")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("⬇ Downloaded")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("⬆ Uploaded")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("RX Packets")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("TX Packets")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("RX Errors")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                ui.label(RichText::new("TX Errors")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
        })
        .body(|mut body| {
            for interface in &metrics.network.interfaces {
                body.row(24.0, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(&interface.name)
                            .size(Typography::BODY_SIZE)
                            .strong());
                    });
                    row.col(|ui| {
                        let (text, color) = if interface.is_up {
                            ("✓ UP", ColorPalette::SUCCESS)
                        } else {
                            ("✗ DOWN", ColorPalette::ERROR)
                        };
                        ui.label(RichText::new(text)
                            .color(color)
                            .size(Typography::BODY_SIZE)
                            .strong());
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(interface.rx_bytes))
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::NETWORK_RX));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(interface.tx_bytes))
                            .size(Typography::BODY_SIZE)
                            .color(ColorPalette::NETWORK_TX));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_number(interface.rx_packets))
                            .size(Typography::SMALL_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_number(interface.tx_packets))
                            .size(Typography::SMALL_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    row.col(|ui| {
                        let color = if interface.rx_errors > 0 {
                            ColorPalette::ERROR
                        } else {
                            ColorPalette::TEXT_MUTED
                        };
                        ui.label(RichText::new(format_number(interface.rx_errors))
                            .color(color)
                            .size(Typography::SMALL_SIZE));
                    });
                    row.col(|ui| {
                        let color = if interface.tx_errors > 0 {
                            ColorPalette::ERROR
                        } else {
                            ColorPalette::TEXT_MUTED
                        };
                        ui.label(RichText::new(format_number(interface.tx_errors))
                            .color(color)
                            .size(Typography::SMALL_SIZE));
                    });
                });
            }
        });

    ui.add_space(Spacing::MEDIUM);

    // Interface details with improved collapsible sections
    ui.label(RichText::new("Interface Details")
        .size(Typography::SUBHEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);

    for interface in &metrics.network.interfaces {
        ui.collapsing(
            RichText::new(&interface.name)
                .size(Typography::BODY_SIZE)
                .strong(),
            |ui| {
                ui.add_space(Spacing::TINY);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Status:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::SMALL);
                    let (text, color) = if interface.is_up {
                        ("✓ UP", ColorPalette::SUCCESS)
                    } else {
                        ("✗ DOWN", ColorPalette::ERROR)
                    };
                    ui.label(RichText::new(text)
                        .color(color)
                        .size(Typography::BODY_SIZE)
                        .strong());
                });

                ui.add_space(Spacing::TINY);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("⬇ Downloaded:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::SMALL);
                    ui.label(RichText::new(format_bytes(interface.rx_bytes))
                        .color(ColorPalette::NETWORK_RX)
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("({} packets)", format_number(interface.rx_packets)))
                        .size(Typography::SMALL_SIZE)
                        .color(ColorPalette::TEXT_SECONDARY));
                });

                ui.add_space(Spacing::TINY);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("⬆ Uploaded:")
                        .size(Typography::BODY_SIZE));
                    ui.add_space(Spacing::SMALL);
                    ui.label(RichText::new(format_bytes(interface.tx_bytes))
                        .color(ColorPalette::NETWORK_TX)
                        .size(Typography::BODY_SIZE));
                    ui.label(RichText::new(format!("({} packets)", format_number(interface.tx_packets)))
                        .size(Typography::SMALL_SIZE)
                        .color(ColorPalette::TEXT_SECONDARY));
                });

                if interface.rx_errors > 0 || interface.tx_errors > 0 {
                    ui.add_space(Spacing::TINY);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("⚠ Errors:")
                            .color(ColorPalette::ERROR)
                            .size(Typography::BODY_SIZE)
                            .strong());
                        ui.add_space(Spacing::SMALL);
                        ui.label(RichText::new(format!("RX: {}, TX: {}",
                            format_number(interface.rx_errors),
                            format_number(interface.tx_errors)
                        ))
                        .size(Typography::BODY_SIZE)
                        .color(ColorPalette::ERROR));
                    });
                }

                ui.add_space(Spacing::TINY);
            }
        );
    }
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

/// Format large numbers with commas
fn format_number(num: u64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}
