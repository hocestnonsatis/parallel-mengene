//! Command-line interface for parallel-mengene
//! 
//! A GPU-accelerated file compression tool - Squeeze it parallel!

use clap::Parser;

mod cli;
mod commands;

use cli::Cli;
use commands::{compress, decompress, benchmark};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Compress { input, output, algorithm, level, threads } => {
            compress(input, output, algorithm, level, threads).await?;
        }
        cli::Command::Decompress { input, output, algorithm, threads } => {
            decompress(input, output, algorithm, threads).await?;
        }
        cli::Command::Benchmark { input, algorithms, threads } => {
            benchmark(input, algorithms, threads).await?;
        }
    }

    Ok(())
}
