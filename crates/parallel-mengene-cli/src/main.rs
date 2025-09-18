//! Command-line interface for parallel-mengene
//!
//! A GPU-accelerated file compression tool - Squeeze it parallel!

use clap::Parser;

mod cli;
mod commands;

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
            algorithm,
            level,
            threads,
            verbose,
        } => {
            compress(input, output, algorithm, level, threads, verbose).await?;
        }
        cli::Command::Decompress {
            input,
            output,
            algorithm,
            threads,
            verbose,
        } => {
            decompress(input, output, algorithm, threads, verbose).await?;
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
