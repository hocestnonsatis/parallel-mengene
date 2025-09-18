//! Advanced benchmark runner for comprehensive performance testing

use crate::metrics_collector::{
    measure_operation, BenchmarkMetrics, MetricsCollector, OperationMetrics,
};
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::error::Result;
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::fs;
use tracing::{info, warn};

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub algorithms: Vec<CompressionAlgorithm>,
    pub test_files: Vec<PathBuf>,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub output_dir: PathBuf,
    pub enable_memory_tracking: bool,
    pub enable_cpu_tracking: bool,
    pub timeout_seconds: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            algorithms: vec![CompressionAlgorithm::Lz4],
            test_files: Vec::new(),
            iterations: 3,
            warmup_iterations: 1,
            output_dir: PathBuf::from("benchmark_results"),
            enable_memory_tracking: true,
            enable_cpu_tracking: true,
            timeout_seconds: 300, // 5 minutes
        }
    }
}

/// Benchmark results summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_tests: usize,
    pub successful_tests: usize,
    pub failed_tests: usize,
    pub average_compression_speed: f64,
    pub average_decompression_speed: f64,
    pub average_compression_ratio: f64,
    pub total_duration: Duration,
    pub peak_memory_usage: f64,
    pub peak_cpu_usage: f64,
}

