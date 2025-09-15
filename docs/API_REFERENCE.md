# Parallel Mengene API Reference

## Overview

Parallel Mengene is a high-performance parallel compression library written in Rust. It provides efficient compression and decompression using multiple CPU cores and optimized algorithms.

## Core Module (`parallel-mengene-core`)

### Compression Algorithms

#### `CompressionAlgorithm`

Supported compression algorithms:

```rust
pub enum CompressionAlgorithm {
    Lz4,    // Fast compression with good speed
    Gzip,   // Balanced compression with good ratio
    Zstd,   // High compression ratio with good speed
}
```

**Methods:**
- `default_level() -> u32`: Get the default compression level
- `max_level() -> u32`: Get the maximum compression level
- `from_str(s: &str) -> Result<Self, String>`: Parse from string

**Example:**
```rust
use parallel_mengene_core::algorithms::CompressionAlgorithm;

let algorithm = CompressionAlgorithm::Zstd;
assert_eq!(algorithm.default_level(), 3);
assert_eq!(algorithm.max_level(), 22);
```

### Compression Context

#### `CompressionContext`

Main compression/decompression interface:

```rust
pub struct CompressionContext {
    algorithm: CompressionAlgorithm,
    level: u32,
}
```

**Methods:**
- `new(algorithm: CompressionAlgorithm, level: Option<u32>) -> Self`: Create new context
- `compress(data: &[u8]) -> Result<Vec<u8>>`: Compress data
- `decompress(data: &[u8]) -> Result<Vec<u8>>`: Decompress data
- `algorithm() -> CompressionAlgorithm`: Get current algorithm
- `level() -> u32`: Get current compression level

**Example:**
```rust
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, Some(6));
let data = b"Hello, World!";
let compressed = ctx.compress(data).unwrap();
let decompressed = ctx.decompress(&compressed).unwrap();
assert_eq!(decompressed, data);
```

### Error Handling

#### `Error`

Comprehensive error types:

```rust
pub enum Error {
    Io(std::io::Error),                    // I/O errors
    Compression(String),                   // Compression errors
    InvalidInput(String),                  // Invalid input errors
    MemoryMapping(String),                 // Memory mapping errors
    Threading(String),                     // Threading errors
    GpuNotAvailable(String),               // GPU not available
}
```

**Example:**
```rust
use parallel_mengene_core::error::{Error, Result};

fn process_data(data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Err(Error::InvalidInput("Data cannot be empty".to_string()));
    }
    // Process data...
    Ok(data.to_vec())
}
```

### Utility Functions

#### `compression_ratio(original_size: usize, compressed_size: usize) -> f64`

Calculate compression ratio (0.0 to 1.0+).

#### `space_savings(original_size: usize, compressed_size: usize) -> f64`

Calculate space savings percentage.

#### `format_file_size(bytes: u64) -> String`

Format file size in human-readable format.

#### `validate_file_path<P: AsRef<Path>>(path: P) -> Result<()>`

Validate that a path exists and is a file.

#### `get_cpu_count() -> usize`

Get the number of available CPU cores.

**Example:**
```rust
use parallel_mengene_core::utils::*;

let ratio = compression_ratio(1000, 500); // 0.5
let savings = space_savings(1000, 500);  // 50.0%
let formatted = format_file_size(1048576); // "1.0 MB"
let cpu_count = get_cpu_count(); // e.g., 8
```

## Pipeline Module (`parallel-mengene-pipeline`)

### Parallel Pipeline

#### `ParallelPipeline`

Main parallel compression pipeline:

```rust
pub struct ParallelPipeline {
    cpu_pipeline: CpuPipeline,
    coordinator: PipelineCoordinator,
    memory_config: MemoryMappingConfig,
    memory_tracker: MemoryUsageTracker,
}
```

**Methods:**
- `new(algorithm: CompressionAlgorithm) -> Result<Self>`: Create new pipeline
- `compress_file(input_path: &Path, output_path: &Path) -> Result<()>`: Compress file
- `decompress_file(input_path: &Path, output_path: &Path) -> Result<()>`: Decompress file
- `get_stats() -> PipelineStats`: Get pipeline statistics
- `configure_memory_mapping(config: MemoryMappingConfig)`: Configure memory mapping

**Example:**
```rust
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::Path;

let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
pipeline.compress_file(Path::new("input.txt"), Path::new("output.pmz")).await?;
pipeline.decompress_file(Path::new("output.pmz"), Path::new("decompressed.txt")).await?;
```

### CPU Pipeline

#### `CpuPipeline`

CPU-based compression pipeline:

```rust
pub struct CpuPipeline {
    algorithm: CompressionAlgorithm,
    context: CompressionContext,
}
```

**Methods:**
- `new(algorithm: CompressionAlgorithm) -> Result<Self>`: Create new CPU pipeline
- `compress_file(input_path: &Path, output_path: &Path) -> Result<()>`: Compress file
- `compress_chunk_sync(data: &[u8]) -> Result<Vec<u8>>`: Compress chunk synchronously
- `compress_chunk(data: &[u8]) -> Result<Vec<u8>>`: Compress chunk asynchronously
- `decompress_chunk_sync(data: &[u8]) -> Result<Vec<u8>>`: Decompress chunk synchronously
- `decompress_chunk(data: &[u8]) -> Result<Vec<u8>>`: Decompress chunk asynchronously
- `process_metadata(file_path: &Path) -> Result<FileMetadata>`: Process file metadata

