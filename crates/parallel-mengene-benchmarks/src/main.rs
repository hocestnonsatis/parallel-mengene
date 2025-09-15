//! Advanced benchmarking tool for parallel-mengene

use clap::{Parser, Subcommand};
use parallel_mengene_benchmarks::benchmark_runner::BenchmarkConfig;
use parallel_mengene_benchmarks::report_generator::{
    BenchmarkReport, ReportConfig, ReportFormat, ReportStyle,
};
use parallel_mengene_benchmarks::test_data_generator::{
    CompressionLevel, DataType, TestDataConfig,
};
use parallel_mengene_benchmarks::*;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(name = "parallel-mengene-bench")]
#[command(about = "Advanced benchmarking tool for parallel-mengene compression algorithms")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run comprehensive benchmarks
    Run {
        /// Input files or directories to benchmark
        #[arg(long)]
        input: Vec<PathBuf>,

        /// Algorithms to test
        #[arg(short, long, default_values = ["lz4", "gzip", "zstd"])]
        algorithms: Vec<CompressionAlgorithm>,

        /// Number of benchmark iterations
        #[arg(long, default_value = "3")]
        iterations: usize,

        /// Number of warmup iterations
        #[arg(long, default_value = "1")]
        warmup: usize,

        /// Output directory for results
        #[arg(long, default_value = "benchmark_results")]
        output: PathBuf,

        /// Enable memory tracking
        #[arg(long)]
        memory_tracking: bool,

        /// Enable CPU tracking
        #[arg(long)]
        cpu_tracking: bool,

        /// Benchmark timeout in seconds
        #[arg(long, default_value = "300")]
        timeout: u64,
    },

    /// Generate test data for benchmarking
    Generate {
        /// Output directory for test data
        #[arg(short, long, default_value = "test_data")]
        output: PathBuf,

        /// File sizes to generate (in bytes)
        #[arg(short, long, default_values = ["1024", "1048576", "10485760", "104857600"])]
        sizes: Vec<usize>,

        /// Data types to generate
        #[arg(short, long, default_values = ["random", "repetitive", "text", "binary", "mixed", "zerofilled"])]
        types: Vec<String>,

        /// Compression difficulty levels
        #[arg(short, long, default_values = ["easy", "medium", "hard"])]
        levels: Vec<String>,

        /// Random seed for reproducible data
        #[arg(long)]
        seed: Option<u64>,
    },

    /// Analyze existing benchmark results
    Analyze {
        /// Input JSON file with benchmark results
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for analysis
        #[arg(short, long, default_value = "analysis_results")]
        output: PathBuf,

        /// Generate charts and visualizations
        #[arg(long)]
        charts: bool,
    },

    /// Compare algorithms performance
    Compare {
        /// Benchmark results JSON files to compare
        #[arg(short, long)]
        results: Vec<PathBuf>,

        /// Output comparison report
        #[arg(short, long, default_value = "comparison_report.html")]
        output: PathBuf,

        /// Comparison criteria
        #[arg(long, default_values = ["speed", "ratio", "memory", "reliability"])]
        criteria: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("parallel_mengene_benchmarks={}", log_level))
        .init();

    match cli.command {
        Command::Run {
            input,
            algorithms,
            iterations,
            warmup,
            output,
            memory_tracking,
            cpu_tracking,
            timeout,
        } => {
            run_benchmarks(
                input,
                algorithms,
                iterations,
                warmup,
                output,
                memory_tracking,
                cpu_tracking,
                timeout,
            )
            .await
        }
        Command::Generate {
            output,
            sizes,
            types,
            levels,
            seed,
        } => generate_test_data(output, sizes, types, levels, seed).await,
        Command::Analyze {
            input,
            output,
            charts,
        } => analyze_results(input, output, charts).await,
        Command::Compare {
            results,
            output,
            criteria,
        } => compare_results(results, output, criteria).await,
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_benchmarks(
    input_files: Vec<PathBuf>,
    algorithms: Vec<CompressionAlgorithm>,
    iterations: usize,
    warmup: usize,
    output_dir: PathBuf,
    memory_tracking: bool,
    cpu_tracking: bool,
    timeout: u64,
) -> anyhow::Result<()> {
    info!("🚀 Starting comprehensive benchmark suite");

    // Create benchmark configuration
    let config = BenchmarkConfig {
        algorithms,
        test_files: input_files,
        iterations,
        warmup_iterations: warmup,
        output_dir: output_dir.clone(),
        enable_memory_tracking: memory_tracking,
        enable_cpu_tracking: cpu_tracking,
        timeout_seconds: timeout,
    };

    // Create benchmark runner
    let mut runner = BenchmarkRunner::new(config).await?;

    // Run benchmarks
    let start_time = std::time::Instant::now();
    let metrics = runner.run_benchmarks().await?;
    let benchmark_duration = start_time.elapsed();

    info!("✅ Benchmark completed in {:?}", benchmark_duration);
    info!("📊 Collected {} metrics", metrics.len());

    // Analyze performance
    info!("📈 Analyzing performance data...");
    let performance_analysis = PerformanceAnalyzer::analyze_performance(&metrics)?;

    // Generate reports
    info!("📝 Generating comprehensive reports...");
    let report_config = ReportConfig {
        output_format: ReportFormat::All,
        include_charts: true,
        include_raw_data: true,
        include_system_info: true,
        template_style: ReportStyle::Professional,
    };

    let report_generator = ReportGenerator::new(report_config);
    report_generator.generate_report(
        metrics,
        performance_analysis,
        benchmark_duration,
        &output_dir,
    )?;

    info!("📊 Reports generated in: {:?}", output_dir);
    info!("🎯 View results: {:?}/benchmark_report.html", output_dir);

    Ok(())
}

async fn generate_test_data(
    output_dir: PathBuf,
    sizes: Vec<usize>,
    types: Vec<String>,
    levels: Vec<String>,
    seed: Option<u64>,
) -> anyhow::Result<()> {
    info!("🔧 Generating test data for benchmarking");

    // Parse data types
    let data_types: anyhow::Result<Vec<DataType>> = types
        .iter()
        .map(|t| match t.to_lowercase().as_str() {
            "random" => Ok(DataType::Random),
            "repetitive" => Ok(DataType::Repetitive),
            "text" => Ok(DataType::Text),
            "binary" => Ok(DataType::Binary),
            "mixed" => Ok(DataType::Mixed),
            "zerofilled" => Ok(DataType::ZeroFilled),
            "patternbased" => Ok(DataType::PatternBased),
            _ => Err(anyhow::anyhow!("Invalid data type: {}", t)),
        })
        .collect();

    // Parse compression levels
    let compression_levels: anyhow::Result<Vec<CompressionLevel>> = levels
        .iter()
        .map(|l| match l.to_lowercase().as_str() {
            "easy" => Ok(CompressionLevel::Easy),
            "medium" => Ok(CompressionLevel::Medium),
            "hard" => Ok(CompressionLevel::Hard),
            _ => Err(anyhow::anyhow!("Invalid compression level: {}", l)),
        })
        .collect();

    // Create configuration
    let config = TestDataConfig {
        output_dir,
        file_sizes: sizes,
        data_types: data_types?,
        compression_levels: compression_levels?,
        seed,
    };

    // Generate test data
    let mut generator = TestDataGenerator::new(config);
    generator.generate_all()?;

    info!("✅ Test data generation completed");

    Ok(())
}

async fn analyze_results(
    input_file: PathBuf,
    output_dir: PathBuf,
    charts: bool,
) -> anyhow::Result<()> {
    info!("📊 Analyzing benchmark results from: {:?}", input_file);

    // Read benchmark results
    let json_content = std::fs::read_to_string(&input_file)?;
    let report: BenchmarkReport = serde_json::from_str(&json_content)?;

    // Create output directory
    std::fs::create_dir_all(&output_dir)?;

    // Perform additional analysis
    let detailed_analysis = PerformanceAnalyzer::analyze_performance(&report.raw_metrics)?;

    // Generate enhanced report
    let report_config = ReportConfig {
        output_format: ReportFormat::All,
        include_charts: charts,
        include_raw_data: true,
        include_system_info: true,
        template_style: ReportStyle::Technical,
    };

    let report_generator = ReportGenerator::new(report_config);
    report_generator.generate_report(
        report.raw_metrics,
        detailed_analysis,
        report.metadata.benchmark_duration,
        &output_dir,
    )?;

    info!("📈 Analysis completed and saved to: {:?}", output_dir);

    Ok(())
}

async fn compare_results(
    result_files: Vec<PathBuf>,
    output_file: PathBuf,
    criteria: Vec<String>,
) -> anyhow::Result<()> {
    info!("⚖️ Comparing benchmark results");

    if result_files.len() < 2 {
        return Err(anyhow::anyhow!("Need at least 2 result files to compare"));
    }

    // Read all result files
    let mut reports = Vec::new();
    for file in &result_files {
        let json_content = std::fs::read_to_string(file)?;
        let report: BenchmarkReport = serde_json::from_str(&json_content)?;
        reports.push(report);
    }

    // Generate comparison report
    let comparison_html = generate_comparison_report(&reports, &criteria);
    std::fs::write(&output_file, comparison_html)?;

    info!("📊 Comparison report generated: {:?}", output_file);

    Ok(())
}

fn generate_comparison_report(_reports: &[BenchmarkReport], _criteria: &[String]) -> String {
    let mut html = String::new();

    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Benchmark Comparison Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 1200px; margin: 0 auto; }
        .comparison-table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        .comparison-table th, .comparison-table td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        .comparison-table th { background-color: #f2f2f2; }
        .best { background-color: #d4edda; font-weight: bold; }
        .worst { background-color: #f8d7da; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 Benchmark Comparison Report</h1>
        <p>Generated on: {}</p>
        
        <h2>📈 Performance Comparison</h2>
        <table class="comparison-table">
            <thead>
                <tr>
                    <th>Metric</th>
                    <th>Best</th>
                    <th>Worst</th>
                    <th>Average</th>
                </tr>
            </thead>
            <tbody>"#);

    // Add comparison data here
    html.push_str(
        r#"
            </tbody>
        </table>
    </div>
</body>
</html>"#,
    );

    html
}
