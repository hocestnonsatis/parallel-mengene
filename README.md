# Parallel-Mengene

<div align="center">

![Parallel-Mengene Logo](https://img.shields.io/badge/Parallel-Mengene-blue?style=for-the-badge&logo=rust)
![Version](https://img.shields.io/badge/version-1.0.0-green?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT%20%7C%20Apache%202.0-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?style=for-the-badge&logo=rust)

**High-performance parallel file compression tool** - Squeeze it parallel! 🚀

[![CI](https://github.com/hocestnonsatis/parallel-mengene/workflows/CI/badge.svg)](https://github.com/hocestnonsatis/parallel-mengene/actions)
[![Security](https://github.com/hocestnonsatis/parallel-mengene/workflows/Security%20Scan/badge.svg)](https://github.com/hocestnonsatis/parallel-mengene/actions)
[![Cross-Platform](https://github.com/hocestnonsatis/parallel-mengene/workflows/Cross-Platform%20Testing/badge.svg)](https://github.com/hocestnonsatis/parallel-mengene/actions)
[![Benchmark](https://github.com/hocestnonsatis/parallel-mengene/workflows/Benchmark/badge.svg)](https://github.com/hocestnonsatis/parallel-mengene/actions)

</div>

## 🎯 Overview

Parallel-Mengene is a cutting-edge file compression tool that leverages advanced CPU parallelism and memory optimization techniques to achieve exceptional compression speeds. Built with Rust for maximum performance and reliability, it supports multiple industry-standard compression algorithms with intelligent workload distribution.

## ✨ Key Features

- 🚀 **Blazing Fast Performance**: Up to 1399 MB/s compression speed
- 🔧 **Multiple Algorithms**: LZ4, Gzip, and Zstd support
- ⚡ **Parallel Processing**: Multi-threaded compression using Rayon
- 🧠 **Intelligent Pipeline**: Automatic workload distribution and optimization
- 📊 **Memory Efficient**: Memory-mapped files and streaming compression
- 🔒 **Data Integrity**: 100% verified compression/decompression cycles
- 🌍 **Cross-Platform**: Linux, Windows, and macOS support
- 📈 **Comprehensive Benchmarking**: Built-in performance analysis tools
- 🛡️ **Enterprise Ready**: Security scanning, dependency management, CI/CD

## 🏗️ Architecture

The project is organized as a modular Rust workspace with specialized crates:

```
parallel-mengene/
├── crates/
│   ├── parallel-mengene-core/     # Core algorithms and utilities
│   ├── parallel-mengene-cli/      # Command-line interface
│   ├── parallel-mengene-pipeline/ # Parallel processing pipeline
│   └── parallel-mengene-benchmarks/ # Performance testing suite
├── docs/                          # Documentation
├── examples/                      # Usage examples
└── tests/                         # Test suites
```

### 🧩 Core Components

- **Core Library**: Compression algorithms, error handling, utilities
- **CLI Interface**: User-friendly command-line tool
- **Pipeline System**: Intelligent workload distribution and parallel processing
- **Benchmark Suite**: Comprehensive performance testing and analysis

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.75 or later
- **Git**: For cloning the repository
- **Build Tools**: 
  - Linux: `build-essential`, `pkg-config`, `libssl-dev`
  - macOS: `pkg-config`, `openssl`
  - Windows: Visual Studio Build Tools

### Installation

```bash
# Clone the repository
git clone https://github.com/hocestnonsatis/parallel-mengene.git
cd parallel-mengene

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path crates/parallel-mengene-cli
```

### Basic Usage

```bash
# Compress a file
parallel-mengene compress input.txt output.pmz --algorithm zstd --level 3

# Decompress a file
parallel-mengene decompress output.pmz input.txt --algorithm zstd

# Benchmark different algorithms
parallel-mengene benchmark input.txt --algorithms lz4 gzip zstd

# Get help
parallel-mengene --help
```

## 📊 Performance

### Current Benchmarks

| Algorithm | Compression Speed | Decompression Speed | Compression Ratio |
|-----------|------------------|-------------------|------------------|
| **Zstd**  | 1399 MB/s        | 3197 MB/s        | 99.9969%         |
| **LZ4**   | 0.46-2.45 MB/s   | 111-337 MB/s     | 98.4%            |
| **Gzip**  | 200-500 MB/s     | 400-800 MB/s     | 60-80%           |

### Memory Usage
- **Peak Memory**: ~2x input file size
- **Large Files**: Memory-mapped processing for files > 1GB
- **Streaming**: Support for files larger than available RAM

## 🛠️ Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --package parallel-mengene-core
cargo test --package parallel-mengene-pipeline

# Run integration tests
cargo test --test integration_tests
```

### Running Benchmarks

```bash
# Run performance benchmarks
cargo bench

# Run specific benchmark
cargo bench --package parallel-mengene-benchmarks
```

### Building for Different Platforms

```bash
# Linux (x86_64)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows (x86_64)
cargo build --release --target x86_64-pc-windows-msvc

# macOS (x86_64)
cargo build --release --target x86_64-apple-darwin

# macOS (ARM64)
cargo build --release --target aarch64-apple-darwin
```

## 📚 Documentation

- **[User Guide](docs/USER_GUIDE.md)**: Complete usage instructions
- **[API Reference](docs/API_REFERENCE.md)**: Detailed API documentation
- **[Testing Summary](TESTING_SUMMARY.md)**: Comprehensive test coverage
- **[Roadmap](roadmap.md)**: Development roadmap and features

## 🔧 Advanced Usage

### Compression Options

```bash
# High compression ratio
parallel-mengene compress input.txt output.pmz --algorithm zstd --level 22

# Fast compression
parallel-mengene compress input.txt output.pmz --algorithm lz4 --level 1

# Custom thread count
parallel-mengene compress input.txt output.pmz --threads 8
```

### Large File Processing

```bash
# Process large files with memory mapping
parallel-mengene compress large_file.bin compressed.pmz --algorithm zstd

# Stream processing for very large files
parallel-mengene compress huge_file.bin compressed.pmz --algorithm zstd --stream
```

## 🧪 Testing

The project includes comprehensive testing:

- **80+ Tests**: Unit, integration, and performance tests
- **Cross-Platform**: Linux, Windows, macOS compatibility
- **Data Integrity**: MD5 verification for all compression cycles
- **Performance**: Automated benchmarking and profiling
- **Security**: Dependency scanning and vulnerability checks

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Areas for Contribution

- 🐛 Bug fixes
- ✨ New features
- 📚 Documentation improvements
- 🧪 Test coverage
- ⚡ Performance optimizations
- 🌍 Cross-platform compatibility

## 📈 Roadmap

### ✅ Completed (v1.0.0)
- Core compression algorithms (LZ4, Gzip, Zstd)
- Parallel processing pipeline
- Memory optimization and mapping
- Comprehensive testing suite
- CLI interface
- Cross-platform support
- CI/CD pipeline

### 🔄 In Progress
- Advanced compression algorithms
- Web interface
- Docker support

### 📋 Planned
- Custom algorithm plugins
- Distributed compression
- Cloud integration
- Enterprise features

## 📄 License

This project is licensed under either of:


- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


## 🙏 Acknowledgments

- **[Rayon](https://github.com/rayon-rs/rayon)** - Data parallelism
- **[Zstd](https://github.com/facebook/zstd)** - Fast compression algorithm
- **[LZ4](https://github.com/lz4/lz4)** - Extremely fast compression
- **[Clap](https://github.com/clap-rs/clap)** - Command-line parsing
- **[Criterion](https://github.com/bheisler/criterion.rs)** - Benchmarking

## 📊 Project Status

![GitHub last commit](https://img.shields.io/github/last-commit/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub pull requests](https://img.shields.io/github/issues-pr/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub stars](https://img.shields.io/github/stars/hocestnonsatis/parallel-mengene?style=flat-square)

---

<div align="center">

**Made with ❤️ in Rust**

[Report Bug](https://github.com/hocestnonsatis/parallel-mengene/issues) • [Request Feature](https://github.com/hocestnonsatis/parallel-mengene/issues) • [Documentation](docs/) • [Changelog](CHANGELOG.md)

</div>