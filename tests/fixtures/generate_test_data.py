#!/usr/bin/env python3
"""
Smart Test Data Generator for Parallel-Mengene
Combines all test data generation methods in one intelligent script
"""

import os
import random
import string
import argparse
from pathlib import Path

def create_dummy_file_truncate(file_path, size_mb):
    """OS-level truncate - fastest for zero-filled files"""
    size_bytes = size_mb * 1024 * 1024
    with open(file_path, 'wb') as f:
        f.truncate(size_bytes)
    print(f"  ✓ Zero file: {file_path.name} ({size_mb} MB)")

def create_dummy_file_seek(file_path, size_mb):
    """OS-level seek - fastest for sparse files"""
    size_bytes = size_mb * 1024 * 1024
    with open(file_path, 'wb') as f:
        f.seek(size_bytes - 1)
        f.write(b'\0')
    print(f"  ✓ Sparse file: {file_path.name} ({size_mb} MB)")

def generate_chunked_data(file_path, size_mb, data_type='random', chunk_size_kb=1024):
    """Memory-efficient chunked generation for real data"""
    size_bytes = size_mb * 1024 * 1024
    chunk_size_bytes = chunk_size_kb * 1024
    
    with open(file_path, 'wb') as f:
        bytes_written = 0
        while bytes_written < size_bytes:
            remaining = size_bytes - bytes_written
            current_chunk_size = min(chunk_size_bytes, remaining)
            
            if data_type == 'random':
                chunk_data = os.urandom(current_chunk_size)
            elif data_type == 'repetitive':
                pattern = b"REPEAT_PATTERN_" * (current_chunk_size // 16)
                chunk_data = pattern[:current_chunk_size]
            elif data_type == 'text':
                text = ''.join(random.choices(string.ascii_letters + ' ', k=current_chunk_size))
                chunk_data = text.encode('utf-8')[:current_chunk_size]
            elif data_type == 'mixed':
                if random.random() < 0.3:
                    pattern = b"MIXED_DATA_" * (current_chunk_size // 11)
                    chunk_data = pattern[:current_chunk_size]
                else:
                    chunk_data = os.urandom(current_chunk_size)
            else:
                chunk_data = os.urandom(current_chunk_size)
            
            f.write(chunk_data)
            bytes_written += current_chunk_size
    
    print(f"  ✓ {data_type.title()} data: {file_path.name} ({size_mb} MB)")

def generate_quick_data(mode='quick'):
    """Generate essential test data quickly"""
    base_dir = Path(__file__).parent
    
    if mode == 'quick':
        # Quick test scenarios - small files only
        test_scenarios = [
            {'size_mb': 1, 'type': 'repetitive', 'method': 'chunked'},
            {'size_mb': 1, 'type': 'random', 'method': 'chunked'},
            {'size_mb': 1, 'type': 'text', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'repetitive', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'random', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'text', 'method': 'chunked'},
            {'size_mb': 50, 'type': 'mixed', 'method': 'chunked'},
        ]
    elif mode == 'fast':
        # Fast test scenarios - mix of methods
        test_scenarios = [
            # Small files - chunked method
            {'size_mb': 1, 'type': 'repetitive', 'method': 'chunked'},
            {'size_mb': 1, 'type': 'random', 'method': 'chunked'},
            {'size_mb': 1, 'type': 'text', 'method': 'chunked'},
            
            # Medium files - chunked method
            {'size_mb': 10, 'type': 'repetitive', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'random', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'text', 'method': 'chunked'},
            {'size_mb': 10, 'type': 'mixed', 'method': 'chunked'},
            
            # Large files - OS level methods
            {'size_mb': 50, 'type': 'zero', 'method': 'truncate'},
            {'size_mb': 50, 'type': 'random', 'method': 'seek'},
            {'size_mb': 100, 'type': 'zero', 'method': 'truncate'},
            {'size_mb': 100, 'type': 'random', 'method': 'seek'},
            {'size_mb': 500, 'type': 'zero', 'method': 'truncate'},
            {'size_mb': 1000, 'type': 'zero', 'method': 'truncate'},
        ]
    else:  # comprehensive
        # Comprehensive test scenarios - all variations
        test_scenarios = []
        sizes = {'small': [1, 5, 10], 'medium': [10, 50], 'large': [100, 500]}
        types = ['repetitive', 'random', 'text', 'binary', 'mixed']
        
        for size_cat, size_list in sizes.items():
            for size_mb in size_list:
                for file_type in types:
                    for i in range(2):  # 2 files per type
                        test_scenarios.append({
                            'size_mb': size_mb,
                            'type': file_type,
                            'method': 'chunked',
                            'suffix': f'_{i+1}'
                        })
    
    print(f"📊 Generating {len(test_scenarios)} test files ({mode} mode)...")
    
    for i, scenario in enumerate(test_scenarios, 1):
        size_mb = scenario['size_mb']
        file_type = scenario['type']
        method = scenario['method']
        suffix = scenario.get('suffix', '')
        
        # Create directory
        dir_path = base_dir / f"{size_mb}mb_{file_type}"
        dir_path.mkdir(parents=True, exist_ok=True)
        
        # Create file
        filename = f"test_{size_mb}mb_{file_type}{suffix}.bin"
        filepath = dir_path / filename
        
        print(f"\n{i:2d}/{len(test_scenarios)} {filename} ({size_mb} MB) - {method.upper()}")
        
        if method == 'truncate':
            create_dummy_file_truncate(filepath, size_mb)
        elif method == 'seek':
            create_dummy_file_seek(filepath, size_mb)
        elif method == 'chunked':
            generate_chunked_data(filepath, size_mb, file_type)
    
    print(f"\n✅ Generated {len(test_scenarios)} test files!")
    return test_scenarios

def generate_benchmark_data():
    """Generate benchmark test data"""
    base_dir = Path(__file__).parent / "benchmark"
    base_dir.mkdir(parents=True, exist_ok=True)
    
    print("\n📊 Generating benchmark test data...")
    
    benchmark_files = [
        {'name': 'test_10mb.bin', 'size': 10, 'method': 'chunked', 'type': 'zero'},
        {'name': 'test_10mb_random.bin', 'size': 10, 'method': 'chunked', 'type': 'random'},
        {'name': 'test_10mb_repetitive.txt', 'size': 10, 'method': 'chunked', 'type': 'repetitive'},
        {'name': 'test_100mb.bin', 'size': 100, 'method': 'truncate', 'type': 'zero'},
        {'name': 'test_1000mb.bin', 'size': 1000, 'method': 'truncate', 'type': 'zero'},
    ]
    
    for file_info in benchmark_files:
        filepath = base_dir / file_info['name']
        size_mb = file_info['size']
        method = file_info['method']
        data_type = file_info['type']
        
        print(f"  {file_info['name']} ({size_mb} MB) - {method.upper()}")
        
        if method == 'truncate':
            create_dummy_file_truncate(filepath, size_mb)
        elif method == 'chunked':
            generate_chunked_data(filepath, size_mb, data_type)
    
    print(f"✅ Generated {len(benchmark_files)} benchmark files!")

def performance_comparison():
    """Compare different generation methods"""
    import time
    
    print("\n⚡ Performance Comparison")
    print("=" * 40)
    
    test_size = 100  # MB
    temp_dir = Path(__file__).parent / "temp_perf_test"
    temp_dir.mkdir(exist_ok=True)
    
    methods = [
        ('truncate', 'OS Truncate'),
        ('seek', 'OS Seek'),
        ('chunked', 'Chunked Write')
    ]
    
    for method, name in methods:
        filepath = temp_dir / f"perf_test_{method}.bin"
        
        start_time = time.time()
        
        if method == 'truncate':
            create_dummy_file_truncate(filepath, test_size)
        elif method == 'seek':
            create_dummy_file_seek(filepath, test_size)
        elif method == 'chunked':
            generate_chunked_data(filepath, test_size, 'random')
        
        end_time = time.time()
        duration = end_time - start_time
        speed = test_size / duration if duration > 0 else float('inf')
        
        print(f"{name:15}: {duration:.2f}s ({speed:.1f} MB/s)")
    
    # Cleanup
    import shutil
    shutil.rmtree(temp_dir, ignore_errors=True)

def main():
    parser = argparse.ArgumentParser(description='Smart Test Data Generator for Parallel-Mengene')
    parser.add_argument('--mode', choices=['quick', 'fast', 'comprehensive'], 
                       default='quick', help='Generation mode')
    parser.add_argument('--benchmark', action='store_true', 
                       help='Also generate benchmark data')
    parser.add_argument('--performance', action='store_true', 
                       help='Show performance comparison')
    parser.add_argument('--clean', action='store_true', 
                       help='Clean existing test data first')
    
    args = parser.parse_args()
    
    print("🔧 Parallel-Mengene Smart Test Data Generator")
    print("=" * 60)
    
    # Clean existing data if requested
    if args.clean:
        print("🧹 Cleaning existing test data...")
        base_dir = Path(__file__).parent
        for item in base_dir.iterdir():
            if item.is_dir() and item.name not in ['benchmark']:
                import shutil
                shutil.rmtree(item, ignore_errors=True)
        print("✅ Cleaned!")
    
    # Generate test data
    print(f"\n🚀 Generating test data ({args.mode} mode)...")
    test_scenarios = generate_quick_data(args.mode)
    
    # Generate benchmark data if requested
    if args.benchmark:
        generate_benchmark_data()
    
    # Show performance comparison if requested
    if args.performance:
        performance_comparison()
    
    # Summary
    print(f"\n🎉 Test data generation complete!")
    print(f"📁 Mode: {args.mode}")
    print(f"📊 Files: {len(test_scenarios)}")
    if args.benchmark:
        print(f"📈 Benchmark data: Generated")
    print(f"💡 Run tests: python3 tests/simple_test_runner.py")

if __name__ == "__main__":
    main()