//! Export functionality
//!
//! Export metrics to JSON format. CSV export is not implemented.

use anyhow::Result;
use std::path::Path;
use crate::models::metrics::SystemMetrics;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// CSV format (not implemented)
    Csv,
    /// JSON format (implemented)
    Json,
}

/// Metrics exporter
pub struct Exporter;

impl Exporter {
    /// Export metrics to a file
    ///
    /// Only JSON format is currently implemented. CSV export will return an error.
    pub fn export(metrics: &[SystemMetrics], path: &Path, format: ExportFormat) -> Result<()> {
        match format {
            ExportFormat::Csv => Self::export_csv(metrics, path),
            ExportFormat::Json => Self::export_json(metrics, path),
        }
    }
    
    /// Export to CSV format (not implemented)
    fn export_csv(_metrics: &[SystemMetrics], _path: &Path) -> Result<()> {
        anyhow::bail!("CSV export is not implemented. Use JSON format instead.")
    }
    
    /// Export to JSON format
    ///
    /// Serializes the metrics array to a pretty-printed JSON file.
    fn export_json(metrics: &[SystemMetrics], path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(metrics)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

