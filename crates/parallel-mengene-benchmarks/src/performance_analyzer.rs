//! Advanced performance analysis and statistical evaluation

use crate::metrics_collector::BenchmarkMetrics;
use parallel_mengene_core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statistical analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    pub mean: f64,
    pub median: f64,
    pub std_deviation: f64,
    pub min: f64,
    pub max: f64,
    pub percentile_25: f64,
    pub percentile_75: f64,
    pub percentile_95: f64,
    pub percentile_99: f64,
}

/// Performance comparison between algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmComparison {
    pub algorithm: String,
    pub compression_speed_stats: StatisticalAnalysis,
    pub decompression_speed_stats: StatisticalAnalysis,
    pub compression_ratio_stats: StatisticalAnalysis,
    pub memory_usage_stats: StatisticalAnalysis,
    pub cpu_usage_stats: StatisticalAnalysis,
    pub reliability_score: f64, // Based on success rate and integrity checks
}

/// Performance analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub algorithm_comparisons: Vec<AlgorithmComparison>,
    pub best_overall_algorithm: String,
    pub best_compression_speed: String,
    pub best_decompression_speed: String,
    pub best_compression_ratio: String,
    pub best_memory_efficiency: String,
    pub scaling_analysis: HashMap<String, ScalingAnalysis>,
    pub recommendations: Vec<String>,
}

/// File size scaling analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingAnalysis {
    pub file_size_category: String,
    pub algorithms_performance: HashMap<String, f64>,
    pub best_algorithm_for_size: String,
    pub performance_trend: String, // "improving", "degrading", "stable"
}

/// Advanced performance analyzer
pub struct PerformanceAnalyzer;

