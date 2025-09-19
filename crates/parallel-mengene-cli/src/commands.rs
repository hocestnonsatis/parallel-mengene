//! Command implementations

use anyhow::Result;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::utils::get_cpu_count;
use parallel_mengene_pipeline::intelligent_pipeline::IntelligentPipeline;
use std::path::PathBuf;
use tracing::info;

// use crate::progress::{ProgressTracker, StatisticsCollector, DecompressionProgressTracker}; // Not used in current implementation

// Removed unused functions: archive_directory_to_temp, generate_compress_output_path, resolve_compress_output_path

/// Compress a file or directory with intelligent algorithm selection
pub async fn compress(input: PathBuf, output: Option<PathBuf>, verbose: bool) -> Result<()> {
    let output = output.unwrap_or_else(|| {
        let input_name = input
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "archive".to_string());
        PathBuf::from(format!("{}.pma", input_name))
    });

    info!("Intelligently compressing {:?} to {:?}", input, output);

    // Create intelligent pipeline
    let pipeline = IntelligentPipeline::new()?;

    // Compress with intelligent algorithm selection
    let results = pipeline.compress_intelligently(&input, &output).await?;

    // Print results
    println!("Compression completed!");
    println!("Files processed: {}", results.jobs.len());
    println!(
        "Original size: {:.2} MB",
        results.total_original_size as f64 / 1_048_576.0
    );
    println!(
        "Compressed size: {:.2} MB",
        results.total_compressed_size as f64 / 1_048_576.0
    );
    println!(
        "Compression ratio: {:.1}%",
        (1.0 - results.total_compressed_size as f64 / results.total_original_size as f64) * 100.0
    );
    println!(
        "Compression time: {:.2}s",
        results.compression_time.as_secs_f64()
    );

    if verbose {
        println!("\nAlgorithm Selection Details:");
        println!("===========================");
        for job in &results.jobs {
            println!(
                "{}: {} (level {}, {} threads) - {}",
                job.input_path.display(),
                format_algorithm(job.algorithm),
                job.level,
                job.threads,
                job.reason
            );
        }
    }

    Ok(())
}

// Removed unused functions: file_seems_tar, generate_decompress_output_base, resolve_decompress_targets

pub async fn decompress(input: PathBuf, output: Option<PathBuf>, verbose: bool) -> Result<()> {
    let output = output.unwrap_or_else(|| {
        let input_name = input
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "extracted".to_string());
        PathBuf::from(format!("{}_decompressed", input_name))
    });

    info!("Decompressing PMA file {:?} to {:?}", input, output);

    // Create intelligent pipeline
    let pipeline = IntelligentPipeline::new()?;

    // Decompress PMA file
    pipeline.decompress_pma(&input, &output).await?;

    println!("Decompression completed!");
    println!("Files extracted to: {:?}", output);

    if verbose {
        // Read and display PMA file information
        use parallel_mengene_core::binary_pma_format::BinaryPmaFile;
        let pma_file = BinaryPmaFile::load(&input)?;
        let stats = pma_file.get_stats();

        println!("\nPMA Archive Information:");
        println!("=======================");
        println!("Files: {}", stats.file_count);
        println!(
            "Original Size: {:.2} MB",
            stats.total_original_size as f64 / 1_048_576.0
        );
        println!(
            "Compressed Size: {:.2} MB",
            stats.total_compressed_size as f64 / 1_048_576.0
        );
        println!("Compression Ratio: {:.1}%", stats.overall_ratio * 100.0);
        println!("Algorithms Used: {}", stats.algorithm_usage.len());

        println!("\nFiles in Archive:");
        for entry in pma_file.list_files() {
            println!(
                "  {} | {} | {} | {:.1}% | {} | {} threads",
                entry.path,
                format_size(entry.original_size),
                format_size(entry.compressed_size),
                entry.compression_ratio as f64 / 10.0,
                format_algorithm(entry.algorithm),
                entry.threads
            );
        }
    }

    Ok(())
}

/// Benchmark compression algorithms
pub async fn benchmark(
    input: PathBuf,
    algorithms: Vec<CompressionAlgorithm>,
    threads: Option<usize>,
    _verbose: bool,
) -> Result<()> {
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Benchmarking algorithms on {:?}", input);
    info!("Algorithms: {:?}, Threads: {}", algorithms, threads);

    if !input.exists() {
        return Err(anyhow::anyhow!("Input file does not exist: {:?}", input));
    }

    let file_size = std::fs::metadata(&input)?.len();
    let input_data = std::fs::read(&input)?;

    println!("Benchmarking on file: {:?} ({} bytes)", input, file_size);
    println!("Algorithms: {:?}", algorithms);
    println!("Threads: {}", threads);
    println!();

    for algorithm in &algorithms {
        println!("Testing algorithm: {:?}", algorithm);

        let compression_context =
            parallel_mengene_core::compression::CompressionContext::new(*algorithm, None);

        // Measure compression
        let start = std::time::Instant::now();
        let compressed_data = compression_context.compress(&input_data)?;
        let compression_time = start.elapsed();

        // Measure decompression
        let start = std::time::Instant::now();
        let decompressed_data = compression_context.decompress(&compressed_data)?;
        let decompression_time = start.elapsed();

        // Verify integrity
        let integrity_ok = input_data == decompressed_data;

        // Calculate metrics
        let compression_ratio =
            (1.0 - compressed_data.len() as f64 / input_data.len() as f64) * 100.0;
        let compression_speed =
            (input_data.len() as f64 / 1_048_576.0) / compression_time.as_secs_f64().max(1e-9);
        let decompression_speed = (compressed_data.len() as f64 / 1_048_576.0)
            / decompression_time.as_secs_f64().max(1e-9);

        println!("  Compression time: {:.2}ms", compression_time.as_millis());
        println!(
            "  Decompression time: {:.2}ms",
            decompression_time.as_millis()
        );
        println!("  Compression ratio: {:.2}%", compression_ratio);
        println!("  Compression speed: {:.2} MB/s", compression_speed);
        println!("  Decompression speed: {:.2} MB/s", decompression_speed);
        println!(
            "  Integrity check: {}",
            if integrity_ok { "PASS" } else { "FAIL" }
        );
        println!();
    }

    println!("Benchmark completed successfully!");

    Ok(())
}

/// Format algorithm name for display
fn format_algorithm(algorithm: CompressionAlgorithm) -> &'static str {
    match algorithm {
        CompressionAlgorithm::Lz4 => "LZ4",
        CompressionAlgorithm::Gzip => "Gzip",
        CompressionAlgorithm::Zstd => "Zstd",
    }
}

/// Format file size in human-readable format
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}
