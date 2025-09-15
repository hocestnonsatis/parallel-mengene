# Parallel-Mengene

A GPU-accelerated file compression tool - Squeeze it parallel!

## Overview

Parallel-Mengene is a high-performance file compression tool that leverages both CPU parallelism and GPU acceleration to achieve maximum compression speeds. It supports multiple compression algorithms including LZ4, Gzip, and Zstd, with intelligent adaptive selection based on data characteristics.

## Features

- **Multi-Algorithm Support**: LZ4, Gzip, and Zstd compression
- **GPU Acceleration**: Vulkan-based compute shaders for maximum performance
- **CPU Parallelism**: Multi-threaded compression using Rayon
- **Adaptive Compression**: Automatically selects the best algorithm for your data
- **Comprehensive Benchmarking**: Built-in performance testing and analysis tools
- **Cross-Platform**: Works on Windows, Linux, and macOS

## Architecture

The project implements a hybrid CPU/GPU compression pipeline organized as a Rust workspace with the following crates:

- **`parallel-mengene-core`**: Core compression algorithms and data structures
- **`parallel-mengene-cli`**: Command-line interface
- **`parallel-mengene-gpu`**: GPU acceleration with Vulkan compute shaders
- **`parallel-mengene-pipeline`**: Hybrid CPU/GPU workload distribution system

### Hybrid Compression Approach

```
┌─── Input File ────┐
│                   │
├── CPU Pipeline ───┤ ← Metadata, small files
│                   │
└── GPU Pipeline ───┘ ← Large chunks, parallel compression
        │
        ├── Vulkan Compute Shaders
        ├── Memory staging buffers  
        └── Async result collection
```

- **CPU Pipeline**: Handles metadata processing, small files (< 1MB), and coordination
- **GPU Pipeline**: Processes large chunks (> 1MB) using parallel Vulkan compute shaders
- **Pipeline Coordinator**: Intelligently distributes workload based on file characteristics

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Vulkan SDK (for GPU acceleration)
- Git

### Building

```bash
git clone https://github.com/hocestnonsatis/parallel-mengene.git
cd parallel-mengene
cargo build --release
```

### Usage

```bash
# Compress a file
./target/release/parallel-mengene compress input.txt output.pmz

# Decompress a file
./target/release/parallel-mengene decompress output.pmz input.txt

# Benchmark algorithms
./target/release/parallel-mengene benchmark input.txt
```

## Development

### Running Tests

```bash
cargo test
```

### Running Benchmarks

```bash
cargo bench
```

### Building for Specific Platforms

```bash
# Windows
cargo build --release --target x86_64-pc-windows-gnu

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- [Vulkano](https://github.com/vulkano-rs/vulkano) for Vulkan bindings
- [Rayon](https://github.com/rayon-rs/rayon) for data parallelism
- [Clap](https://github.com/clap-rs/clap) for command-line parsing
- [Criterion](https://github.com/bheisler/criterion.rs) for benchmarking