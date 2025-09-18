//! CLI argument parsing

use clap::{Parser, Subcommand};
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "parallel-mengene")]
#[command(about = "Fast LZ4-based file compression tool - High-speed parallel processing!")]
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

        /// Output file or directory (optional - will auto-generate with .pm extension if not provided)
        output: Option<PathBuf>,

        /// Compression algorithm to use (only 'lz4' is supported)
        #[arg(short, long, default_value = "lz4")]
        algorithm: CompressionAlgorithm,

        /// Compression level (fixed for pm)
        #[arg(short, long)]
        level: Option<u32>,

        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,

        /// Verbose output (print speed and compression ratio)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Decompress a file or directory
    Decompress {
        /// Input file or directory to decompress
        input: PathBuf,

        /// Output file or directory (optional - will auto-generate based on input if not provided)
        output: Option<PathBuf>,

        /// Compression algorithm to use (only 'lz4' is supported)
        #[arg(short, long, default_value = "lz4")]
        algorithm: CompressionAlgorithm,

        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,

        /// Verbose output (print speed)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Benchmark compression algorithms
    Benchmark {
        /// Input file or directory to benchmark
        input: PathBuf,

        /// Algorithms to benchmark (only 'lz4' is supported)
        #[arg(short, long, default_values = ["lz4"])]
        algorithms: Vec<CompressionAlgorithm>,

        /// Number of threads to use
        #[arg(short, long)]
        threads: Option<usize>,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },
}
