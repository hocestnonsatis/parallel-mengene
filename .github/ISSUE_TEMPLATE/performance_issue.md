---
name: ⚡ Performance Issue
about: Report a performance problem or request optimization
title: '[PERF] '
labels: ['performance', 'needs-triage']
assignees: ''
---

## ⚡ Performance Issue Description

A clear and concise description of the performance issue.

## 📊 Current Performance

Describe the current performance metrics:

- **Compression Speed**: [e.g. 100 MB/s]
- **Decompression Speed**: [e.g. 200 MB/s]
- **Compression Ratio**: [e.g. 50%]
- **Memory Usage**: [e.g. 2GB peak]
- **CPU Usage**: [e.g. 80% average]

## 🎯 Expected Performance

Describe the expected or target performance:

- **Target Compression Speed**: [e.g. 500 MB/s]
- **Target Decompression Speed**: [e.g. 1000 MB/s]
- **Target Compression Ratio**: [e.g. 60%]
- **Target Memory Usage**: [e.g. 1GB peak]
- **Target CPU Usage**: [e.g. 60% average]

## 🔄 Steps to Reproduce

Steps to reproduce the performance issue:

1. Create test file: `dd if=/dev/urandom of=test.bin bs=1M count=1000`
2. Run compression: `parallel-mengene compress test.bin output.pmz --algorithm zstd`
3. Measure performance with: `time parallel-mengene compress test.bin output.pmz --algorithm zstd`

## 🖥️ Environment

- **OS**: [e.g. Ubuntu 22.04, Windows 11, macOS 13.0]
- **Rust Version**: [e.g. 1.75.0]
- **Parallel-Mengene Version**: [e.g. v1.0.0]
- **Architecture**: [e.g. x86_64, ARM64]
- **CPU**: [e.g. Intel i7-12700K, AMD Ryzen 7 5800X]
- **Memory**: [e.g. 32GB DDR4]
- **Storage**: [e.g. NVMe SSD, SATA SSD, HDD]

## 📋 Test Data

Describe the test data used:

- **File Size**: [e.g. 1GB, 10GB]
- **Data Type**: [e.g. Random, Text, Binary, Repetitive]
- **File Format**: [e.g. .bin, .txt, .log, .json]

## 📊 Benchmark Results

If you have benchmark results, please include them:

```
# Example benchmark output
Compression time: 5.2s
Decompression time: 2.1s
Compression ratio: 45.2%
Memory peak: 2.1GB
```

## 🔍 Profiling Information

If you have profiling information:

- **Hotspots**: [e.g. CPU hotspots, memory allocations]
- **Bottlenecks**: [e.g. I/O, CPU, Memory]
- **Thread Usage**: [e.g. Single-threaded, Multi-threaded]

## 💡 Suggested Optimizations

Any ideas for potential optimizations:

- [ ] Algorithm optimization
- [ ] Memory usage reduction
- [ ] Parallel processing improvement
- [ ] I/O optimization
- [ ] Cache optimization

## 📝 Additional Context

Add any other context about the performance issue here.

## ✅ Checklist

- [ ] I have searched existing issues to ensure this is not a duplicate
- [ ] I have provided performance metrics
- [ ] I have included test data information
- [ ] I have run benchmarks if possible
