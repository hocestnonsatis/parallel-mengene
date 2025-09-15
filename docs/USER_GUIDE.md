# Parallel Mengene User Guide

## Introduction

Parallel Mengene is a high-performance parallel compression library designed for efficient data compression and decompression using multiple CPU cores. It supports multiple compression algorithms and provides both synchronous and asynchronous APIs.

## Installation

### From Source

```bash
git clone https://github.com/your-username/parallel-mengene.git
cd parallel-mengene
cargo build --release
```

### Using Cargo

Add to your `Cargo.toml`:

```toml
[dependencies]
parallel-mengene-core = "0.1.0"
parallel-mengene-pipeline = "0.1.0"
```

## Quick Start

### Basic Compression

```rust
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create compression context
    let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, None);
    
    // Compress data
    let data = b"Hello, World!";
    let compressed = ctx.compress(data)?;
    
    // Decompress data
    let decompressed = ctx.decompress(&compressed)?;
    
    assert_eq!(decompressed, data);
    println!("Compression successful!");
    Ok(())
}
```

### File Compression

```rust
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create parallel pipeline
    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
    
    // Compress file
    pipeline.compress_file(
        Path::new("input.txt"),
        Path::new("output.pmz")
    ).await?;
    
    // Decompress file
    pipeline.decompress_file(
        Path::new("output.pmz"),
        Path::new("decompressed.txt")
    ).await?;
    
    println!("File compression successful!");
    Ok(())
}
```

## Compression Algorithms

### LZ4
- **Speed**: Very fast
- **Compression Ratio**: Moderate
- **Use Case**: Real-time applications, streaming
- **Levels**: 1-16 (default: 1)

```rust
let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, Some(4));
```

### Gzip
- **Speed**: Fast
- **Compression Ratio**: Good
- **Use Case**: General purpose, web applications
- **Levels**: 1-9 (default: 6)

```rust
let ctx = CompressionContext::new(CompressionAlgorithm::Gzip, Some(9));
```

### Zstd
- **Speed**: Fast to moderate
- **Compression Ratio**: Excellent
- **Use Case**: Storage, archival, high compression needs
- **Levels**: 1-22 (default: 3)

```rust
let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, Some(15));
```

## Advanced Usage

### Custom Compression Levels

```rust
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

// High compression (slower but better ratio)
let high_compression = CompressionContext::new(
    CompressionAlgorithm::Zstd, 
    Some(22)
);

// Fast compression (faster but lower ratio)
let fast_compression = CompressionContext::new(
    CompressionAlgorithm::Lz4, 
    Some(1)
);
```

### Memory Mapping for Large Files

```rust
use parallel_mengene_pipeline::memory_mapping::MemoryMappingConfig;
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;

let mut pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();

// Configure memory mapping
let config = MemoryMappingConfig {
    use_memory_mapping: true,
    chunk_size: 1024 * 1024, // 1MB chunks
    max_memory_usage: 1024 * 1024 * 1024, // 1GB max
};

pipeline.configure_memory_mapping(config);
```

### Progress Tracking

```rust
use parallel_mengene_pipeline::parallel_pipeline::CompressionProgress;
use std::time::Instant;

let mut progress = CompressionProgress::new(1000, 10);

// Update progress
progress.update(500, 5);
println!("Progress: {:.1}%", progress.percentage());
println!("Speed: {:.1} MB/s", progress.current_speed_mbps);

if let Some(eta) = progress.eta_seconds() {
    println!("ETA: {:.1} seconds", eta);
}
```

### Performance Profiling

```rust
use crate::performance::PerformanceProfiler;
use parallel_mengene_core::algorithms::CompressionAlgorithm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut profiler = PerformanceProfiler::new();
    
    // Test different algorithms
    let test_data = b"Hello, World! This is a test string for compression testing.";
    
    let lz4_metrics = profiler.profile_compression(
        CompressionAlgorithm::Lz4,
        test_data,
        10
    ).await;
    
    let zstd_metrics = profiler.profile_compression(
        CompressionAlgorithm::Zstd,
        test_data,
        10
    ).await;
    
    // Compare results
    println!("LZ4 Speed: {:.2} MB/s", lz4_metrics.compression_speed_mbps);
    println!("Zstd Speed: {:.2} MB/s", zstd_metrics.compression_speed_mbps);
    
    // Generate report
    let report = profiler.generate_report();
    println!("{}", report);
    
    Ok(())
}
```

## Command Line Interface

### Basic Usage

```bash
# Compress a file
parallel-mengene compress input.txt output.pmz

# Decompress a file
parallel-mengene decompress input.pmz output.txt

# Show help
parallel-mengene --help
```

### Advanced Options

```bash
# Compress with specific algorithm and level
parallel-mengene compress input.txt output.pmz \
    --algorithm zstd \
    --level 15 \
    --threads 8

# Enable memory mapping for large files
parallel-mengene compress large_file.txt output.pmz \
    --memory-mapping \
    --verbose

# Decompress with progress tracking
parallel-mengene decompress input.pmz output.txt \
    --verbose
```

