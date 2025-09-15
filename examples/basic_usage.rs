//! Basic usage example for parallel-mengene

use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::utils::{compression_ratio, space_savings, format_file_size};

fn main() -> anyhow::Result<()> {
    // Sample data to compress
    let data = b"Hello, world! This is a test of the parallel-mengene compression system. ".repeat(1000);
    
    println!("Original data size: {}", format_file_size(data.len() as u64));
    
    // Test different compression algorithms
    let algorithms = vec![
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ];
    
    for algorithm in algorithms {
        println!("\n--- Testing {:?} ---", algorithm);
        
        // Create compression context
        let context = CompressionContext::new(algorithm, None);
        
        // Compress data
        let compressed = context.compress(&data)?;
        println!("Compressed size: {}", format_file_size(compressed.len() as u64));
        
        // Calculate metrics
        let ratio = compression_ratio(data.len(), compressed.len());
        let savings = space_savings(data.len(), compressed.len());
        
        println!("Compression ratio: {:.3}", ratio);
        println!("Space savings: {:.1}%", savings);
        
        // Decompress data
        let decompressed = context.decompress(&compressed)?;
        
        // Verify data integrity
        if data == decompressed {
            println!("✓ Data integrity verified");
        } else {
            println!("✗ Data integrity check failed!");
        }
    }
    
    Ok(())
}
