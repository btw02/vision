//! GPU metrics collector
//!
//! GPU monitoring is not currently implemented.
//! This module exists as a placeholder for future GPU support.

use anyhow::Result;
use crate::models::metrics::GpuMetrics;
use super::Collector;

/// GPU metrics collector (not implemented)
///
/// GPU monitoring requires platform-specific libraries:
/// - NVIDIA: nvml-wrapper crate with CUDA toolkit
/// - AMD: ROCm libraries
/// - Intel: Level Zero or similar
pub struct GpuCollector {
    _phantom: std::marker::PhantomData<()>,
}

impl GpuCollector {
    /// Create a new GPU collector
    ///
    /// Currently returns a non-functional collector as GPU monitoring
    /// is not implemented.
    pub fn new() -> Result<Self> {
        Ok(Self {
            _phantom: std::marker::PhantomData,
        })
    }
    
    /// Get the current metrics
    ///
    /// Always returns None as GPU monitoring is not implemented.
    pub fn get_metrics(&self) -> Option<GpuMetrics> {
        None
    }
}

impl Collector for GpuCollector {
    fn collect(&mut self) -> Result<()> {
        // GPU monitoring not implemented
        Ok(())
    }
}