### Batch Processing

```bash
# Compress multiple files
for file in *.txt; do
    parallel-mengene compress "$file" "${file%.txt}.pmz"
done

# Decompress multiple files
for file in *.pmz; do
    parallel-mengene decompress "$file" "${file%.pmz}.txt"
done
```

## Best Practices

### Algorithm Selection

1. **LZ4**: Use for real-time applications, streaming, or when speed is critical
2. **Gzip**: Use for general purpose compression, web applications, or when you need good balance
3. **Zstd**: Use for storage, archival, or when compression ratio is important

### Compression Levels

1. **Level 1-3**: Fast compression, moderate ratio
2. **Level 4-6**: Balanced speed and ratio
3. **Level 7-9**: Higher compression, slower speed
4. **Level 10+**: Maximum compression, slowest speed

### Memory Management

1. **Small files (< 1MB)**: Use regular I/O
2. **Medium files (1MB - 100MB)**: Use memory mapping
3. **Large files (> 100MB)**: Use streaming compression

### Threading

1. **CPU-bound tasks**: Use all available cores
2. **I/O-bound tasks**: Use fewer cores to avoid context switching
3. **Memory-constrained systems**: Reduce thread count

## Troubleshooting

### Common Issues

#### "Out of memory" errors
- Reduce chunk size
- Enable memory mapping
- Use streaming compression for very large files

#### Slow compression
- Check if you're using the right algorithm
- Verify thread count is appropriate
- Consider using a lower compression level

#### Poor compression ratio
- Try a different algorithm
- Increase compression level
- Check if data is already compressed

#### Decompression errors
- Verify the compressed file is not corrupted
- Ensure you're using the same algorithm for decompression
- Check file permissions

### Performance Optimization

1. **Profile your data**: Use the performance profiler to find the best algorithm
2. **Tune chunk size**: Adjust based on your data characteristics
3. **Monitor memory usage**: Use memory mapping for large files
4. **Choose appropriate threads**: Balance between CPU cores and memory usage

### Debugging

Enable verbose output to see detailed information:

```rust
use tracing::Level;
use tracing_subscriber;

// Initialize logging
tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();

// Your compression code here
```

## Examples

### Complete File Compression Example

```rust
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create test data
    let test_data = b"Hello, World! This is a test string for compression testing.";
    fs::write("input.txt", test_data)?;
    
    // Create pipeline
    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
    
    // Compress
    println!("Compressing...");
    pipeline.compress_file(Path::new("input.txt"), Path::new("output.pmz")).await?;
    
    // Get file sizes
    let input_size = fs::metadata("input.txt")?.len();
    let output_size = fs::metadata("output.pmz")?.len();
    let ratio = output_size as f64 / input_size as f64;
    
    println!("Input size: {} bytes", input_size);
    println!("Output size: {} bytes", output_size);
    println!("Compression ratio: {:.2}%", ratio * 100.0);
    
    // Decompress
    println!("Decompressing...");
    pipeline.decompress_file(Path::new("output.pmz"), Path::new("decompressed.txt")).await?;
    
    // Verify
    let decompressed = fs::read("decompressed.txt")?;
    assert_eq!(decompressed, test_data);
    println!("Verification successful!");
    
    // Clean up
    fs::remove_file("input.txt")?;
    fs::remove_file("output.pmz")?;
    fs::remove_file("decompressed.txt")?;
    
    Ok(())
}
```

### Performance Comparison Example

```rust
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_data = b"Hello, World! This is a test string for compression testing.".repeat(1000);
    
    let algorithms = [
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ];
    
    for algorithm in algorithms {
        let ctx = CompressionContext::new(algorithm, None);
        
        // Measure compression time
        let start = Instant::now();
        let compressed = ctx.compress(&test_data)?;
        let compression_time = start.elapsed();
        
        // Measure decompression time
        let start = Instant::now();
        let decompressed = ctx.decompress(&compressed)?;
        let decompression_time = start.elapsed();
        
        // Calculate metrics
        let ratio = compressed.len() as f64 / test_data.len() as f64;
        let compression_speed = test_data.len() as f64 / compression_time.as_secs_f64() / 1_048_576.0;
        let decompression_speed = test_data.len() as f64 / decompression_time.as_secs_f64() / 1_048_576.0;
        
        println!("Algorithm: {:?}", algorithm);
        println!("  Compression ratio: {:.2}%", ratio * 100.0);
        println!("  Compression speed: {:.2} MB/s", compression_speed);
        println!("  Decompression speed: {:.2} MB/s", decompression_speed);
        println!();
    }
    
    Ok(())
}
```

## Support

For questions, issues, or contributions, please visit our GitHub repository or contact the maintainers.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
