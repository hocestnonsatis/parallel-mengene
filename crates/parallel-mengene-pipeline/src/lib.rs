//! High-performance parallel compression pipeline for parallel-mengene
//!
//! This crate implements the parallel compression approach:
//! - CPU Pipeline: Multi-threaded compression with intelligent chunking
//! - Parallel Processing: Optimized workload distribution

pub mod coordinator;
pub mod cpu_pipeline;
pub mod memory_mapping;
pub mod memory_monitor;
pub mod parallel_pipeline;

// Re-export core error types for convenience
pub use parallel_mengene_core::error::{Error, Result};
