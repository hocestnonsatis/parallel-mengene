//! CLI argument parsing

use clap::{Parser, Subcommand};
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "parallel-mengene")]
#[command(
    about = "High-performance parallel file compression tool - Multiple algorithms supported!"
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Compress a file or directory with intelligent algorithm selection
    Compress {
        /// Input file or directory to compress
        input: PathBuf,

        /// Output .pma file (optional - will auto-generate if not provided)
        output: Option<PathBuf>,

        /// Verbose output (print algorithm selection details)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Decompress a .pma file
    Decompress {
        /// Input .pma file to decompress
        input: PathBuf,

        /// Output directory for decompressed files (optional - will auto-generate if not provided)
        output: Option<PathBuf>,

        /// Verbose output (print decompression details)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Benchmark compression algorithms
    Benchmark {
        /// Input file or directory to benchmark
        input: PathBuf,

        /// Algorithms to benchmark (lz4, gzip, zstd)
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
