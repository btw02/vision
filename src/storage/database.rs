//! Database management
//!
//! Historical metrics storage is not currently implemented.
//! The application keeps metrics in memory only (see AppState).

use anyhow::Result;
use rusqlite::Connection;
use crate::models::metrics::SystemMetrics;

/// Database manager for metrics storage (not implemented)
///
/// This is a placeholder for future database functionality.
/// Currently, the application stores metrics in memory only via AppState.
pub struct Database {
    _conn: Connection,
}

impl Database {
    /// Create a new database connection
    ///
    /// Note: Database storage is not implemented. This creates a connection
    /// but does not store any metrics.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { _conn: conn })
    }

    /// Store metrics snapshot (not implemented)
    pub fn store_metrics(&self, _metrics: &SystemMetrics) -> Result<()> {
        // Not implemented - metrics are stored in memory only
        Ok(())
    }

    /// Query metrics for a time range (not implemented)
    pub fn query_metrics(
        &self,
        _start: chrono::DateTime<chrono::Utc>,
        _end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<SystemMetrics>> {
        // Not implemented - use AppState.get_history() instead
        Ok(Vec::new())
    }

    /// Clean up old metrics (not implemented)
    pub fn cleanup_old_metrics(&self, _retention_hours: u32) -> Result<()> {
        // Not implemented - memory cleanup is automatic via ring buffer
        Ok(())
    }
}
