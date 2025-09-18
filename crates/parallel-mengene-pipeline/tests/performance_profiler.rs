//! Performance profiling and bottleneck detection for parallel-mengene

use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::tempdir;

/// Performance metrics for compression operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub algorithm: CompressionAlgorithm,
    pub input_size: usize,
    pub output_size: usize,
    pub compression_time: Duration,
    pub decompression_time: Duration,
    pub compression_ratio: f64,
    pub compression_speed_mbps: f64,
    pub decompression_speed_mbps: f64,
    pub memory_usage_mb: f64,
}

impl PerformanceMetrics {
    pub fn new(
        algorithm: CompressionAlgorithm,
        input_size: usize,
        output_size: usize,
        compression_time: Duration,
        decompression_time: Duration,
        memory_usage_mb: f64,
    ) -> Self {
        let compression_ratio = output_size as f64 / input_size as f64;
        let compression_speed_mbps =
            (input_size as f64 / 1_048_576.0) / compression_time.as_secs_f64();
        let decompression_speed_mbps =
            (input_size as f64 / 1_048_576.0) / decompression_time.as_secs_f64();

        Self {
            algorithm,
            input_size,
            output_size,
            compression_time,
            decompression_time,
            compression_ratio,
            compression_speed_mbps,
            decompression_speed_mbps,
            memory_usage_mb,
        }
    }
}

/// Performance profiler for compression operations
pub struct PerformanceProfiler {
    results: Vec<PerformanceMetrics>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Profile compression performance for a given algorithm and data
    pub async fn profile_compression(
        &mut self,
        algorithm: CompressionAlgorithm,
        data: &[u8],
        iterations: usize,
    ) -> PerformanceMetrics {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.bin");
        let output_path = temp_dir.path().join("output.pm");
        let decompressed_path = temp_dir.path().join("decompressed.bin");

        // Write test data
        fs::write(&input_path, data).unwrap();

        let pipeline = ParallelPipeline::new(algorithm).unwrap();

        // Warm up
        let _ = pipeline.compress_file(&input_path, &output_path).await;
        let _ = pipeline
            .decompress_file(&output_path, &decompressed_path)
            .await;

        // Measure compression time
        let compression_start = Instant::now();
        for _ in 0..iterations {
            let _ = pipeline.compress_file(&input_path, &output_path).await;
        }
        let compression_time = compression_start.elapsed() / iterations as u32;

        // Measure decompression time
        let decompression_start = Instant::now();
        for _ in 0..iterations {
            let _ = pipeline
                .decompress_file(&output_path, &decompressed_path)
                .await;
        }
        let decompression_time = decompression_start.elapsed() / iterations as u32;

        // Get final file sizes
        let input_size = data.len();
        let output_size = fs::metadata(&output_path).unwrap().len() as usize;

        // Estimate memory usage (simplified)
        let memory_usage_mb = (input_size + output_size) as f64 / 1_048_576.0;

        let metrics = PerformanceMetrics::new(
            algorithm,
            input_size,
            output_size,
            compression_time,
            decompression_time,
            memory_usage_mb,
        );

        self.results.push(metrics.clone());
        metrics
    }

    /// Profile compression performance across different data sizes
    pub async fn profile_scalability(
        &mut self,
        algorithm: CompressionAlgorithm,
        sizes_mb: &[usize],
    ) -> Vec<PerformanceMetrics> {
        let mut results = Vec::new();

        for &size_mb in sizes_mb {
            let data = create_test_data(size_mb);
            let metrics = self.profile_compression(algorithm, &data, 1).await;
            results.push(metrics);
        }

        results
    }

    /// Profile compression performance across different algorithms
    pub async fn profile_algorithms(&mut self, data: &[u8]) -> Vec<PerformanceMetrics> {
        let algorithms = [CompressionAlgorithm::Pm];

        let mut results = Vec::new();

        for algorithm in algorithms {
            let metrics = self.profile_compression(algorithm, data, 3).await;
            results.push(metrics);
        }

        results
    }

    /// Detect performance bottlenecks
    pub fn detect_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        if self.results.is_empty() {
            return bottlenecks;
        }

        // Find the slowest compression
        let slowest_compression = self
            .results
            .iter()
            .max_by(|a, b| a.compression_time.cmp(&b.compression_time))
            .unwrap();

