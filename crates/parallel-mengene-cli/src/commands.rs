//! Command implementations

use anyhow::Result;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::utils::{format_file_size, get_cpu_count};
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use std::io::Read;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use tracing::info;

/// Archive a directory into a temporary .tar file
fn archive_directory_to_temp(input_dir: &Path) -> Result<NamedTempFile> {
    use std::fs::File;

    let temp_tar = NamedTempFile::new()?;
    let file = File::options().write(true).open(temp_tar.path())?;
    let mut builder = tar::Builder::new(file);

    // Preserve the top-level directory name
    let dir_name = input_dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("archive"));

    builder.append_dir_all(&dir_name, input_dir)?;
    builder.finish()?;

    Ok(temp_tar)
}

/// Generate output path with .lz4 extension for compression
fn generate_compress_output_path(input: &Path) -> PathBuf {
    // Place alongside input, with full name (including original extension) plus .lz4 suffix
    let parent = input.parent().unwrap_or_else(|| Path::new("."));
    let name = input.file_name().unwrap_or_default().to_string_lossy();
    parent.join(format!("{}.lz4", name))
}

/// Resolve compression output path when user provided `output` may be a directory
fn resolve_compress_output_path(input: &Path, output: Option<PathBuf>) -> PathBuf {
    match output {
        None => generate_compress_output_path(input),
        Some(out) => {
            if out.exists() && out.is_dir() {
                // Put file inside directory, preserving input file/dir name and adding .lz4
                let name = input.file_name().unwrap_or_default().to_string_lossy();
                return out.join(format!("{}.lz4", name));
            }
            out
        }
    }
}

/// Compress a file or directory
pub async fn compress(
    input: PathBuf,
    output: Option<PathBuf>,
    algorithm: CompressionAlgorithm,
    level: Option<u32>,
    threads: Option<usize>,
    verbose: bool,
) -> Result<()> {
    let output = resolve_compress_output_path(&input, output);
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Compressing {:?} to {:?}", input, output);
    info!(
        "Algorithm: {:?}, Level: {:?}, Threads: {}",
        algorithm, level, threads
    );

    // Create parallel pipeline
    let pipeline = ParallelPipeline::new(algorithm)?;

    // Decide whether input is file or directory; if directory, archive first
    let metadata = std::fs::metadata(&input)?;
    let (input_path_for_compression, file_size);
    let _temp_guard; // keep temp file alive while compressing

    if metadata.is_dir() {
        info!("Input is a directory. Creating temporary tar archive...");
        let temp_tar = archive_directory_to_temp(&input)?;
        let tar_path = temp_tar.path().to_path_buf();
        let tar_size = std::fs::metadata(&tar_path)?.len();
        _temp_guard = Some(temp_tar);
        input_path_for_compression = tar_path;
        file_size = tar_size;
    } else {
        input_path_for_compression = input.clone();
        file_size = metadata.len();
        _temp_guard = None;
    }

    // Compress using parallel approach
    let start = std::time::Instant::now();
    pipeline
        .compress_file(&input_path_for_compression, &output)
        .await?;
    let elapsed = start.elapsed();

    // Report results
    let compressed_size = std::fs::metadata(&output)?.len();
    let compression_ratio = (1.0 - compressed_size as f64 / file_size as f64) * 100.0;

    println!("Compression completed successfully!");
    println!("Input: {}", format_file_size(file_size));
    println!("Output: {}", format_file_size(compressed_size));
    println!("Compression ratio: {:.2}%", compression_ratio);
    if verbose {
        let mb = file_size as f64 / 1_048_576f64;
        let secs = elapsed.as_secs_f64().max(1e-9);
        let speed = mb / secs;
        println!("Speed: {:.2} MB/s (elapsed: {:.2}s)", speed, secs);
    }

    Ok(())
}

