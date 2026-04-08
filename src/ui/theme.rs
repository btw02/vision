//! Theme management
//!
//! Handles application theming and visual styles with a modern color palette.

use egui::{Context, Visuals, Color32, Stroke, Rounding, Style};

/// Application theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    /// Apply the theme to the egui context
    pub fn apply(&self, ctx: &Context) {
        let mut style = Style::default();
        
        match self {
            Theme::Dark => {
                ctx.set_visuals(Visuals::dark());
                
                // Customize dark theme
                let mut visuals = Visuals::dark();
                
                // Modern dark color palette
                visuals.window_fill = Color32::from_rgb(24, 24, 27);
                visuals.panel_fill = Color32::from_rgb(24, 24, 27);
                visuals.faint_bg_color = Color32::from_rgb(39, 39, 42);
                visuals.extreme_bg_color = Color32::from_rgb(9, 9, 11);
                
                // Improved contrast for text
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(228, 228, 231));
                visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(161, 161, 170));
                
                // Interactive elements
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(63, 63, 70);
                visuals.widgets.active.bg_fill = Color32::from_rgb(82, 82, 91);
                
                // Selection color
                visuals.selection.bg_fill = Color32::from_rgba_premultiplied(59, 130, 246, 100);
                visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(59, 130, 246));
                
                // Hyperlinks
                visuals.hyperlink_color = Color32::from_rgb(96, 165, 250);
                
                // Rounded corners for modern look
                visuals.window_rounding = Rounding::same(8.0);
                visuals.menu_rounding = Rounding::same(6.0);
                
                ctx.set_visuals(visuals);
                
                // Spacing improvements
                style.spacing.item_spacing = egui::vec2(8.0, 6.0);
                style.spacing.button_padding = egui::vec2(12.0, 6.0);
                style.spacing.window_margin = egui::Margin::same(12.0);
                style.spacing.menu_margin = egui::Margin::same(8.0);
            }
            Theme::Light => {
                ctx.set_visuals(Visuals::light());
                
                // Customize light theme
                let mut visuals = Visuals::light();
                
                // Modern light color palette
                visuals.window_fill = Color32::from_rgb(255, 255, 255);
                visuals.panel_fill = Color32::from_rgb(250, 250, 250);
                visuals.faint_bg_color = Color32::from_rgb(244, 244, 245);
                visuals.extreme_bg_color = Color32::from_rgb(228, 228, 231);
                
                // Text colors
                visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(24, 24, 27));
                visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(82, 82, 91));
                
                // Interactive elements
                visuals.widgets.hovered.bg_fill = Color32::from_rgb(228, 228, 231);
                visuals.widgets.active.bg_fill = Color32::from_rgb(212, 212, 216);
                
                // Selection color
                visuals.selection.bg_fill = Color32::from_rgba_premultiplied(59, 130, 246, 100);
                visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(59, 130, 246));
                
                // Hyperlinks
                visuals.hyperlink_color = Color32::from_rgb(37, 99, 235);
                
                // Rounded corners
                visuals.window_rounding = Rounding::same(8.0);
                visuals.menu_rounding = Rounding::same(6.0);
                
                ctx.set_visuals(visuals);
                
                // Spacing improvements
                style.spacing.item_spacing = egui::vec2(8.0, 6.0);
                style.spacing.button_padding = egui::vec2(12.0, 6.0);
                style.spacing.window_margin = egui::Margin::same(12.0);
                style.spacing.menu_margin = egui::Margin::same(8.0);
            }
        }
        
        ctx.set_style(style);
    }
    
    /// Parse theme from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }
    
    /// Convert theme to string
    pub fn to_string(&self) -> String {
        match self {
            Theme::Dark => "Dark".to_string(),
            Theme::Light => "Light".to_string(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// Modern color palette for consistent UI
pub struct ColorPalette;

impl ColorPalette {
    // Status colors
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
    pub const WARNING: Color32 = Color32::from_rgb(251, 146, 60);
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const INFO: Color32 = Color32::from_rgb(59, 130, 246);
    
    // Usage level colors (for progress bars)
    pub const USAGE_LOW: Color32 = Color32::from_rgb(59, 130, 246);      // Blue
    pub const USAGE_MODERATE: Color32 = Color32::from_rgb(34, 197, 94);  // Green
    pub const USAGE_HIGH: Color32 = Color32::from_rgb(251, 146, 60);     // Orange
    pub const USAGE_CRITICAL: Color32 = Color32::from_rgb(239, 68, 68);  // Red
    
    // Temperature colors
    pub const TEMP_COOL: Color32 = Color32::from_rgb(59, 130, 246);      // Blue
    pub const TEMP_NORMAL: Color32 = Color32::from_rgb(34, 197, 94);     // Green
    pub const TEMP_WARM: Color32 = Color32::from_rgb(251, 146, 60);      // Orange
    pub const TEMP_HOT: Color32 = Color32::from_rgb(239, 68, 68);        // Red
    
    // Data visualization colors
    pub const CHART_PRIMARY: Color32 = Color32::from_rgb(59, 130, 246);
    pub const CHART_SECONDARY: Color32 = Color32::from_rgb(168, 85, 247);
    pub const CHART_TERTIARY: Color32 = Color32::from_rgb(236, 72, 153);
    
    // Network colors
    pub const NETWORK_RX: Color32 = Color32::from_rgb(34, 197, 94);      // Download - Green
    pub const NETWORK_TX: Color32 = Color32::from_rgb(251, 146, 60);     // Upload - Orange
    
    // Text colors
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(161, 161, 170);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(113, 113, 122);
    
    /// Get color based on usage percentage
    pub fn usage_color(usage: f32) -> Color32 {
        if usage > 90.0 {
            Self::USAGE_CRITICAL
        } else if usage > 75.0 {
            Self::USAGE_HIGH
        } else if usage > 50.0 {
            Self::USAGE_MODERATE
        } else {
            Self::USAGE_LOW
        }
    }
    
    /// Get color based on temperature (Celsius)
    pub fn temperature_color(temp: f32) -> Color32 {
        if temp > 85.0 {
            Self::TEMP_HOT
        } else if temp > 75.0 {
            Self::TEMP_WARM
        } else if temp > 60.0 {
            Self::TEMP_NORMAL
        } else {
            Self::TEMP_COOL
        }
    }
}

/// Typography helpers for consistent text styling
pub struct Typography;

impl Typography {
    pub const HEADING_SIZE: f32 = 20.0;
    pub const SUBHEADING_SIZE: f32 = 16.0;
    pub const BODY_SIZE: f32 = 14.0;
    pub const SMALL_SIZE: f32 = 12.0;
    pub const TINY_SIZE: f32 = 10.0;
}

/// Spacing constants for consistent layout
pub struct Spacing;

impl Spacing {
    pub const TINY: f32 = 4.0;
    pub const SMALL: f32 = 8.0;
    pub const MEDIUM: f32 = 12.0;
    pub const LARGE: f32 = 16.0;
    pub const XLARGE: f32 = 24.0;
    pub const XXLARGE: f32 = 32.0;
}
