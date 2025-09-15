//! Advanced benchmarking tools for parallel-mengene
//! 
//! This crate provides comprehensive benchmarking capabilities including:
//! - Performance analysis and comparison
//! - Memory usage tracking
//! - Real-time metrics collection
//! - Statistical analysis and reporting

pub mod benchmark_runner;
pub mod metrics_collector;
pub mod performance_analyzer;
pub mod report_generator;
pub mod test_data_generator;

// Re-export core types
pub use benchmark_runner::BenchmarkRunner;
pub use metrics_collector::MetricsCollector;
pub use performance_analyzer::PerformanceAnalyzer;
pub use report_generator::ReportGenerator;
pub use test_data_generator::TestDataGenerator;

// Re-export core error types
pub use parallel_mengene_core::error::{Error, Result};
