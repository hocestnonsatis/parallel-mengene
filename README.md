# Parallel-Mengene

<div align="center">

![Parallel-Mengene](https://img.shields.io/badge/Parallel--Mengene-blue?style=for-the-badge)
![Version](https://img.shields.io/badge/version-1.0.4--rc1-green?style=for-the-badge)
![License](https://img.shields.io/badge/license-Unlicense-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?style=for-the-badge&logo=rust)

**Fast LZ4-based file compression tool** - High-speed parallel processing! 🚀

[![CI/CD Pipeline](https://github.com/hocestnonsatis/parallel-mengene/actions/workflows/ci.yml/badge.svg)](https://github.com/hocestnonsatis/parallel-mengene/actions/workflows/ci.yml)


</div>

## 🎯 Overview

Parallel-Mengene is a fast file compression tool that uses LZ4 compression with parallel processing. Built with Rust, it provides a high-performance compression pipeline around the industry-standard LZ4 format.

## ✨ Key Features

- 🚀 **High Performance**: Fast LZ4 compression with parallel processing
- 🔧 **Algorithm**: LZ4 algorithm (industry-standard fast compression)
- ⚡ **Parallel Processing**: Multi-threaded compression using Rayon
- 🧠 **Intelligent Pipeline**: Advanced workload distribution
- 📊 **Memory Efficient**: Memory-mapped files and streaming compression
- 🔒 **Data Integrity**: 100% verified compression/decompression cycles
- 🌍 **Cross-Platform**: Linux and Windows support
- 📈 **Comprehensive Benchmarking**: Advanced performance testing tools
- 🛡️ **Enterprise Security**: Dependency management, CI/CD

## 🏗️ Architecture

The project is organized as a modular Rust workspace with specialized crates:

```
parallel-mengene/
├── crates/
│   ├── parallel-mengene-core/     # Core algorithms and utilities
│   ├── parallel-mengene-cli/      # Command-line interface
│   ├── parallel-mengene-pipeline/ # Parallel processing pipeline
│   └── parallel-mengene-benchmarks/ # Performance testing suite
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
  - Windows: Visual Studio Build Tools (for local builds)

### Installation

#### Option 1: Download Pre-built Binary (Recommended)

1. Go to [Releases](https://github.com/hocestnonsatis/parallel-mengene/releases)
2. Download the appropriate binary for your system:
   - **Linux**: `parallel-mengene` (x86_64)
   - **Windows**: `parallel-mengene.exe` (x86_64)
3. Make it executable:
   ```bash
   chmod +x parallel-mengene
   ```
4. Move to your PATH (optional):
   ```bash
   sudo mv parallel-mengene /usr/local/bin/
   ```

#### Option 2: Build from Source

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
# Compress a file (output auto-generated as .lz4 if not provided)
parallel-mengene compress input.txt
# -> creates: input.txt.lz4

# Compress a directory (creates tar archive first, then compresses)
parallel-mengene compress my_folder
# -> creates: my_folder.lz4

# Compress with explicit output (optional)
parallel-mengene compress input.txt custom_output.lz4

# Decompress a file (output auto-detected)
parallel-mengene decompress input.txt.lz4
# -> creates: input.txt

# Decompress a directory archive
parallel-mengene decompress my_folder.lz4 restored_folder
# -> creates: restored_folder (contains the original directory structure)

# Benchmark (LZ4 algorithm)
parallel-mengene benchmark input.txt

# Get help
parallel-mengene --help
```

#### Automatic Output Rules

- **File Compression**: if no output is provided, the tool produces a file with the same name and adds `.lz4` suffix (e.g., `document.pdf` → `document.pdf.lz4`, `data` → `data.lz4`).
- **Directory Compression**: directories are first archived into a temporary tar file, then compressed with `.lz4` extension (e.g., `my_folder` → `my_folder.lz4`).
- **Decompression**: if the input ends with `.lz4`, the extension is removed (e.g., `archive.lz4` → `archive`). Otherwise, `_decompressed` is appended.


### 🔧 Workflow Features

- ✅ **Security Audit**: `cargo audit`, `cargo deny`, `cargo outdated`
- ✅ **Code Quality**: `cargo fmt`, `cargo clippy`, comprehensive testing
- ✅ **Multi-Platform**: Linux x86_64 builds with cross-compilation support
- ✅ **Release Automation**: Automatic GitHub releases with artifacts
- ✅ **Performance Monitoring**: Built-in benchmarking and profiling

### 📊 Performance Notes

- **LZ4 Algorithm**: Industry-standard fast compression with excellent speed-to-ratio balance
- **Parallel Processing**: Multi-threaded compression using Rayon for optimal performance
- **Memory Mapping**: Efficient handling of large files with memory-mapped I/O
- **Streaming Support**: Process files larger than available RAM
- **Real Performance**: Tested on 1GB files with 881 MB/s compression speed

### ⚠️ Limitations

- **Single Algorithm**: Only supports LZ4 compression (though it's very efficient)
- **Speed vs Ratio Trade-off**: LZ4 prioritizes speed over maximum compression ratio
- **No GPU Acceleration**: No GPU processing is implemented
- **Memory Usage**: Uses ~2x input file size during compression
- **Format Dependency**: Uses LZ4 format, requires compatible tools for decompression

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

# Windows (cross-compilation)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## 📚 Documentation

- See this README and the tests in `crates/parallel-mengene-pipeline/tests` and `tests/` for examples.

## 🔧 Advanced Usage

### Compression Options

```bash
# Basic compression (LZ4 algorithm)
parallel-mengene compress input.txt result.lz4

# Custom thread count
parallel-mengene compress input.txt threaded.lz4 --threads 8

# Compress entire directory
parallel-mengene compress my_project

# Compress directory with custom output
parallel-mengene compress my_project backup.lz4
```

### Large File Processing

```bash
# Process large files with memory mapping
parallel-mengene compress large_file.bin compressed.lz4

# Stream processing for very large files
parallel-mengene compress huge_file.bin compressed.lz4

# Compress large directory structures
parallel-mengene compress large_dataset

# Compress with custom thread count for better performance
parallel-mengene compress big_folder --threads 16
```

## 🧪 Testing & Quality Assurance

### 🔍 Comprehensive Test Coverage

The project includes extensive testing with **100% success rate**:

- **79+ Tests**: Unit, integration, and performance tests
- **Cross-Platform**: Linux, Windows compatibility  
- **Data Integrity**: 100% verified compression/decompression cycles
- **Performance**: Automated benchmarking and profiling with real LZ4 metrics
- **Security**: Dependency scanning and vulnerability checks
- **Large File Testing**: Successfully tested with 1GB files

### 🚀 CI/CD Test Results

| Test Category | Status | Coverage | Performance |
|---------------|--------|----------|-------------|
| **Unit Tests** | ✅ 100% | 36 tests | < 1 second |
| **Integration Tests** | ✅ 100% | 8 tests | < 2 seconds |
| **Performance Tests** | ✅ 100% | 8 tests | < 3 seconds |
| **Benchmark Tests** | ✅ 100% | 12 tests | < 1 second |
| **Pipeline Tests** | ✅ 100% | 15 tests | < 1 second |
| **Security Audit** | ✅ 100% | All dependencies | < 30 seconds |
| **Code Quality** | ✅ 100% | Format + Clippy | < 10 seconds |

### 📊 Test Execution Summary

```
✅ All code quality checks passed!
   Running unittests src/lib.rs (parallel_mengene_benchmarks)
   test result: ok. 12 passed; 0 failed; 0 ignored
   
   Running unittests src/lib.rs (parallel_mengene_core)  
   test result: ok. 36 passed; 0 failed; 0 ignored
   
   Running unittests src/lib.rs (parallel_mengene_pipeline)
   test result: ok. 15 passed; 0 failed; 0 ignored
   
   Running tests/integration_tests.rs
   test result: ok. 8 passed; 0 failed; 0 ignored
   
   Running tests/performance_profiler.rs
   test result: ok. 8 passed; 0 failed; 0 ignored
```

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

### ✅ Completed (v1.0.2)
- **LZ4 Algorithm**: Industry-standard fast compression implementation
- **Performance Upgrade**: 881 MB/s compression speed on 1GB files
- **Data Integrity**: 100% verified compression/decompression cycles
- **Parallel Processing**: Multi-threaded compression using Rayon
- **Memory Optimization**: Memory-mapped files and streaming compression
- **Comprehensive Testing**: 79+ tests with 100% success rate
- **CLI Interface**: Updated for LZ4 format (.lz4 files)
- **Cross-Platform**: Linux and Windows support
- **CI/CD Pipeline**: Automated testing and quality assurance

### 🔄 In Progress
- Performance optimization for larger files
- Additional compression algorithms (Gzip, Zstd)
- Enhanced benchmarking tools

### 📋 Planned
- Multi-algorithm support
- Web interface
- Docker support
- Custom algorithm plugins
- Distributed compression
- Cloud integration
- Enterprise features

## 📄 License

This project is released into the public domain under **The Unlicense**. See `LICENSE` or https://unlicense.org/ for details.


## 🙏 Acknowledgments

- **[LZ4](https://github.com/lz4/lz4)** - Fast compression algorithm
- **[lz4_flex](https://github.com/PolyMeilex/lz4_flex)** - Rust LZ4 implementation
- **[Rayon](https://github.com/rayon-rs/rayon)** - Data parallelism
- **[Clap](https://github.com/clap-rs/clap)** - Command-line parsing
- **[Criterion](https://github.com/bheisler/criterion.rs)** - Benchmarking

## 🚀 LZ4 Performance Highlights

### 📊 Real-World Performance (1GB File Test)
- **Compression Speed**: 881.29 MB/s
- **Decompression Speed**: 399.09 MB/s  
- **Compression Ratio**: 24.60% (1GB → 772MB)
- **Data Integrity**: 100% verified (bit-perfect)
- **Memory Usage**: ~2x input file size during compression
- **Thread Utilization**: 16 parallel threads

### 🎯 Algorithm Benefits
- **Industry Standard**: LZ4 is widely supported and optimized
- **Speed Focus**: Prioritizes compression/decompression speed
- **Good Ratios**: Excellent compression for repetitive data
- **Low CPU Usage**: Efficient algorithm with minimal overhead
- **Cross-Platform**: Works identically on Linux and Windows

## 📊 Project Status

### 🎯 Current Status
- **Version**: v1.0.4-rc1 (Latest Release)
- **Build Status**: ✅ All tests passing (79 tests)
- **Security**: ✅ No vulnerabilities detected
- **Performance**: ✅ Optimized for production use
- **Algorithm**: ✅ LZ4 implementation complete

### 📈 Repository Metrics

![GitHub last commit](https://img.shields.io/github/last-commit/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub pull requests](https://img.shields.io/github/issues-pr/hocestnonsatis/parallel-mengene?style=flat-square)
![GitHub stars](https://img.shields.io/github/stars/hocestnonsatis/parallel-mengene?style=flat-square)

### 🚀 Release Information

- **Latest Release**: [v1.0.4-rc1](https://github.com/hocestnonsatis/parallel-mengene/releases/tag/v1.0.4-rc1)
- **Release Date**: December 2024
- **Binary Downloads**: Available for Linux x86_64 and Windows x86_64
- **Source Code**: Unlicense
- **Major Update**: LZ4 algorithm implementation with significant performance improvements

---

<div align="center">

**Made with ❤️ in Rust**

[Report Bug](https://github.com/hocestnonsatis/parallel-mengene/issues) • [Request Feature](https://github.com/hocestnonsatis/parallel-mengene/issues) • [Documentation](docs/) • [Changelog](CHANGELOG.md)

</div>