/// Comprehensive benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    metrics_collector: MetricsCollector,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub async fn new(config: BenchmarkConfig) -> Result<Self> {
        let metrics_collector = MetricsCollector::new()?;

        // Create output directory
        tokio::fs::create_dir_all(&config.output_dir).await?;

        Ok(Self {
            config,
            metrics_collector,
        })
    }

    /// Run comprehensive benchmarks
    pub async fn run_benchmarks(&mut self) -> Result<Vec<BenchmarkMetrics>> {
        info!("🚀 Starting comprehensive benchmark suite");
        info!("📊 Algorithms: {:?}", self.config.algorithms);
        info!("📁 Test files: {}", self.config.test_files.len());
        info!("🔄 Iterations: {}", self.config.iterations);

        let start_time = Instant::now();
        let mut all_metrics = Vec::new();

        let algorithms = self.config.algorithms.clone();
        let test_files = self.config.test_files.clone();

        for algorithm in &algorithms {
            info!("🔧 Testing algorithm: {:?}", algorithm);

            for test_file in &test_files {
                if !test_file.exists() {
                    warn!("⚠️ Test file not found: {:?}", test_file);
                    continue;
                }

                info!("📄 Testing file: {:?}", test_file);

                let file_metrics = self.run_single_file_benchmark(algorithm, test_file).await?;
                all_metrics.extend(file_metrics);
            }
        }

        let total_duration = start_time.elapsed();
        info!("✅ Benchmark suite completed in {:?}", total_duration);
        info!("📊 Total tests: {}", all_metrics.len());

        Ok(all_metrics)
    }

    /// Run benchmark for a single file and algorithm
    async fn run_single_file_benchmark(
        &mut self,
        algorithm: &CompressionAlgorithm,
        test_file: &Path,
    ) -> Result<Vec<BenchmarkMetrics>> {
        let file_size = fs::metadata(test_file).await?.len();
        let mut file_metrics = Vec::new();

        // Run warmup iterations
        for i in 0..self.config.warmup_iterations {
            info!(
                "🔥 Warmup iteration {}/{}",
                i + 1,
                self.config.warmup_iterations
            );
            let _ = self
                .run_single_iteration(algorithm, test_file, file_size)
                .await;
        }

        // Run actual benchmark iterations
        for i in 0..self.config.iterations {
            info!(
                "🔄 Benchmark iteration {}/{}",
                i + 1,
                self.config.iterations
            );

            let metrics = self
                .run_single_iteration(algorithm, test_file, file_size)
                .await?;
            file_metrics.push(metrics);
        }

        Ok(file_metrics)
    }

    /// Run a single benchmark iteration
    async fn run_single_iteration(
        &mut self,
        algorithm: &CompressionAlgorithm,
        test_file: &Path,
        file_size: u64,
    ) -> Result<BenchmarkMetrics> {
        let temp_dir = self.config.output_dir.join("temp");
        fs::create_dir_all(&temp_dir).await?;

        let compressed_file = temp_dir.join(format!(
            "compressed_{}.pmz",
            test_file.file_name().unwrap().to_string_lossy()
        ));
        let decompressed_file = temp_dir.join(format!(
            "decompressed_{}",
            test_file.file_name().unwrap().to_string_lossy()
        ));

        // Create pipeline
        let _pipeline = ParallelPipeline::new(*algorithm)?;

        let mut system_metrics = Vec::new();

        // Start system monitoring
        if self.config.enable_memory_tracking || self.config.enable_cpu_tracking {
            self.metrics_collector.start_collection();
        }

        // Measure compression
        let compression_metrics = measure_operation("compression", file_size, || {
            // Collect system metrics during compression
            if self.config.enable_memory_tracking || self.config.enable_cpu_tracking {
                if let Ok(snapshot) = self.metrics_collector.collect_snapshot() {
                    system_metrics.push(snapshot);
                }
            }

            // Actually compress the data using the core compression library
            let input_data = std::fs::read(test_file)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            
            let compression_context = parallel_mengene_core::compression::CompressionContext::new(*algorithm, None);
            let compressed_data = compression_context.compress(&input_data)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            
            std::fs::write(&compressed_file, compressed_data)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            Ok(())
        })
        .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;

        // Get compressed file size
        let compressed_size = fs::metadata(&compressed_file).await?.len();
        let compression_ratio = (compressed_size as f64 / file_size as f64) * 100.0;

        // Measure decompression
        let decompression_metrics = measure_operation("decompression", compressed_size, || {
            // Collect system metrics during decompression
            if self.config.enable_memory_tracking || self.config.enable_cpu_tracking {
                if let Ok(snapshot) = self.metrics_collector.collect_snapshot() {
                    system_metrics.push(snapshot);
                }
            }

            // Actually decompress the data using the core compression library
            let compressed_data = std::fs::read(&compressed_file)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            
            let compression_context = parallel_mengene_core::compression::CompressionContext::new(*algorithm, None);
            let decompressed_data = compression_context.decompress(&compressed_data)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            
            std::fs::write(&decompressed_file, decompressed_data)
                .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
            Ok(())
        })
        .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;

        // Stop system monitoring
        if self.config.enable_memory_tracking || self.config.enable_cpu_tracking {
            system_metrics.extend(self.metrics_collector.stop_collection());
        }

        // Verify integrity
        let integrity_check = self
            .verify_file_integrity(test_file, &decompressed_file)
            .await?;

        // Cleanup temporary files
        let _ = fs::remove_file(&compressed_file).await;
        let _ = fs::remove_file(&decompressed_file).await;

        Ok(BenchmarkMetrics {
            algorithm: format!("{:?}", algorithm),
            file_name: test_file.file_name().unwrap().to_string_lossy().to_string(),
            file_size,
            compression_metrics: OperationMetrics {
                compression_ratio,
                ..compression_metrics
            },
            decompression_metrics: OperationMetrics {
                compression_ratio: 0.0, // Not applicable for decompression
                ..decompression_metrics
            },
            system_metrics,
            integrity_check,
            error_message: None,
        })
    }

    /// Verify file integrity after compression/decompression
    async fn verify_file_integrity(&self, original: &Path, decompressed: &Path) -> Result<bool> {
        let original_content = fs::read(original).await?;
        let decompressed_content = fs::read(decompressed).await?;

        Ok(original_content == decompressed_content)
    }

    /// Generate benchmark summary
    pub fn generate_summary(&self, metrics: &[BenchmarkMetrics]) -> BenchmarkSummary {
        let total_tests = metrics.len();
        let successful_tests = metrics.iter().filter(|m| m.error_message.is_none()).count();
        let failed_tests = total_tests - successful_tests;

        let compression_speeds: Vec<f64> = metrics
            .iter()
            .map(|m| m.compression_metrics.throughput_mbps)
            .collect();
        let decompression_speeds: Vec<f64> = metrics
            .iter()
            .map(|m| m.decompression_metrics.throughput_mbps)
            .collect();
        let compression_ratios: Vec<f64> = metrics
            .iter()
            .map(|m| m.compression_metrics.compression_ratio)
            .collect();

        let average_compression_speed = if !compression_speeds.is_empty() {
            compression_speeds.iter().sum::<f64>() / compression_speeds.len() as f64
        } else {
            0.0
        };

        let average_decompression_speed = if !decompression_speeds.is_empty() {
            decompression_speeds.iter().sum::<f64>() / decompression_speeds.len() as f64
        } else {
            0.0
        };

        let average_compression_ratio = if !compression_ratios.is_empty() {
            compression_ratios.iter().sum::<f64>() / compression_ratios.len() as f64
        } else {
            0.0
        };

        let peak_memory_usage = metrics
            .iter()
            .map(|m| m.compression_metrics.memory_peak_mb)
            .fold(0.0, f64::max);

        let peak_cpu_usage = metrics
            .iter()
            .map(|m| m.compression_metrics.cpu_usage_percent)
            .fold(0.0, f64::max);

        BenchmarkSummary {
            total_tests,
            successful_tests,
            failed_tests,
            average_compression_speed,
            average_decompression_speed,
            average_compression_ratio,
            total_duration: Duration::from_secs(0), // Will be set by caller
            peak_memory_usage,
            peak_cpu_usage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_benchmark_runner_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = BenchmarkConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let runner = BenchmarkRunner::new(config).await;
        assert!(runner.is_ok());
    }

    #[test]
    fn test_benchmark_summary_generation() {
        let metrics = vec![BenchmarkMetrics {
            algorithm: "zstd".to_string(),
            file_name: "test.bin".to_string(),
            file_size: 1024,
            compression_metrics: OperationMetrics {
                operation_type: "compression".to_string(),
                file_size: 1024,
                duration: Duration::from_millis(100),
                throughput_mbps: 10.0,
                compression_ratio: 50.0,
                memory_peak_mb: 100.0,
                cpu_usage_percent: 50.0,
                timestamp: Instant::now(),
            },
            decompression_metrics: OperationMetrics {
                operation_type: "decompression".to_string(),
                file_size: 512,
                duration: Duration::from_millis(50),
                throughput_mbps: 20.0,
                compression_ratio: 0.0,
                memory_peak_mb: 50.0,
                cpu_usage_percent: 25.0,
                timestamp: Instant::now(),
            },
            system_metrics: vec![],
            integrity_check: true,
            error_message: None,
        }];

        let runner = BenchmarkRunner {
            config: BenchmarkConfig::default(),
            metrics_collector: MetricsCollector::new().unwrap(),
        };

        let summary = runner.generate_summary(&metrics);

        assert_eq!(summary.total_tests, 1);
        assert_eq!(summary.successful_tests, 1);
        assert_eq!(summary.failed_tests, 0);
        assert_eq!(summary.average_compression_speed, 10.0);
        assert_eq!(summary.average_decompression_speed, 20.0);
        assert_eq!(summary.average_compression_ratio, 50.0);
        assert_eq!(summary.peak_memory_usage, 100.0);
        assert_eq!(summary.peak_cpu_usage, 50.0);
    }
}
