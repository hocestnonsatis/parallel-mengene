//! Comprehensive report generation for benchmark results

use crate::benchmark_runner::BenchmarkSummary;
use crate::metrics_collector::BenchmarkMetrics;
use crate::performance_analyzer::PerformanceAnalysis;
use parallel_mengene_core::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    pub output_format: ReportFormat,
    pub include_charts: bool,
    pub include_raw_data: bool,
    pub include_system_info: bool,
    pub template_style: ReportStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Json,
    Html,
    Markdown,
    Csv,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportStyle {
    Professional,
    Technical,
    Summary,
}

/// Comprehensive benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub metadata: ReportMetadata,
    pub summary: BenchmarkSummary,
    pub performance_analysis: PerformanceAnalysis,
    pub raw_metrics: Vec<BenchmarkMetrics>,
    pub system_info: Option<SystemInfo>,
}

/// Report metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub generated_at: String,
    pub version: String,
    pub benchmark_duration: Duration,
    pub total_files_tested: usize,
    pub total_algorithms_tested: usize,
    pub report_format: String,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu_cores: usize,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub cpu_model: Option<String>,
    pub rust_version: String,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            output_format: ReportFormat::All,
            include_charts: true,
            include_raw_data: true,
            include_system_info: true,
            template_style: ReportStyle::Professional,
        }
    }
}

/// Advanced report generator
pub struct ReportGenerator {
    config: ReportConfig,
}

impl ReportGenerator {
    /// Create a new report generator
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive benchmark report
    pub fn generate_report(
        &self,
        metrics: Vec<BenchmarkMetrics>,
        performance_analysis: PerformanceAnalysis,
        benchmark_duration: Duration,
        output_dir: &Path,
    ) -> Result<()> {
        let summary = self.generate_summary(&metrics);
        let metadata = self.generate_metadata(&metrics, benchmark_duration);
        let system_info = if self.config.include_system_info {
            Some(Self::collect_system_info())
        } else {
            None
        };

        let report = BenchmarkReport {
            metadata,
            summary,
            performance_analysis,
            raw_metrics: if self.config.include_raw_data {
                metrics
            } else {
                vec![]
            },
            system_info,
        };

        // Generate reports in requested formats
        match self.config.output_format {
            ReportFormat::Json => self.generate_json_report(&report, output_dir)?,
            ReportFormat::Html => self.generate_html_report(&report, output_dir)?,
            ReportFormat::Markdown => self.generate_markdown_report(&report, output_dir)?,
            ReportFormat::Csv => self.generate_csv_report(&report, output_dir)?,
            ReportFormat::All => {
                self.generate_json_report(&report, output_dir)?;
                self.generate_html_report(&report, output_dir)?;
                self.generate_markdown_report(&report, output_dir)?;
                self.generate_csv_report(&report, output_dir)?;
            }
        }

        Ok(())
    }

    /// Generate summary statistics
    fn generate_summary(&self, metrics: &[BenchmarkMetrics]) -> BenchmarkSummary {
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

        BenchmarkSummary {
            total_tests,
            successful_tests,
            failed_tests,
            average_compression_speed: if !compression_speeds.is_empty() {
                compression_speeds.iter().sum::<f64>() / compression_speeds.len() as f64
            } else {
                0.0
            },
            average_decompression_speed: if !decompression_speeds.is_empty() {
                decompression_speeds.iter().sum::<f64>() / decompression_speeds.len() as f64
            } else {
                0.0
            },
            average_compression_ratio: if !compression_ratios.is_empty() {
                compression_ratios.iter().sum::<f64>() / compression_ratios.len() as f64
            } else {
                0.0
            },
            total_duration: Duration::from_secs(0), // Will be set by caller
            peak_memory_usage: metrics
                .iter()
                .map(|m| m.compression_metrics.memory_peak_mb)
                .fold(0.0, f64::max),
            peak_cpu_usage: metrics
                .iter()
                .map(|m| m.compression_metrics.cpu_usage_percent)
                .fold(0.0, f64::max),
        }
    }