/// Decompress a file or directory
/// Determine if a file appears to be a tar archive by probing with `tar`
fn file_seems_tar(path: &Path) -> bool {
    // Only check for tar if the file is reasonably large (tar files are usually > 512 bytes)
    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.len() < 512 {
            return false;
        }
    }

    // For LZ4 files, we should not treat them as tar archives
    if let Some(extension) = path.extension() {
        if extension == "lz4" {
            return false;
        }
    }

    // More robust tar detection: check for tar magic bytes and try to read entries
    std::fs::File::open(path)
        .ok()
        .and_then(|mut f| {
            // Check for tar magic bytes at the beginning
            let mut buffer = [0; 512];
            if f.read_exact(&mut buffer).is_err() {
                return Some(false);
            }

            // Tar files start with a 512-byte header, check if it looks like a tar header
            // Tar headers have specific structure: filename (100 bytes), mode (8 bytes), etc.
            let filename_start = 0;
            let filename_end = 100;
            let filename_bytes = &buffer[filename_start..filename_end];

            // Check if the filename area contains printable characters or nulls (typical for tar)
            let has_valid_filename = filename_bytes
                .iter()
                .all(|&b| b == 0 || (b >= 32 && b <= 126));

            if !has_valid_filename {
                return Some(false);
            }

            // Now try to read the tar archive
            let mut ar = tar::Archive::new(f);
            if let Ok(entries) = ar.entries() {
                // Try to read the first few entries to confirm it's a valid tar
                let mut count = 0;
                for entry in entries {
                    if entry.is_err() {
                        return Some(false);
                    }
                    count += 1;
                    if count >= 3 {
                        // Check first 3 entries
                        break;
                    }
                }
                Some(count > 0)
            } else {
                Some(false)
            }
        })
        .unwrap_or(false)
}

/// Build default decompressed base path (without considering output-as-directory)
fn generate_decompress_output_base(input: &Path) -> PathBuf {
    let mut output = input.to_path_buf();
    if let Some(extension) = output.extension() {
        if extension == "lz4" {
            // Remove .lz4 extension
            output.set_extension("");
        } else {
            let stem = output.file_stem().unwrap_or_default();
            output.set_file_name(format!("{}_decompressed", stem.to_string_lossy()));
        }
    } else {
        let stem = output.file_name().unwrap_or_default();
        output.set_file_name(format!("{}_decompressed", stem.to_string_lossy()));
    }
    output
}

/// Resolve decompression target location considering if `output` is a directory
fn resolve_decompress_targets(
    input: &Path,
    output: Option<PathBuf>,
) -> (PathBuf, bool /*output_is_dir*/) {
    let base = generate_decompress_output_base(input);
    match output {
        None => (base, false),
        Some(out) => {
            if out.exists() && out.is_dir() {
                // Place result inside this directory with original name (input stem without .lz4)
                let stem = input
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "output".to_string());
                return (out.join(stem), true);
            }
            (out, false)
        }
    }
}

pub async fn decompress(
    input: PathBuf,
    output: Option<PathBuf>,
    algorithm: CompressionAlgorithm,
    threads: Option<usize>,
    verbose: bool,
) -> Result<()> {
    // Resolve final target path. If user passed a directory, we will create the result inside it.
    let (final_target, output_is_dir_hint) = resolve_decompress_targets(&input, output);
    let threads = threads.unwrap_or_else(get_cpu_count);

    info!("Decompressing {:?} to {:?}", input, final_target);
    info!("Algorithm: {:?}, Threads: {}", algorithm, threads);

    // Create parallel pipeline
    let pipeline = ParallelPipeline::new(algorithm)?;

    // Decompress to a temporary file first
    let temp_out = NamedTempFile::new()?;
    let temp_out_path = temp_out.path().to_path_buf();

    let start = std::time::Instant::now();
    pipeline.decompress_file(&input, &temp_out_path).await?;
    let elapsed = start.elapsed();

    // Post-process: if the decompressed data is a tar archive, extract preserving nested structure
    if file_seems_tar(&temp_out_path) {
        // Determine parent directory for extraction
        let target_parent = if (final_target.exists() && final_target.is_dir())
            || output_is_dir_hint
        {
            // User passed an output directory; extract under it preserving top-level dir in the tar
            final_target.clone()
        } else {
            // Extract into parent directory; tar contains top-level folder already
            final_target
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        };

        std::fs::create_dir_all(&target_parent)?;
        let file = std::fs::File::open(&temp_out_path)?;
        let mut archive = tar::Archive::new(file);
        archive.unpack(&target_parent)?;

        // Remove temp file and, if final_target was intended as a file path (not dir), nothing to move:
        // Because archive preserves original top directory name from compression, we don't rename.
        drop(temp_out);
    } else {
        // Regular file: move to final target path. If user specified a directory, we already resolved that to include the original filename
        if let Some(parent) = final_target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Overwrite if exists
        std::fs::rename(&temp_out_path, &final_target).or_else(|_| {
            // Cross-device fallback
            let data = std::fs::read(&temp_out_path)?;
            std::fs::write(&final_target, data)
        })?;
        drop(temp_out);
    }

    println!("Decompression completed successfully!");
    if verbose {
        let compressed_size = std::fs::metadata(&input)?.len();
        let mb = compressed_size as f64 / 1_048_576f64;
        let secs = elapsed.as_secs_f64().max(1e-9);
        let speed = mb / secs;
        println!("Speed: {:.2} MB/s (elapsed: {:.2}s)", speed, secs);
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
