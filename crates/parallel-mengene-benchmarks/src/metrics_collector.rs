//! Real-time metrics collection for benchmarking

use parallel_mengene_core::error::Result;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

/// Performance metrics for a single operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    pub operation_type: String,
    pub file_size: u64,
    pub duration: Duration,
    pub throughput_mbps: f64,
    pub compression_ratio: f64,
    pub memory_peak_mb: f64,
    pub cpu_usage_percent: f64,
    #[serde(skip, default = "Instant::now")]
    pub timestamp: Instant,
}

/// System resource usage during operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub memory_available_mb: f64,
    pub memory_total_mb: f64,
    #[serde(skip, default = "Instant::now")]
    pub timestamp: Instant,
}

/// Comprehensive benchmark metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub algorithm: String,
    pub file_name: String,
    pub file_size: u64,
    pub compression_metrics: OperationMetrics,
    pub decompression_metrics: OperationMetrics,
    pub system_metrics: Vec<SystemMetrics>,
    pub integrity_check: bool,
    pub error_message: Option<String>,
}

/// Real-time metrics collector
pub struct MetricsCollector {
    system: System,
    #[allow(dead_code)]
    process_id: Pid,
    start_time: Instant,
    metrics: Vec<SystemMetrics>,
    #[allow(dead_code)]
    collection_interval: Duration,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();

        let process_id = std::process::id();

        Ok(Self {
            system,
            process_id: Pid::from_u32(process_id),
            start_time: Instant::now(),
            metrics: Vec::new(),
            collection_interval: Duration::from_millis(100), // Collect every 100ms
        })
    }

    /// Start collecting metrics
    pub fn start_collection(&mut self) {
        self.start_time = Instant::now();
        self.metrics.clear();
    }

    /// Collect current system metrics
    pub fn collect_snapshot(&mut self) -> Result<SystemMetrics> {
        self.system.refresh_all();

        let cpu_usage = self.system.global_cpu_usage();
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let available_memory = self.system.available_memory();

        let metrics = SystemMetrics {
            cpu_usage_percent: cpu_usage as f64,
            memory_usage_mb: used_memory as f64 / 1024.0,
            memory_available_mb: available_memory as f64 / 1024.0,
            memory_total_mb: total_memory as f64 / 1024.0,
            timestamp: Instant::now(),
        };

        self.metrics.push(metrics.clone());
        Ok(metrics)
    }

    /// Stop collecting metrics and return summary
    pub fn stop_collection(&mut self) -> Vec<SystemMetrics> {
        self.metrics.clone()
    }

    /// Get average metrics during collection period
    pub fn get_average_metrics(&self) -> SystemMetrics {
        if self.metrics.is_empty() {
            return SystemMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0.0,
                memory_available_mb: 0.0,
                memory_total_mb: 0.0,
                timestamp: Instant::now(),
            };
        }

        let cpu_avg = self
            .metrics
            .iter()
            .map(|m| m.cpu_usage_percent)
            .sum::<f64>()
            / self.metrics.len() as f64;
        let memory_avg =
            self.metrics.iter().map(|m| m.memory_usage_mb).sum::<f64>() / self.metrics.len() as f64;
        let memory_available_avg = self
            .metrics
            .iter()
            .map(|m| m.memory_available_mb)
            .sum::<f64>()
            / self.metrics.len() as f64;
        let memory_total_avg =
            self.metrics.iter().map(|m| m.memory_total_mb).sum::<f64>() / self.metrics.len() as f64;

        SystemMetrics {
            cpu_usage_percent: cpu_avg,
            memory_usage_mb: memory_avg,
            memory_available_mb: memory_available_avg,
            memory_total_mb: memory_total_avg,
            timestamp: Instant::now(),
        }
    }

    /// Get peak memory usage during collection
    pub fn get_peak_memory(&self) -> f64 {
        self.metrics
            .iter()
            .map(|m| m.memory_usage_mb)
            .fold(0.0, f64::max)
    }

    /// Get peak CPU usage during collection
    pub fn get_peak_cpu(&self) -> f64 {
        self.metrics
            .iter()
            .map(|m| m.cpu_usage_percent)
            .fold(0.0, f64::max)
    }
}

/// Helper function to measure operation performance
pub fn measure_operation<F, R>(
    operation_type: &str,
    file_size: u64,
    operation: F,
) -> Result<OperationMetrics>
where
    F: FnOnce() -> Result<R>,
{
    let start_time = Instant::now();
    let mut collector = MetricsCollector::new()?;

    // Start collecting metrics
    collector.start_collection();

    // Perform operation
    let _result = operation();

    // Stop collecting and get metrics
    let _system_metrics = collector.stop_collection();
    let duration = start_time.elapsed();

    // Calculate metrics
    let throughput_mbps = if duration.as_secs_f64() > 0.0 {
        (file_size as f64 / 1_048_576.0) / duration.as_secs_f64()
    } else {
        0.0
    };

    let _avg_metrics = collector.get_average_metrics();
    let peak_memory = collector.get_peak_memory();
    let peak_cpu = collector.get_peak_cpu();

    Ok(OperationMetrics {
        operation_type: operation_type.to_string(),
        file_size,
        duration,
        throughput_mbps,
        compression_ratio: 0.0, // Will be set by caller
        memory_peak_mb: peak_memory,
        cpu_usage_percent: peak_cpu,
        timestamp: start_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new().unwrap();
        collector.start_collection();

        // Simulate some work
        thread::sleep(Duration::from_millis(50));
        collector.collect_snapshot().unwrap();

        thread::sleep(Duration::from_millis(50));
        let metrics = collector.stop_collection();

        assert!(!metrics.is_empty());
        assert!(metrics[0].cpu_usage_percent >= 0.0);
        assert!(metrics[0].memory_usage_mb >= 0.0);
    }

    #[test]
    fn test_measure_operation() {
        let result = measure_operation("test", 1024, || {
            thread::sleep(Duration::from_millis(10));
            Ok(())
        })
        .unwrap();

        assert_eq!(result.operation_type, "test");
        assert_eq!(result.file_size, 1024);
        assert!(result.duration.as_millis() >= 10);
        assert!(result.throughput_mbps >= 0.0);
    }
}