    /// Generate report metadata
    fn generate_metadata(
        &self,
        metrics: &[BenchmarkMetrics],
        duration: Duration,
    ) -> ReportMetadata {
        let algorithms: std::collections::HashSet<String> =
            metrics.iter().map(|m| m.algorithm.clone()).collect();

        ReportMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            benchmark_duration: duration,
            total_files_tested: metrics.len(),
            total_algorithms_tested: algorithms.len(),
            report_format: format!("{:?}", self.config.output_format),
        }
    }

    /// Collect system information
    fn collect_system_info() -> SystemInfo {
        let total_memory =
            sysinfo::System::new_all().total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_memory =
            sysinfo::System::new_all().available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

        SystemInfo {
            os: std::env::consts::OS.to_string(),
            cpu_cores: num_cpus::get(),
            total_memory_gb: total_memory,
            available_memory_gb: available_memory,
            cpu_model: None, // Would need additional system calls
            rust_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Generate JSON report
    fn generate_json_report(&self, report: &BenchmarkReport, output_dir: &Path) -> Result<()> {
        let json_content = serde_json::to_string_pretty(report)
            .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;
        let json_path = output_dir.join("benchmark_report.json");
        fs::write(json_path, json_content)?;
        Ok(())
    }

    /// Generate HTML report
    fn generate_html_report(&self, report: &BenchmarkReport, output_dir: &Path) -> Result<()> {
        let html_content = self.generate_html_content(report);
        let html_path = output_dir.join("benchmark_report.html");
        fs::write(html_path, html_content)?;
        Ok(())
    }

    /// Generate Markdown report
    fn generate_markdown_report(&self, report: &BenchmarkReport, output_dir: &Path) -> Result<()> {
        let markdown_content = self.generate_markdown_content(report);
        let markdown_path = output_dir.join("benchmark_report.md");
        fs::write(markdown_path, markdown_content)?;
        Ok(())
    }

    /// Generate CSV report
    fn generate_csv_report(&self, report: &BenchmarkReport, output_dir: &Path) -> Result<()> {
        let mut csv_content = String::new();
        csv_content.push_str("Algorithm,File,File_Size,Compression_Speed_MBps,Decompression_Speed_MBps,Compression_Ratio,Memory_Peak_MB,CPU_Usage_Percent,Integrity_Check,Error\n");

        for metric in &report.raw_metrics {
            csv_content.push_str(&format!(
                "{},{},{},{:.2},{:.2},{:.4},{:.2},{:.2},{},{}\n",
                metric.algorithm,
                metric.file_name,
                metric.file_size,
                metric.compression_metrics.throughput_mbps,
                metric.decompression_metrics.throughput_mbps,
                metric.compression_metrics.compression_ratio,
                metric.compression_metrics.memory_peak_mb,
                metric.compression_metrics.cpu_usage_percent,
                metric.integrity_check,
                metric.error_message.as_deref().unwrap_or("")
            ));
        }

        let csv_path = output_dir.join("benchmark_results.csv");
        fs::write(csv_path, csv_content)?;
        Ok(())
    }

    /// Generate HTML content
    fn generate_html_content(&self, report: &BenchmarkReport) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Parallel-Mengene Benchmark Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f5f5f5; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .header {{ text-align: center; border-bottom: 2px solid #333; padding-bottom: 20px; margin-bottom: 30px; }}
        .summary {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 30px; }}
        .summary-card {{ background: #f8f9fa; padding: 20px; border-radius: 8px; text-align: center; }}
        .summary-card h3 {{ margin: 0 0 10px 0; color: #333; }}
        .summary-card .value {{ font-size: 2em; font-weight: bold; color: #007bff; }}
        .algorithm-comparison {{ margin-bottom: 30px; }}
        .algorithm-card {{ background: #f8f9fa; padding: 20px; margin: 10px 0; border-radius: 8px; border-left: 4px solid #007bff; }}
        .recommendations {{ background: #e7f3ff; padding: 20px; border-radius: 8px; border-left: 4px solid #007bff; }}
        .recommendations h3 {{ margin-top: 0; color: #0056b3; }}
        .recommendations ul {{ margin: 0; }}
        .recommendations li {{ margin: 8px 0; }}
        table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        th {{ background-color: #f2f2f2; font-weight: bold; }}
        .status-success {{ color: #28a745; font-weight: bold; }}
        .status-error {{ color: #dc3545; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 Parallel-Mengene Benchmark Report</h1>
            <p>Generated on: {}</p>
            <p>Version: {}</p>
            <p>Benchmark Duration: {:.2}s</p>
        </div>
        
        <div class="summary">
            <div class="summary-card">
                <h3>Total Tests</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Successful Tests</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Failed Tests</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Avg Compression Speed</h3>
                <div class="value">{:.1} MB/s</div>
            </div>
            <div class="summary-card">
                <h3>Avg Decompression Speed</h3>
                <div class="value">{:.1} MB/s</div>
            </div>
            <div class="summary-card">
                <h3>Avg Compression Ratio</h3>
                <div class="value">{:.2}%</div>
            </div>
        </div>
        
        <h2>🏆 Performance Analysis</h2>
        <div class="algorithm-comparison">
            <h3>Best Overall Algorithm: {}</h3>
            <p>Based on weighted performance score considering speed, compression ratio, memory usage, and reliability.</p>
        </div>
        
        <h2>📊 Algorithm Comparisons</h2>
        {}
        
        <h2>💡 Recommendations</h2>
        <div class="recommendations">
            <h3>Performance Recommendations</h3>
            <ul>
                {}
            </ul>
        </div>
        
        <h2>📈 Detailed Results</h2>
        {}
        
        <h2>🖥️ System Information</h2>
        {}
    </div>
</body>
</html>"#,
            report.metadata.generated_at,
            report.metadata.version,
            report.metadata.benchmark_duration.as_secs_f64(),
            report.summary.total_tests,
            report.summary.successful_tests,
            report.summary.failed_tests,
            report.summary.average_compression_speed,
            report.summary.average_decompression_speed,
            report.summary.average_compression_ratio,
            report.performance_analysis.best_overall_algorithm,
            self.generate_algorithm_comparison_html(
                &report.performance_analysis.algorithm_comparisons
            ),
            report
                .performance_analysis
                .recommendations
                .join("</li><li>"),
            self.generate_detailed_results_html(&report.raw_metrics),
            self.generate_system_info_html(&report.system_info)
        )
    }

    /// Generate algorithm comparison HTML
    fn generate_algorithm_comparison_html(
        &self,
        comparisons: &[crate::performance_analyzer::AlgorithmComparison],
    ) -> String {
        let mut html = String::new();

        for comparison in comparisons {
            html.push_str(&format!(
                r#"<div class="algorithm-card">
                    <h3>{}</h3>
                    <p><strong>Compression Speed:</strong> {:.1} ± {:.1} MB/s</p>
                    <p><strong>Decompression Speed:</strong> {:.1} ± {:.1} MB/s</p>
                    <p><strong>Compression Ratio:</strong> {:.2} ± {:.2}%</p>
                    <p><strong>Memory Usage:</strong> {:.1} ± {:.1} MB</p>
                    <p><strong>Reliability:</strong> {:.1}%</p>
                </div>"#,
                comparison.algorithm,
                comparison.compression_speed_stats.mean,
                comparison.compression_speed_stats.std_deviation,
                comparison.decompression_speed_stats.mean,
                comparison.decompression_speed_stats.std_deviation,
                comparison.compression_ratio_stats.mean,
                comparison.compression_ratio_stats.std_deviation,
                comparison.memory_usage_stats.mean,
                comparison.memory_usage_stats.std_deviation,
                comparison.reliability_score
            ));
        }

        html
    }

    /// Generate detailed results HTML
    fn generate_detailed_results_html(&self, metrics: &[BenchmarkMetrics]) -> String {
        if metrics.is_empty() {
            return "<p>No detailed results available.</p>".to_string();
        }

        let mut html = r#"<table>
            <thead>
                <tr>
                    <th>Algorithm</th>
                    <th>File</th>
                    <th>File Size</th>
                    <th>Compression Speed</th>
                    <th>Decompression Speed</th>
                    <th>Compression Ratio</th>
                    <th>Memory Peak</th>
                    <th>CPU Usage</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>"#
            .to_string();

        for metric in metrics {
            let status_class = if metric.error_message.is_none() && metric.integrity_check {
                "status-success"
            } else {
                "status-error"
            };

            let status_text = if metric.error_message.is_none() && metric.integrity_check {
                "✅ Success"
            } else {
                "❌ Failed"
            };

            html.push_str(&format!(
                r#"<tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{:.1} MB/s</td>
                    <td>{:.1} MB/s</td>
                    <td>{:.2}%</td>
                    <td>{:.1} MB</td>
                    <td>{:.1}%</td>
                    <td class="{}">{}</td>
                </tr>"#,
                metric.algorithm,
                metric.file_name,
                metric.file_size,
                metric.compression_metrics.throughput_mbps,
                metric.decompression_metrics.throughput_mbps,
                metric.compression_metrics.compression_ratio,
                metric.compression_metrics.memory_peak_mb,
                metric.compression_metrics.cpu_usage_percent,
                status_class,
                status_text
            ));
        }

        html.push_str("</tbody></table>");
        html
    }

    /// Generate system info HTML
    fn generate_system_info_html(&self, system_info: &Option<SystemInfo>) -> String {
        match system_info {
            Some(info) => format!(
                r#"<table>
                    <tr><th>Operating System</th><td>{}</td></tr>
                    <tr><th>CPU Cores</th><td>{}</td></tr>
                    <tr><th>Total Memory</th><td>{:.1} GB</td></tr>
                    <tr><th>Available Memory</th><td>{:.1} GB</td></tr>
                    <tr><th>Rust Version</th><td>{}</td></tr>
                </table>"#,
                info.os,
                info.cpu_cores,
                info.total_memory_gb,
                info.available_memory_gb,
                info.rust_version
            ),
            None => "<p>System information not available.</p>".to_string(),
        }
    }

    /// Generate Markdown content
    fn generate_markdown_content(&self, report: &BenchmarkReport) -> String {
        format!(
            r#"# 🚀 Parallel-Mengene Benchmark Report

## 📊 Summary

- **Generated:** {}
- **Version:** {}
- **Duration:** {:.2}s
- **Total Tests:** {}
- **Successful:** {}
- **Failed:** {}

## 🏆 Performance Overview

- **Average Compression Speed:** {:.1} MB/s
- **Average Decompression Speed:** {:.1} MB/s
- **Average Compression Ratio:** {:.2}%
- **Peak Memory Usage:** {:.1} MB
- **Peak CPU Usage:** {:.1}%

## 🥇 Best Algorithms

- **Overall Best:** {}
- **Best Compression Speed:** {}
- **Best Decompression Speed:** {}
- **Best Compression Ratio:** {}
- **Most Memory Efficient:** {}

## 💡 Recommendations

{}

## 📈 Detailed Results

{}

## 🖥️ System Information

{}
"#,
            report.metadata.generated_at,
            report.metadata.version,
            report.metadata.benchmark_duration.as_secs_f64(),
            report.summary.total_tests,
            report.summary.successful_tests,
            report.summary.failed_tests,
            report.summary.average_compression_speed,
            report.summary.average_decompression_speed,
            report.summary.average_compression_ratio,
            report.summary.peak_memory_usage,
            report.summary.peak_cpu_usage,
            report.performance_analysis.best_overall_algorithm,
            report.performance_analysis.best_compression_speed,
            report.performance_analysis.best_decompression_speed,
            report.performance_analysis.best_compression_ratio,
            report.performance_analysis.best_memory_efficiency,
            report.performance_analysis.recommendations.join("\n- "),
            self.generate_detailed_results_markdown(&report.raw_metrics),
            self.generate_system_info_markdown(&report.system_info)
        )
    }

    /// Generate detailed results Markdown
    fn generate_detailed_results_markdown(&self, metrics: &[BenchmarkMetrics]) -> String {
        if metrics.is_empty() {
            return "No detailed results available.".to_string();
        }

        let mut markdown = "| Algorithm | File | Size | Comp Speed | Decomp Speed | Ratio | Memory | CPU | Status |\n".to_string();
        markdown.push_str("|-----------|------|------|------------|--------------|-------|--------|-----|--------|\n");

        for metric in metrics {
            let status = if metric.error_message.is_none() && metric.integrity_check {
                "✅"
            } else {
                "❌"
            };

            markdown.push_str(&format!(
                "| {} | {} | {} | {:.1} MB/s | {:.1} MB/s | {:.2}% | {:.1} MB | {:.1}% | {} |\n",
                metric.algorithm,
                metric.file_name,
                metric.file_size,
                metric.compression_metrics.throughput_mbps,
                metric.decompression_metrics.throughput_mbps,
                metric.compression_metrics.compression_ratio,
                metric.compression_metrics.memory_peak_mb,
                metric.compression_metrics.cpu_usage_percent,
                status
            ));
        }

        markdown
    }

    /// Generate system info Markdown
    fn generate_system_info_markdown(&self, system_info: &Option<SystemInfo>) -> String {
        match system_info {
            Some(info) => format!(
                "- **OS:** {}\n- **CPU Cores:** {}\n- **Total Memory:** {:.1} GB\n- **Available Memory:** {:.1} GB\n- **Rust Version:** {}",
                info.os,
                info.cpu_cores,
                info.total_memory_gb,
                info.available_memory_gb,
                info.rust_version
            ),
            None => "System information not available.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_generator_creation() {
        let config = ReportConfig::default();
        let generator = ReportGenerator::new(config);
        assert!(generator.config.include_charts);
    }

    #[test]
    fn test_system_info_collection() {
        let system_info = ReportGenerator::collect_system_info();
        assert!(system_info.cpu_cores > 0);
        assert!(system_info.total_memory_gb > 0.0);
    }
}
