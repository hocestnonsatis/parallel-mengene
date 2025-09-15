# Parallel-Mengene Test Suite

Professional testing framework for the Parallel-Mengene compression project.

## 📁 Directory Structure

```
tests/
├── unit/                    # Unit tests for individual components
│   └── test_compression.py  # Compression algorithm tests
├── integration/             # Integration tests for complete workflows
│   └── test_pipeline.py     # Pipeline integration tests
├── benchmarks/              # Performance benchmark tests
│   ├── run_benchmarks.py    # Benchmark runner
│   ├── benchmark_comparison.sh  # Tool comparison script
│   ├── benchmark_analysis.py    # Results analysis
│   └── results/             # Benchmark results
├── fixtures/                # Test data and fixtures
│   ├── generate_test_data.py    # Test data generator
│   ├── small/               # Small test files (1-10MB)
│   ├── medium/              # Medium test files (50-100MB)
│   ├── large/               # Large test files (500MB-1GB)
│   └── [size]mb_[type]/     # Organized by size and data type
├── results/                 # Test execution results
├── reports/                 # Generated test reports
└── run_tests.py            # Main test runner
```

## 🚀 Quick Start

### Run All Tests
```bash
# From project root
python3 tests/run_tests.py

# Run specific test categories
python3 tests/run_tests.py --categories unit integration

# Quiet mode
python3 tests/run_tests.py --quiet
```

### Run Individual Test Categories

#### Unit Tests
```bash
python3 -m unittest discover -s tests/unit -p "test_*.py" -v
```

#### Integration Tests
```bash
python3 -m unittest discover -s tests/integration -p "test_*.py" -v
```

#### Benchmark Tests
```bash
# Quick benchmark
python3 tests/benchmarks/run_benchmarks.py --scenario quick

# Standard benchmark
python3 tests/benchmarks/run_benchmarks.py --scenario standard

# Comprehensive benchmark
python3 tests/benchmarks/run_benchmarks.py --scenario comprehensive
```

## 📊 Test Categories

### Unit Tests (`tests/unit/`)
- **Purpose**: Test individual components and algorithms
- **Scope**: Compression algorithms, error handling, edge cases
- **Examples**: Basic compression, data integrity, error recovery

### Integration Tests (`tests/integration/`)
- **Purpose**: Test complete workflows and system integration
- **Scope**: End-to-end compression pipeline, performance characteristics
- **Examples**: Full compression/decompression cycles, concurrent operations

### Benchmark Tests (`tests/benchmarks/`)
- **Purpose**: Performance testing and comparison with other tools
- **Scope**: Speed, compression ratio, memory usage, scalability
- **Examples**: Tool comparison, performance regression testing

## 🎯 Test Scenarios

### Quick Tests
- Small files only
- Basic functionality
- Fast execution (~1-2 minutes)

### Standard Tests
- Small and medium files
- Multiple data types
- Moderate execution time (~5-10 minutes)

### Comprehensive Tests
- All file sizes (small, medium, large)
- All data types (repetitive, random, text, binary, mixed)
- Multiple files per type
- Long execution time (~30-60 minutes)

## 📈 Test Data Generation

### Smart Test Data Generator (Single Script)
One intelligent script for all test data generation needs:

```bash
# Quick mode (recommended for development)
python3 tests/fixtures/generate_test_data.py --mode quick

# Fast mode (with large files using OS operations)
python3 tests/fixtures/generate_test_data.py --mode fast

# Comprehensive mode (all variations)
python3 tests/fixtures/generate_test_data.py --mode comprehensive

# With benchmark data
python3 tests/fixtures/generate_test_data.py --mode quick --benchmark

# With performance comparison
python3 tests/fixtures/generate_test_data.py --mode fast --performance

# Clean existing data first
python3 tests/fixtures/generate_test_data.py --mode fast --clean
```

**Available Modes:**
- **Quick**: Essential test files (7 files, ~1-50MB)
- **Fast**: Mix of methods including large files (13 files, up to 1GB)
- **Comprehensive**: All variations (30+ files, all sizes)

**Performance Comparison:**
- **OS Truncate**: ~3,000,000 MB/s (instant for zero-filled files)
- **OS Seek**: ~3,000,000 MB/s (instant for sparse files)  
- **Chunked Write**: ~600 MB/s (fast for real data)

**Test Data Structure:**
- **Size categories**: small (1-10MB), medium (10-50MB), large (100-1000MB)
- **Data types**: repetitive, random, text, binary, mixed, zero-filled
- **Multiple files**: 1-3 files per type for statistical significance

## 📋 Test Results

### Results Storage
- **JSON results**: `tests/results/test_results_YYYYMMDD_HHMMSS.json`
- **Test reports**: `tests/reports/test_report_YYYYMMDD_HHMMSS.md`
- **Benchmark data**: `tests/benchmarks/results/`

### Result Analysis
- Performance metrics and trends
- Error analysis and debugging information
- Comparison with previous test runs
- Recommendations for improvements

## 🔧 Configuration

### Environment Variables
```bash
export PARALLEL_MENGENE_BINARY="/path/to/binary"
export TEST_TIMEOUT=300  # seconds
export BENCHMARK_SCENARIO="standard"
```

### Test Configuration
- Modify test parameters in individual test files
- Adjust benchmark scenarios in `run_benchmarks.py`
- Configure test data generation in `generate_test_data.py`

## 🐛 Debugging

### Verbose Output
```bash
python3 tests/run_tests.py --categories unit --quiet=false
```

### Individual Test Debugging
```bash
python3 -m unittest tests.unit.test_compression.TestCompressionAlgorithms.test_compression_basic -v
```

### Benchmark Debugging
```bash
python3 tests/benchmarks/run_benchmarks.py --scenario quick --tools parallel-mengene
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
      - name: Build
        run: cargo build --release
      - name: Run Tests
        run: python3 tests/run_tests.py --categories unit integration
      - name: Run Benchmarks
        run: python3 tests/benchmarks/run_benchmarks.py --scenario quick
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
- [Python unittest Documentation](https://docs.python.org/3/library/unittest.html)
- [Performance Testing Best Practices](https://martinfowler.com/articles/practical-test-pyramid.html)

---

*This test suite ensures the reliability, performance, and correctness of the Parallel-Mengene compression project.*