//! Formatting utility functions
//!
//! Functions for formatting values for display.

use std::time::Duration;

/// Format bytes to human-readable string (e.g., "1.5 GB")
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Format duration to human-readable string (e.g., "2h 30m 15s")
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{}s", seconds));
    }
    
    parts.join(" ")
}

/// Format percentage with specified decimal places
pub fn format_percentage(value: f32, decimals: usize) -> String {
    format!("{:.decimals$}%", value, decimals = decimals)
}

/// Format frequency in Hz to human-readable string (e.g., "2.4 GHz")
pub fn format_frequency(hz: u64) -> String {
    if hz >= 1_000_000_000 {
        format!("{:.2} GHz", hz as f64 / 1_000_000_000.0)
    } else if hz >= 1_000_000 {
        format!("{:.2} MHz", hz as f64 / 1_000_000.0)
    } else if hz >= 1_000 {
        format!("{:.2} KHz", hz as f64 / 1_000.0)
    } else {
        format!("{} Hz", hz)
    }
}

/// Format speed in bytes per second to human-readable string (e.g., "10.5 MB/s")
pub fn format_speed(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1536 * 1024 * 1024), "1.50 GB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }
    
    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(50.0, 0), "50%");
        assert_eq!(format_percentage(50.5, 1), "50.5%");
        assert_eq!(format_percentage(50.55, 2), "50.55%");
    }
}