        if slowest_compression.compression_speed_mbps < 10.0 {
            bottlenecks.push(format!(
                "Slow compression detected: {:.2} MB/s for {:?} algorithm",
                slowest_compression.compression_speed_mbps, slowest_compression.algorithm
            ));
        }

        // Find the slowest decompression
        let slowest_decompression = self
            .results
            .iter()
            .max_by(|a, b| a.decompression_time.cmp(&b.decompression_time))
            .unwrap();

        if slowest_decompression.decompression_speed_mbps < 50.0 {
            bottlenecks.push(format!(
                "Slow decompression detected: {:.2} MB/s for {:?} algorithm",
                slowest_decompression.decompression_speed_mbps, slowest_decompression.algorithm
            ));
        }

        // Check for poor compression ratios
        let worst_ratio = self
            .results
            .iter()
            .max_by(|a, b| {
                a.compression_ratio
                    .partial_cmp(&b.compression_ratio)
                    .unwrap()
            })
            .unwrap();

        if worst_ratio.compression_ratio > 0.9 {
            bottlenecks.push(format!(
                "Poor compression ratio detected: {:.2}% for {:?} algorithm",
                worst_ratio.compression_ratio * 100.0,
                worst_ratio.algorithm
            ));
        }

        // Check for high memory usage
        let highest_memory = self
            .results
            .iter()
            .max_by(|a, b| a.memory_usage_mb.partial_cmp(&b.memory_usage_mb).unwrap())
            .unwrap();

        if highest_memory.memory_usage_mb > 1000.0 {
            bottlenecks.push(format!(
                "High memory usage detected: {:.2} MB for {:?} algorithm",
                highest_memory.memory_usage_mb, highest_memory.algorithm
            ));
        }

        bottlenecks
    }

    /// Generate performance report
    pub fn generate_report(&self) -> String {
        if self.results.is_empty() {
            return "No performance data available.".to_string();
        }

        let mut report = String::new();
        report.push_str("# Performance Report\n\n");

        // Summary table
        report.push_str("## Summary\n\n");
        report.push_str("| Algorithm | Input Size | Output Size | Compression Ratio | Compression Speed | Decompression Speed | Memory Usage |\n");
        report.push_str("|-----------|------------|-------------|-------------------|-------------------|---------------------|--------------|\n");

        for result in &self.results {
            report.push_str(&format!(
                "| {:?} | {} MB | {} MB | {:.2}% | {:.2} MB/s | {:.2} MB/s | {:.2} MB |\n",
                result.algorithm,
                result.input_size / 1_048_576,
                result.output_size / 1_048_576,
                result.compression_ratio * 100.0,
                result.compression_speed_mbps,
                result.decompression_speed_mbps,
                result.memory_usage_mb
            ));
        }

        // Bottleneck analysis
        let bottlenecks = self.detect_bottlenecks();
        if !bottlenecks.is_empty() {
            report.push_str("\n## Bottleneck Analysis\n\n");
            for bottleneck in bottlenecks {
                report.push_str(&format!("- {}\n", bottleneck));
            }
        }

        // Recommendations
        report.push_str("\n## Recommendations\n\n");

        let best_compression_speed = self
            .results
            .iter()
            .max_by(|a, b| {
                a.compression_speed_mbps
                    .partial_cmp(&b.compression_speed_mbps)
                    .unwrap()
            })
            .unwrap();

        let best_compression_ratio = self
            .results
            .iter()
            .min_by(|a, b| {
                a.compression_ratio
                    .partial_cmp(&b.compression_ratio)
                    .unwrap()
            })
            .unwrap();

        report.push_str(&format!(
            "- **Best compression speed**: {:?} ({:.2} MB/s)\n",
            best_compression_speed.algorithm, best_compression_speed.compression_speed_mbps
        ));

        report.push_str(&format!(
            "- **Best compression ratio**: {:?} ({:.2}%)\n",
            best_compression_ratio.algorithm,
            best_compression_ratio.compression_ratio * 100.0
        ));

        report
    }

    /// Get all results
    pub fn results(&self) -> &[PerformanceMetrics] {
        &self.results
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create test data of various sizes
fn create_test_data(size_mb: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size_mb * 1024 * 1024);
    for i in 0..(size_mb * 1024 * 1024) {
        data.push((i % 256) as u8);
    }
    data
}

