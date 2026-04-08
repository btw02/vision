//! Process view
//!
//! Detailed process list with filtering and sorting capabilities.

use egui::{Ui, Context, RichText};
use crate::models::{metrics::SystemMetrics, state::{AppState, ProcessSortColumn}};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the process view
pub fn render(_ctx: &Context, ui: &mut Ui, metrics: &SystemMetrics, state: &mut AppState) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("Process Manager")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    // Controls with improved layout
    ui.horizontal(|ui| {
        ui.label(RichText::new("🔍 Filter:")
            .size(Typography::BODY_SIZE));
        ui.add_space(Spacing::SMALL);
        
        let text_edit = egui::TextEdit::singleline(&mut state.process_filter)
            .hint_text("Search by name or PID...")
            .desired_width(250.0);
        ui.add(text_edit);
        
        ui.add_space(Spacing::SMALL);
        if ui.button(RichText::new("✕ Clear").size(Typography::BODY_SIZE)).clicked() {
            state.process_filter.clear();
        }
        
        ui.add_space(Spacing::MEDIUM);
        ui.separator();
        ui.add_space(Spacing::SMALL);
        
        ui.label(RichText::new(format!("📊 Total: {}", metrics.processes.len()))
            .size(Typography::BODY_SIZE)
            .color(ColorPalette::TEXT_SECONDARY));
    });
    
    ui.add_space(Spacing::MEDIUM);
    
    // Filter processes
    let filter_lower = state.process_filter.to_lowercase();
    let filtered_processes: Vec<_> = metrics.processes
        .iter()
        .filter(|p| {
            if filter_lower.is_empty() {
                true
            } else {
                p.name.to_lowercase().contains(&filter_lower) ||
                p.pid.to_string().contains(&filter_lower)
            }
        })
        .collect();
    
    // Show filtered count
    ui.label(RichText::new(format!("Showing {} of {} processes",
        filtered_processes.len(),
        metrics.processes.len()))
        .size(Typography::SMALL_SIZE)
        .color(ColorPalette::TEXT_SECONDARY));
    ui.add_space(Spacing::SMALL);
    
    // Process table with improved design
    use egui_extras::{TableBuilder, Column};
    
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::exact(80.0))   // PID
        .column(Column::remainder().at_least(220.0))  // Name
        .column(Column::exact(100.0))  // CPU%
        .column(Column::exact(120.0))  // Memory
        .column(Column::exact(100.0))  // Memory%
        .column(Column::exact(100.0))  // Status
        .column(Column::exact(80.0))   // Parent PID
        .header(28.0, |mut header| {
            // Helper function to create sortable header
            let make_header = |ui: &mut Ui, text: &str, column: ProcessSortColumn, current_column: ProcessSortColumn, ascending: bool| {
                let is_sorted = column == current_column;
                let arrow = if is_sorted {
                    if ascending { " ▲" } else { " ▼" }
                } else {
                    ""
                };
                let label = format!("{}{}", text, arrow);
                let color = if is_sorted { ColorPalette::INFO } else { ColorPalette::TEXT_SECONDARY };
                
                ui.button(RichText::new(label)
                    .strong()
                    .size(Typography::BODY_SIZE)
                    .color(color))
            };
            
            header.col(|ui| {
                if make_header(ui, "PID", ProcessSortColumn::Pid, state.process_sort_column, state.process_sort_ascending).clicked() {
                    state.process_sort_column = ProcessSortColumn::Pid;
                    state.process_sort_ascending = !state.process_sort_ascending;
                }
            });
            header.col(|ui| {
                if make_header(ui, "Name", ProcessSortColumn::Name, state.process_sort_column, state.process_sort_ascending).clicked() {
                    state.process_sort_column = ProcessSortColumn::Name;
                    state.process_sort_ascending = !state.process_sort_ascending;
                }
            });
            header.col(|ui| {
                if make_header(ui, "CPU %", ProcessSortColumn::Cpu, state.process_sort_column, state.process_sort_ascending).clicked() {
                    state.process_sort_column = ProcessSortColumn::Cpu;
                    state.process_sort_ascending = !state.process_sort_ascending;
                }
            });
            header.col(|ui| {
                ui.label(RichText::new("Memory")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
            header.col(|ui| {
                if make_header(ui, "Memory %", ProcessSortColumn::Memory, state.process_sort_column, state.process_sort_ascending).clicked() {
                    state.process_sort_column = ProcessSortColumn::Memory;
                    state.process_sort_ascending = !state.process_sort_ascending;
                }
            });
            header.col(|ui| {
                if make_header(ui, "Status", ProcessSortColumn::Status, state.process_sort_column, state.process_sort_ascending).clicked() {
                    state.process_sort_column = ProcessSortColumn::Status;
                    state.process_sort_ascending = !state.process_sort_ascending;
                }
            });
            header.col(|ui| {
                ui.label(RichText::new("Parent")
                    .strong()
                    .size(Typography::BODY_SIZE));
            });
        })
        .body(|mut body| {
            // Sort processes based on selected column
            let mut sorted_processes = filtered_processes.clone();
            sorted_processes.sort_by(|a, b| {
                let ordering = match state.process_sort_column {
                    ProcessSortColumn::Pid => a.pid.cmp(&b.pid),
                    ProcessSortColumn::Name => a.name.cmp(&b.name),
                    ProcessSortColumn::Cpu => {
                        b.cpu_percent.partial_cmp(&a.cpu_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }
                    ProcessSortColumn::Memory => {
                        b.memory_bytes.cmp(&a.memory_bytes)
                    }
                    ProcessSortColumn::Status => a.status.cmp(&b.status),
                };
                
                if state.process_sort_ascending {
                    ordering
                } else {
                    ordering.reverse()
                }
            });
            
            for process in sorted_processes {
                body.row(24.0, |mut row| {
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
                            .size(Typography::BODY_SIZE)
                            .strong());
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(format_bytes(process.memory_bytes))
                            .size(Typography::BODY_SIZE));
                    });
                    row.col(|ui| {
                        let color = if process.memory_percent > 10.0 {
                            ColorPalette::ERROR
                        } else if process.memory_percent > 5.0 {
                            ColorPalette::WARNING
                        } else {
                            ColorPalette::TEXT_MUTED
                        };
                        ui.label(RichText::new(format!("{:.2}%", process.memory_percent))
                            .color(color)
                            .size(Typography::BODY_SIZE));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(&process.status)
                            .size(Typography::SMALL_SIZE)
                            .color(ColorPalette::TEXT_SECONDARY));
                    });
                    row.col(|ui| {
                        if let Some(ppid) = process.parent_pid {
                            ui.label(RichText::new(ppid.to_string())
                                .size(Typography::SMALL_SIZE)
                                .color(ColorPalette::TEXT_SECONDARY));
                        } else {
                            ui.label(RichText::new("-")
                                .size(Typography::SMALL_SIZE)
                                .color(ColorPalette::TEXT_MUTED));
                        }
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