**Example:**
```rust
use parallel_mengene_pipeline::cpu_pipeline::CpuPipeline;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
let data = b"Hello, World!";
let compressed = pipeline.compress_chunk_sync(data).unwrap();
let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();
assert_eq!(decompressed, data);
```

### Progress Tracking

#### `CompressionProgress`

Progress tracking for compression operations:

```rust
pub struct CompressionProgress {
    pub total_bytes: u64,
    pub processed_bytes: u64,
    pub chunks_total: usize,
    pub chunks_processed: usize,
    pub start_time: Instant,
    pub current_speed_mbps: f64,
}
```

**Methods:**
- `new(total_bytes: u64, chunks_total: usize) -> Self`: Create new progress tracker
- `update(processed_bytes: u64, chunks_processed: usize)`: Update progress
- `percentage() -> f64`: Get completion percentage
- `eta_seconds() -> Option<f64>`: Get estimated time to completion

**Example:**
```rust
use parallel_mengene_pipeline::parallel_pipeline::CompressionProgress;
use std::time::Instant;

let mut progress = CompressionProgress::new(1000, 10);
progress.update(500, 5);
assert_eq!(progress.percentage(), 50.0);
```

### Pipeline Statistics

#### `PipelineStats`

Pipeline configuration and statistics:

```rust
pub struct PipelineStats {
    pub chunk_size: usize,
    pub parallel_threshold: usize,
    pub max_workers: usize,
    pub memory_mapping_enabled: bool,
}
```

**Example:**
```rust
let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
let stats = pipeline.get_stats();
println!("Chunk size: {} bytes", stats.chunk_size);
println!("Max workers: {}", stats.max_workers);
```

## CLI Module (`parallel-mengene-cli`)

### Command Line Interface

The CLI provides a command-line interface for compression operations:

```bash
# Compress a file
parallel-mengene compress input.txt output.pmz --algorithm zstd --level 6

# Decompress a file
parallel-mengene decompress input.pmz output.txt

# Show help
parallel-mengene --help
```

**Options:**
- `--algorithm <ALGORITHM>`: Compression algorithm (lz4, gzip, zstd)
- `--level <LEVEL>`: Compression level (1-22 for zstd, 1-9 for gzip, 1-16 for lz4)
- `--threads <THREADS>`: Number of threads to use
- `--memory-mapping`: Enable memory mapping for large files
- `--verbose`: Enable verbose output

## Examples

### Basic Compression

```rust
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, None);
let data = b"Hello, World!";
let compressed = ctx.compress(data).unwrap();
let decompressed = ctx.decompress(&compressed).unwrap();
assert_eq!(decompressed, data);
```

### File Compression

```rust
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
    
    pipeline.compress_file(
        Path::new("input.txt"),
        Path::new("output.pmz")
    ).await?;
    
    pipeline.decompress_file(
        Path::new("output.pmz"),
        Path::new("decompressed.txt")
    ).await?;
    
    Ok(())
}
```

### Performance Profiling

```rust
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use crate::performance::PerformanceProfiler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut profiler = PerformanceProfiler::new();
    let test_data = b"Hello, World! This is a test string for compression testing.";
    
    let metrics = profiler.profile_compression(
        CompressionAlgorithm::Zstd,
        test_data,
        10
    ).await;
    
    println!("Compression speed: {:.2} MB/s", metrics.compression_speed_mbps);
    println!("Compression ratio: {:.2}%", metrics.compression_ratio * 100.0);
    
    Ok(())
}
```

## Error Handling

All functions return `Result<T, Error>` where `Error` is the main error type. Handle errors appropriately:

```rust
use parallel_mengene_core::error::{Error, Result};

fn process_file(path: &str) -> Result<()> {
    let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, None);
    let data = std::fs::read(path)?; // Propagates I/O errors
    let compressed = ctx.compress(&data)?; // Propagates compression errors
    std::fs::write(format!("{}.compressed", path), compressed)?;
    Ok(())
}
```

## Performance Considerations

1. **Memory Usage**: Large files use memory mapping to reduce memory consumption
2. **Parallel Processing**: Multiple CPU cores are used for compression/decompression
3. **Chunk Size**: Optimal chunk sizes are calculated based on file size and available memory
4. **Algorithm Selection**: Choose algorithms based on your speed vs. compression ratio requirements

## Thread Safety

All public APIs are thread-safe and can be used from multiple threads concurrently. The library uses internal synchronization to ensure thread safety.

## Dependencies

- **Core**: `lz4_flex`, `flate2`, `zstd`, `rayon`, `memmap2`
- **Pipeline**: `tokio`, `crossbeam-channel`, `byteorder`
- **CLI**: `clap`, `tokio`

## License

This project is licensed under the MIT License - see the LICENSE file for details.
