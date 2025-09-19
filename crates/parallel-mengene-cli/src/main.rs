//! Command-line interface for parallel-mengene
//!
//! A GPU-accelerated file compression tool - Squeeze it parallel!

use clap::Parser;

mod cli;
mod commands;
// mod progress; // Not used in current implementation

use cli::Cli;
use commands::{benchmark, compress, decompress};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Compress {
            input,
            output,
            verbose,
        } => {
            compress(input, output, verbose).await?;
        }
        cli::Command::Decompress {
            input,
            output,
            verbose,
        } => {
            decompress(input, output, verbose).await?;
        }
        cli::Command::Benchmark {
            input,
            algorithms,
            threads,
            verbose,
        } => {
            benchmark(input, algorithms, threads, verbose).await?;
        }
    }

    Ok(())
}