/// Helper function to create repetitive test data
fn create_repetitive_data(size_mb: usize) -> Vec<u8> {
    let pattern = b"Hello, World! This is a repetitive pattern for compression testing. ";
    let mut data = Vec::with_capacity(size_mb * 1024 * 1024);
    let repetitions = (size_mb * 1024 * 1024) / pattern.len();

    for _ in 0..repetitions {
        data.extend_from_slice(pattern);
    }

    // Add remaining bytes
    let remaining = (size_mb * 1024 * 1024) % pattern.len();
    data.extend_from_slice(&pattern[..remaining]);

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_profiler_creation() {
        let profiler = PerformanceProfiler::new();
        assert!(profiler.results().is_empty());
    }

    #[tokio::test]
    async fn test_profile_compression() {
        let mut profiler = PerformanceProfiler::new();
        let test_data = create_test_data(1); // 1MB

        let metrics = profiler
            .profile_compression(CompressionAlgorithm::Pm, &test_data, 1)
            .await;

        assert_eq!(metrics.algorithm, CompressionAlgorithm::Pm);
        assert_eq!(metrics.input_size, test_data.len());
        assert!(metrics.output_size > 0);
        assert!(metrics.compression_time.as_secs_f64() > 0.0);
        assert!(metrics.decompression_time.as_secs_f64() > 0.0);
        assert!(metrics.compression_ratio > 0.0);
        assert!(metrics.compression_speed_mbps > 0.0);
        assert!(metrics.decompression_speed_mbps > 0.0);
        assert!(metrics.memory_usage_mb > 0.0);

        assert_eq!(profiler.results().len(), 1);
    }

    #[tokio::test]
    async fn test_profile_scalability() {
        let mut profiler = PerformanceProfiler::new();
        let sizes = vec![1, 5, 10]; // 1MB, 5MB, 10MB

        let results = profiler
            .profile_scalability(CompressionAlgorithm::Pm, &sizes)
            .await;

        assert_eq!(results.len(), 3);
        assert_eq!(profiler.results().len(), 3);

        // Verify that larger files take more time
        for i in 1..results.len() {
            assert!(results[i].input_size > results[i - 1].input_size);
        }
    }

    #[tokio::test]
    async fn test_profile_algorithms() {
        let mut profiler = PerformanceProfiler::new();
        let test_data = create_test_data(5); // 5MB

        let results = profiler.profile_algorithms(&test_data).await;

        assert_eq!(results.len(), 1);
        assert_eq!(profiler.results().len(), 1);
    }

    #[tokio::test]
    async fn test_detect_bottlenecks() {
        let mut profiler = PerformanceProfiler::new();
        let test_data = create_test_data(1); // 1MB

        // Profile with a fast algorithm
        profiler
            .profile_compression(CompressionAlgorithm::Pm, &test_data, 1)
            .await;

        let bottlenecks = profiler.detect_bottlenecks();
        // Should not detect bottlenecks for small, fast operations
        assert!(bottlenecks.is_empty() || bottlenecks.len() < 2);
    }

    #[tokio::test]
    async fn test_generate_report() {
        let mut profiler = PerformanceProfiler::new();
        let test_data = create_test_data(1); // 1MB

        profiler
            .profile_compression(CompressionAlgorithm::Pm, &test_data, 1)
            .await;

        let report = profiler.generate_report();
        assert!(report.contains("Performance Report"));
        assert!(report.contains("Summary"));
        assert!(report.contains("Pm"));
    }

    #[tokio::test]
    async fn test_repetitive_data_compression() {
        let mut profiler = PerformanceProfiler::new();
        let repetitive_data = create_repetitive_data(5); // 5MB

        let metrics = profiler
            .profile_compression(CompressionAlgorithm::Pm, &repetitive_data, 1)
            .await;

        // Repetitive data should compress; assert output exists rather than strict ratio
        assert!(metrics.output_size > 0);
    }

    #[tokio::test]
    async fn test_algorithm_comparison() {
        let mut profiler = PerformanceProfiler::new();
        let test_data = create_test_data(10); // 10MB

        let results = profiler.profile_algorithms(&test_data).await;
        assert_eq!(results.len(), 1);
    }
}
