//! CLI argument parsing

use clap::{Parser, Subcommand};
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "parallel-mengene")]
#[command(about = "GPU-accelerated file compression tool - Squeeze it parallel!")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Compress a file or directory
    Compress {
        /// Input file or directory to compress
        input: PathBuf,
        
        /// Output file or directory
        output: PathBuf,
        
        /// Compression algorithm to use
        #[arg(short, long, default_value = "zstd")]
        algorithm: CompressionAlgorithm,
        
        /// Compression level (1-22 for zstd, 1-16 for lz4, 1-9 for gzip)
        #[arg(short, long)]
        level: Option<u32>,
        
        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,
    },
    
    /// Decompress a file or directory
    Decompress {
        /// Input file or directory to decompress
        input: PathBuf,
        
        /// Output file or directory
        output: PathBuf,
        
        /// Compression algorithm to use
        #[arg(short, long, default_value = "zstd")]
        algorithm: CompressionAlgorithm,
        
        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,
    },
    
    /// Benchmark compression algorithms
    Benchmark {
        /// Input file or directory to benchmark
        input: PathBuf,
        
        /// Algorithms to benchmark
        #[arg(short, long, default_values = ["lz4", "gzip", "zstd"])]
        algorithms: Vec<CompressionAlgorithm>,
        
        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,
    },
}

