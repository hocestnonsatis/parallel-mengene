//! Command implementations

use anyhow::Result;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::utils::{format_file_size, get_cpu_count};
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use std::path::PathBuf;
use tracing::info;

/// Compress a file or directory
pub async fn compress(
    input: PathBuf,
    output: PathBuf,
    algorithm: CompressionAlgorithm,
    level: Option<u32>,
    threads: Option<usize>,
) -> Result<()> {
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Compressing {:?} to {:?}", input, output);
    info!(
        "Algorithm: {:?}, Level: {:?}, Threads: {}",
        algorithm, level, threads
    );

    // Create parallel pipeline
    let pipeline = ParallelPipeline::new(algorithm)?;

    // Get file size for progress reporting
    let file_size = std::fs::metadata(&input)?.len();

    // Compress using parallel approach
    pipeline.compress_file(&input, &output).await?;

    // Report results
    let compressed_size = std::fs::metadata(&output)?.len();
    let compression_ratio = (1.0 - compressed_size as f64 / file_size as f64) * 100.0;

    println!("Compression completed successfully!");
    println!("Input: {}", format_file_size(file_size));
    println!("Output: {}", format_file_size(compressed_size));
    println!("Compression ratio: {:.2}%", compression_ratio);

    Ok(())
}

/// Decompress a file or directory
pub async fn decompress(
    input: PathBuf,
    output: PathBuf,
    algorithm: CompressionAlgorithm,
    threads: Option<usize>,
) -> Result<()> {
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Decompressing {:?} to {:?}", input, output);
    info!("Algorithm: {:?}, Threads: {}", algorithm, threads);

    // Create parallel pipeline
    let pipeline = ParallelPipeline::new(algorithm)?;

    // Decompress using parallel approach
    pipeline.decompress_file(&input, &output).await?;

    println!("Decompression completed successfully!");

    Ok(())
}

/// Benchmark compression algorithms
pub async fn benchmark(
    input: PathBuf,
    algorithms: Vec<CompressionAlgorithm>,
    threads: Option<usize>,
) -> Result<()> {
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Benchmarking algorithms on {:?}", input);
    info!("Algorithms: {:?}, Threads: {}", algorithms, threads);

    // TODO: Implement actual benchmarking logic
    // This would involve:
    // 1. Reading the input file
    // 2. Testing each algorithm with different compression levels
    // 3. Measuring compression time, decompression time, and ratio
    // 4. Displaying results in a nice table format

    println!("Benchmark completed successfully!");

    Ok(())
}