impl PerformanceAnalyzer {
    /// Analyze benchmark metrics and generate comprehensive performance report
    pub fn analyze_performance(metrics: &[BenchmarkMetrics]) -> Result<PerformanceAnalysis> {
        let mut algorithm_groups: HashMap<String, Vec<&BenchmarkMetrics>> = HashMap::new();
        
        // Group metrics by algorithm
        for metric in metrics {
            algorithm_groups.entry(metric.algorithm.clone())
                .or_insert_with(Vec::new)
                .push(metric);
        }
        
        // Analyze each algorithm
        let mut algorithm_comparisons = Vec::new();
        for (algorithm, algorithm_metrics) in &algorithm_groups {
            let comparison = Self::analyze_algorithm(algorithm, algorithm_metrics)?;
            algorithm_comparisons.push(comparison);
        }
        
        // Sort by overall performance score
        algorithm_comparisons.sort_by(|a, b| {
            let score_a = Self::calculate_overall_score(a);
            let score_b = Self::calculate_overall_score(b);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Determine best algorithms for different criteria
        let best_overall = algorithm_comparisons.first()
            .map(|a| a.algorithm.clone())
            .unwrap_or_default();
        
        let best_compression_speed = Self::find_best_for_criterion(
            &algorithm_comparisons,
            |a| a.compression_speed_stats.mean
        );
        
        let best_decompression_speed = Self::find_best_for_criterion(
            &algorithm_comparisons,
            |a| a.decompression_speed_stats.mean
        );
        
        let best_compression_ratio = Self::find_best_for_criterion(
            &algorithm_comparisons,
            |a| 100.0 - a.compression_ratio_stats.mean // Lower ratio is better
        );
        
        let best_memory_efficiency = Self::find_best_for_criterion(
            &algorithm_comparisons,
            |a| 1000.0 - a.memory_usage_stats.mean // Lower memory usage is better
        );
        
        // Analyze scaling performance
        let scaling_analysis = Self::analyze_scaling_performance(metrics)?;
        
        // Generate recommendations
        let recommendations = Self::generate_recommendations(&algorithm_comparisons);
        
        Ok(PerformanceAnalysis {
            algorithm_comparisons,
            best_overall_algorithm: best_overall,
            best_compression_speed,
            best_decompression_speed,
            best_compression_ratio,
            best_memory_efficiency,
            scaling_analysis,
            recommendations,
        })
    }
    
    /// Analyze performance for a single algorithm
    fn analyze_algorithm(
        algorithm: &str,
        metrics: &[&BenchmarkMetrics]
    ) -> Result<AlgorithmComparison> {
        let compression_speeds: Vec<f64> = metrics.iter()
            .map(|m| m.compression_metrics.throughput_mbps)
            .collect();
        
        let decompression_speeds: Vec<f64> = metrics.iter()
            .map(|m| m.decompression_metrics.throughput_mbps)
            .collect();
        
        let compression_ratios: Vec<f64> = metrics.iter()
            .map(|m| m.compression_metrics.compression_ratio)
            .collect();
        
        let memory_usages: Vec<f64> = metrics.iter()
            .map(|m| m.compression_metrics.memory_peak_mb)
            .collect();
        
        let cpu_usages: Vec<f64> = metrics.iter()
            .map(|m| m.compression_metrics.cpu_usage_percent)
            .collect();
        
        let reliability_score = Self::calculate_reliability_score(metrics);
        
        Ok(AlgorithmComparison {
            algorithm: algorithm.to_string(),
            compression_speed_stats: Self::calculate_statistics(&compression_speeds),
            decompression_speed_stats: Self::calculate_statistics(&decompression_speeds),
            compression_ratio_stats: Self::calculate_statistics(&compression_ratios),
            memory_usage_stats: Self::calculate_statistics(&memory_usages),
            cpu_usage_stats: Self::calculate_statistics(&cpu_usages),
            reliability_score,
        })
    }
    
    /// Calculate statistical measures for a dataset
    fn calculate_statistics(data: &[f64]) -> StatisticalAnalysis {
        if data.is_empty() {
            return StatisticalAnalysis {
                mean: 0.0,
                median: 0.0,
                std_deviation: 0.0,
                min: 0.0,
                max: 0.0,
                percentile_25: 0.0,
                percentile_75: 0.0,
                percentile_95: 0.0,
                percentile_99: 0.0,
            };
        }
        
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let n = sorted_data.len();
        let mean = sorted_data.iter().sum::<f64>() / n as f64;
        
        let variance = sorted_data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / n as f64;
        let std_deviation = variance.sqrt();
        
        let median = if n % 2 == 0 {
            (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
        } else {
            sorted_data[n / 2]
        };
        
        let percentile_25 = sorted_data[(n as f64 * 0.25) as usize];
        let percentile_75 = sorted_data[(n as f64 * 0.75) as usize];
        let percentile_95 = sorted_data[(n as f64 * 0.95) as usize];
        let percentile_99 = sorted_data[(n as f64 * 0.99) as usize];
        
        StatisticalAnalysis {
            mean,
            median,
            std_deviation,
            min: sorted_data[0],
            max: sorted_data[n - 1],
            percentile_25,
            percentile_75,
            percentile_95,
            percentile_99,
        }
    }
    
    /// Calculate reliability score based on success rate and integrity checks
    fn calculate_reliability_score(metrics: &[&BenchmarkMetrics]) -> f64 {
        if metrics.is_empty() {
            return 0.0;
        }
        
        let total_tests = metrics.len();
        let successful_tests = metrics.iter()
            .filter(|m| m.error_message.is_none() && m.integrity_check)
            .count();
        
        (successful_tests as f64 / total_tests as f64) * 100.0
    }
    
    /// Calculate overall performance score for an algorithm
    fn calculate_overall_score(comparison: &AlgorithmComparison) -> f64 {
        let speed_score = comparison.compression_speed_stats.mean / 1000.0; // Normalize
        let decomp_score = comparison.decompression_speed_stats.mean / 1000.0;
        let ratio_score = (100.0 - comparison.compression_ratio_stats.mean) / 100.0; // Lower is better
        let memory_score = (1000.0 - comparison.memory_usage_stats.mean) / 1000.0; // Lower is better
        let reliability_score = comparison.reliability_score / 100.0;
        
        // Weighted average
        speed_score * 0.3 + decomp_score * 0.2 + ratio_score * 0.2 + 
        memory_score * 0.15 + reliability_score * 0.15
    }
    
    /// Find best algorithm for a specific criterion
    fn find_best_for_criterion<F>(comparisons: &[AlgorithmComparison], criterion: F) -> String
    where
        F: Fn(&AlgorithmComparison) -> f64,
    {
        comparisons.iter()
            .max_by(|a, b| criterion(a).partial_cmp(&criterion(b)).unwrap_or(std::cmp::Ordering::Equal))
            .map(|a| a.algorithm.clone())
            .unwrap_or_default()
    }
    
    /// Analyze scaling performance across different file sizes
    fn analyze_scaling_performance(metrics: &[BenchmarkMetrics]) -> Result<HashMap<String, ScalingAnalysis>> {
        let mut size_groups: HashMap<String, Vec<&BenchmarkMetrics>> = HashMap::new();
        
        // Group by file size categories
        for metric in metrics {
            let size_category = Self::categorize_file_size(metric.file_size);
            size_groups.entry(size_category)
                .or_insert_with(Vec::new)
                .push(metric);
        }
        
        let mut scaling_analysis = HashMap::new();
        
        for (size_category, size_metrics) in size_groups {
            let mut algorithm_performance: HashMap<String, Vec<f64>> = HashMap::new();
            
            // Group by algorithm within size category
            for metric in size_metrics {
                let performance = metric.compression_metrics.throughput_mbps;
                algorithm_performance.entry(metric.algorithm.clone())
                    .or_insert_with(Vec::new)
                    .push(performance);
            }
            
            // Calculate average performance for each algorithm
            let mut algorithms_performance: HashMap<String, f64> = HashMap::new();
            for (algorithm, performances) in algorithm_performance {
                let avg_performance = performances.iter().sum::<f64>() / performances.len() as f64;
                algorithms_performance.insert(algorithm, avg_performance);
            }
            
            // Find best algorithm for this size
            let best_algorithm = algorithms_performance.iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(name, _)| name.clone())
                .unwrap_or_default();
            
            // Analyze performance trend (simplified)
            let performance_trend = Self::analyze_performance_trend(&algorithms_performance);
            
            scaling_analysis.insert(size_category.clone(), ScalingAnalysis {
                file_size_category: size_category,
                algorithms_performance,
                best_algorithm_for_size: best_algorithm,
                performance_trend,
            });
        }
        
        Ok(scaling_analysis)
    }
    
    /// Categorize file size into meaningful categories
    fn categorize_file_size(size: u64) -> String {
        const SMALL_LIMIT: u64 = 1024 * 1024; // 1MB
        const MEDIUM_LIMIT: u64 = 10 * 1024 * 1024; // 10MB
        const LARGE_LIMIT: u64 = 100 * 1024 * 1024; // 100MB
        
        if size <= SMALL_LIMIT {
            "Small (< 1MB)".to_string()
        } else if size <= MEDIUM_LIMIT {
            "Medium (1-10MB)".to_string()
        } else if size <= LARGE_LIMIT {
            "Large (10-100MB)".to_string()
        } else {
            "Very Large (> 100MB)".to_string()
        }
    }
    
    /// Analyze performance trend (simplified implementation)
    fn analyze_performance_trend(performances: &HashMap<String, f64>) -> String {
        if performances.len() < 2 {
            return "stable".to_string();
        }
        
        let values: Vec<f64> = performances.values().cloned().collect();
        let mut increasing = 0;
        let mut decreasing = 0;
        
        for i in 1..values.len() {
            if values[i] > values[i-1] {
                increasing += 1;
            } else if values[i] < values[i-1] {
                decreasing += 1;
            }
        }
        
        if increasing > decreasing {
            "improving".to_string()
        } else if decreasing > increasing {
            "degrading".to_string()
        } else {
            "stable".to_string()
        }
    }
    
    /// Generate performance recommendations
    fn generate_recommendations(comparisons: &[AlgorithmComparison]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if comparisons.is_empty() {
            return vec!["No benchmark data available for recommendations".to_string()];
        }
        
        let best_overall = &comparisons[0];
        
        recommendations.push(format!(
            "Best overall algorithm: {} (score: {:.2})",
            best_overall.algorithm,
            Self::calculate_overall_score(best_overall)
        ));
        
        // Speed recommendations
        let fastest_compression = comparisons.iter()
            .max_by(|a, b| a.compression_speed_stats.mean.partial_cmp(&b.compression_speed_stats.mean)
                .unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        
        recommendations.push(format!(
            "For maximum compression speed: {} ({:.1} MB/s)",
            fastest_compression.algorithm,
            fastest_compression.compression_speed_stats.mean
        ));
        
        // Memory recommendations
        let most_memory_efficient = comparisons.iter()
            .min_by(|a, b| a.memory_usage_stats.mean.partial_cmp(&b.memory_usage_stats.mean)
                .unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        
        recommendations.push(format!(
            "For memory efficiency: {} ({:.1} MB peak)",
            most_memory_efficient.algorithm,
            most_memory_efficient.memory_usage_stats.mean
        ));
        
        // Reliability recommendations
        let most_reliable = comparisons.iter()
            .max_by(|a, b| a.reliability_score.partial_cmp(&b.reliability_score)
                .unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        
        recommendations.push(format!(
            "For reliability: {} ({:.1}% success rate)",
            most_reliable.algorithm,
            most_reliable.reliability_score
        ));
        
        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    
    fn create_test_metrics() -> Vec<BenchmarkMetrics> {
        vec![
            BenchmarkMetrics {
                algorithm: "zstd".to_string(),
                file_name: "test1.bin".to_string(),
                file_size: 1024 * 1024, // 1MB
                compression_metrics: crate::metrics_collector::OperationMetrics {
                    operation_type: "compression".to_string(),
                    file_size: 1024 * 1024,
                    duration: Duration::from_millis(100),
                    throughput_mbps: 10.0,
                    compression_ratio: 50.0,
                    memory_peak_mb: 100.0,
                    cpu_usage_percent: 50.0,
                    timestamp: Instant::now(),
                },
                decompression_metrics: crate::metrics_collector::OperationMetrics {
                    operation_type: "decompression".to_string(),
                    file_size: 512 * 1024,
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
            }
        ]
    }
    
    #[test]
    fn test_statistical_analysis() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = PerformanceAnalyzer::calculate_statistics(&data);
        
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert!(stats.std_deviation > 0.0);
    }
    
    #[test]
    fn test_performance_analysis() {
        let metrics = create_test_metrics();
        let analysis = PerformanceAnalyzer::analyze_performance(&metrics).unwrap();
        
        assert!(!analysis.algorithm_comparisons.is_empty());
        assert!(!analysis.recommendations.is_empty());
        assert!(!analysis.best_overall_algorithm.is_empty());
    }
    
    #[test]
    fn test_file_size_categorization() {
        assert_eq!(PerformanceAnalyzer::categorize_file_size(500 * 1024), "Small (< 1MB)");
        assert_eq!(PerformanceAnalyzer::categorize_file_size(5 * 1024 * 1024), "Medium (1-10MB)");
        assert_eq!(PerformanceAnalyzer::categorize_file_size(50 * 1024 * 1024), "Large (10-100MB)");
        assert_eq!(PerformanceAnalyzer::categorize_file_size(200 * 1024 * 1024), "Very Large (> 100MB)");
    }
}
