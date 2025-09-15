# Parallel-Mengene Test Suite

Professional testing framework for the Parallel-Mengene compression project using Rust.

## 📁 Directory Structure

```
tests/
├── fixtures/                # Test data and fixtures
│   ├── benchmark/          # Benchmark test files
│   ├── small/              # Small test files (1-10MB)
│   ├── medium/             # Medium test files (50-100MB)
│   ├── large/              # Large test files (500MB-1GB)
│   └── [size]mb_[type]/    # Organized by size and data type
├── benchmarks/             # Benchmark results and analysis
│   ├── benchmark_comparison.sh  # Tool comparison script
│   └── results/            # Benchmark results
├── integration/            # Integration test results
├── performance/            # Performance profiling results
├── results/                # Test execution results
├── reports/                # Generated test reports
├── integration_tests.rs    # Integration tests
└── README.md              # This file
```

## 🚀 Quick Start

### Run All Tests
```bash
# From project root
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test suite
cargo test --package parallel-mengene-core
cargo test --package parallel-mengene-pipeline
cargo test --package parallel-mengene-benchmarks
```

### Run Integration Tests
```bash
# Run integration tests
cargo test --test integration_tests

# Run with specific test
cargo test --test integration_tests test_compression_pipeline
```

### Run Benchmarks
```bash
# Run performance benchmarks
cargo bench

# Run specific benchmark
cargo bench --package parallel-mengene-benchmarks

# Run benchmark tool directly
cargo run --bin parallel-mengene-bench -- run --algorithms lz4,gzip,zstd
```

## 📊 Test Categories

### Unit Tests (80+ tests)
- **Core Module** (`parallel-mengene-core`): 36 tests
  - Algorithms Module: 9 tests
  - Compression Module: 15 tests  
  - Error Module: 6 tests
  - Utils Module: 6 tests
- **Pipeline Module** (`parallel-mengene-pipeline`): 16 tests
- **Benchmarks Module** (`parallel-mengene-benchmarks`): 12 tests

### Integration Tests (8 tests)
- **File Compression Testing**:
  - Small file compression (1KB)
  - Medium file compression (10MB)
  - Large file compression (100MB)
  - Repetitive data compression (50MB)
  - Empty file compression
  - Multiple compression/decompression cycles
  - Different algorithm testing
  - Error handling

### Performance Tests (8 tests)
- **Performance Profiling**:
  - Profiler creation and configuration
  - Compression performance measurement
  - Scalability testing across different file sizes
  - Algorithm comparison
  - Bottleneck detection
  - Report generation
  - Repetitive data compression analysis

## 🎯 Test Scenarios

### Quick Tests
```bash
cargo test --package parallel-mengene-core
```

### Standard Tests
```bash
cargo test
```

### Comprehensive Tests
```bash
cargo test --all
cargo bench --all
```

## 📈 Test Data Generation

### Using Rust Test Data Generator
```bash
# Generate test data
cargo run --bin parallel-mengene-bench -- generate --output test_data

# Generate with specific sizes
cargo run --bin parallel-mengene-bench -- generate --sizes 1024,1048576,10485760

# Generate with specific types
cargo run --bin parallel-mengene-bench -- generate --types random,repetitive,text
```

### Available Data Types
- **Random**: Random binary data
- **Repetitive**: Highly repetitive patterns
- **Text**: Natural language text
- **Binary**: Binary file data
- **Mixed**: Combination of data types
- **ZeroFilled**: Zero-filled data
- **PatternBased**: Pattern-based data

## 📋 Test Results

### Results Storage
- **JSON results**: `tests/results/test_results_YYYYMMDD_HHMMSS.json`
- **Benchmark reports**: `benchmark_results/benchmark_report.html`
- **Test reports**: `tests/reports/`

### Result Analysis
```bash
# Analyze benchmark results
cargo run --bin parallel-mengene-bench -- analyze --input results.json --charts

# Compare results
cargo run --bin parallel-mengene-bench -- compare --results file1.json,file2.json
```

## 🔧 Configuration

### Environment Variables
```bash
export RUST_LOG=debug                    # Enable debug logging
export PARALLEL_MENGENE_THREADS=8        # Set thread count
export BENCHMARK_TIMEOUT=300             # Benchmark timeout in seconds
```

### Benchmark Configuration
```bash
# Custom benchmark configuration
cargo run --bin parallel-mengene-bench -- run \
  --algorithms lz4,gzip,zstd \
  --iterations 5 \
  --warmup 2 \
  --memory-tracking \
  --cpu-tracking
```

## 🐛 Debugging

### Verbose Output
```bash
cargo test -- --nocapture --test-threads=1
```

### Individual Test Debugging
```bash
cargo test test_name -- --nocapture
```

### Benchmark Debugging
```bash
RUST_LOG=debug cargo run --bin parallel-mengene-bench -- run --algorithms lz4
```

## 📊 Continuous Integration

### GitHub Actions Example
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Tests
        run: cargo test --all
      - name: Run Benchmarks
        run: cargo bench --all
      - name: Run Integration Tests
        run: cargo test --test integration_tests
```

## 🎯 Best Practices

### Writing Tests
1. **Test one thing at a time**: Each test should verify a single behavior
2. **Use descriptive names**: Test names should clearly indicate what's being tested
3. **Clean up resources**: Always clean up temporary files and resources
4. **Handle edge cases**: Test with empty files, large files, invalid data
5. **Verify data integrity**: Always check that compressed/decompressed data matches

### Performance Testing
1. **Use realistic data**: Test with data similar to real-world usage
2. **Measure consistently**: Use the same metrics across all tests
3. **Test scalability**: Verify performance with different file sizes
4. **Compare baselines**: Track performance changes over time

### Maintenance
1. **Keep tests updated**: Update tests when code changes
2. **Remove obsolete tests**: Clean up tests that are no longer relevant
3. **Document test purposes**: Add comments explaining complex test logic
4. **Regular test runs**: Run tests frequently during development

## 📚 Additional Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Performance Testing Best Practices](https://martinfowler.com/articles/practical-test-pyramid.html)

## 🏆 Test Coverage

- **Total Tests**: 80+ tests
- **Unit Tests**: 64 tests (100% pass rate)
- **Integration Tests**: 8 tests (100% pass rate)
- **Performance Tests**: 8 tests (100% pass rate)
- **Overall Coverage**: 100% pass rate

---

*This test suite ensures the reliability, performance, and correctness of the Parallel-Mengene compression project using native Rust testing tools.*