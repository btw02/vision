//! Alerts view
//!
//! Display and manage system alerts.

use egui::{Ui, Context, RichText};
use crate::models::state::{AppState, AlertSeverity};
use super::theme::{ColorPalette, Typography, Spacing};

/// Render the alerts view
pub fn render(_ctx: &Context, ui: &mut Ui, state: &mut AppState) {
    // Page header
    ui.add_space(Spacing::SMALL);
    ui.label(RichText::new("System Alerts")
        .size(Typography::HEADING_SIZE)
        .strong());
    ui.add_space(Spacing::TINY);
    ui.separator();
    ui.add_space(Spacing::MEDIUM);
    
    if state.active_alerts.is_empty() {
        // Improved empty state
        ui.vertical_centered(|ui| {
            ui.add_space(Spacing::XXLARGE);
            
            ui.label(RichText::new("✓")
                .size(64.0)
                .color(ColorPalette::SUCCESS));
            ui.add_space(Spacing::MEDIUM);
            
            ui.label(RichText::new("No Active Alerts")
                .size(Typography::SUBHEADING_SIZE)
                .strong()
                .color(ColorPalette::SUCCESS));
            ui.add_space(Spacing::SMALL);
            
            ui.label(RichText::new("Your system is running normally")
                .size(Typography::BODY_SIZE)
                .color(ColorPalette::TEXT_SECONDARY));
            
            ui.add_space(Spacing::XXLARGE);
        });
        return;
    }
    
    // Alert summary with improved design
    ui.horizontal(|ui| {
        let critical_count = state.active_alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Critical))
            .count();
        let warning_count = state.active_alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Warning))
            .count();
        let info_count = state.active_alerts.iter()
            .filter(|a| matches!(a.severity, AlertSeverity::Info))
            .count();
        
        if critical_count > 0 {
            ui.label(RichText::new(format!("🔴 {} Critical", critical_count))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::ERROR));
            ui.add_space(Spacing::MEDIUM);
        }
        if warning_count > 0 {
            ui.label(RichText::new(format!("🟡 {} Warning", warning_count))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::WARNING));
            ui.add_space(Spacing::MEDIUM);
        }
        if info_count > 0 {
            ui.label(RichText::new(format!("🔵 {} Info", info_count))
                .size(Typography::BODY_SIZE)
                .strong()
                .color(ColorPalette::INFO));
        }
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(RichText::new("🗑 Clear All")
                .size(Typography::BODY_SIZE)).clicked() {
                state.clear_alerts();
            }
        });
    });
    
    ui.add_space(Spacing::MEDIUM);
    
    // Alert list with improved card design
    let mut alerts_to_remove = Vec::new();
    
    for alert in &state.active_alerts {
        let (icon, color) = match alert.severity {
            AlertSeverity::Critical => ("🔴", ColorPalette::ERROR),
            AlertSeverity::Warning => ("🟡", ColorPalette::WARNING),
            AlertSeverity::Info => ("🔵", ColorPalette::INFO),
        };
        
        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Icon with larger size
                ui.label(RichText::new(icon)
                    .size(24.0));
                ui.add_space(Spacing::SMALL);
                
                // Alert content
                ui.vertical(|ui| {
                    ui.label(RichText::new(&alert.name)
                        .strong()
                        .size(Typography::SUBHEADING_SIZE)
                        .color(color));
                    ui.add_space(Spacing::TINY);
                    
                    ui.label(RichText::new(&alert.message)
                        .size(Typography::BODY_SIZE));
                    
                    ui.add_space(Spacing::SMALL);
                    
                    // Alert metadata
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("🕐 {}",
                            alert.triggered_at.format("%Y-%m-%d %H:%M:%S")))
                            .color(ColorPalette::TEXT_MUTED)
                            .size(Typography::SMALL_SIZE));
                        
                        ui.add_space(Spacing::SMALL);
                        ui.separator();
                        ui.add_space(Spacing::SMALL);
                        
                        ui.label(RichText::new(format!("📊 Value: {:.2} (Threshold: {:.2})",
                            alert.current_value, alert.threshold))
                            .color(ColorPalette::TEXT_MUTED)
                            .size(Typography::SMALL_SIZE));
                    });
                });
                
                // Dismiss button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("✕ Dismiss")
                        .size(Typography::BODY_SIZE)).clicked() {
                        alerts_to_remove.push(alert.id.clone());
                    }
                });
            });
        });
        
        ui.add_space(Spacing::SMALL);
    }
    
    // Remove dismissed alerts
    for id in alerts_to_remove {
        state.remove_alert(&id);
    }
    
    ui.add_space(Spacing::LARGE);
    
    // Alert configuration hint with better styling
    ui.separator();
    ui.add_space(Spacing::SMALL);
    ui.horizontal(|ui| {
        ui.label(RichText::new("💡")
            .size(Typography::SUBHEADING_SIZE));
        ui.add_space(Spacing::TINY);
        ui.vertical(|ui| {
            ui.label(RichText::new("Tip")
                .strong()
                .size(Typography::BODY_SIZE));
            ui.label(RichText::new("Configure alert thresholds in the Settings tab")
                .size(Typography::SMALL_SIZE)
                .color(ColorPalette::TEXT_SECONDARY));
        });
    });
}

