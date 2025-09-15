#!/usr/bin/env python3
"""
Simple Test Runner for Parallel-Mengene
Simplified test runner that actually works
"""

import os
import sys
import subprocess
import argparse
from pathlib import Path
from datetime import datetime

def run_unit_tests():
    """Run unit tests"""
    print("🧪 Running unit tests...")
    
    cmd = ['python3', '-m', 'unittest', 'discover', '-s', 'tests/unit', '-p', 'test_*.py', '-v']
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    print(f"Return code: {result.returncode}")
    if result.stdout:
        print("Output:")
        print(result.stdout)
    if result.stderr:
        print("Errors:")
        print(result.stderr)
    
    return result.returncode == 0

def run_integration_tests():
    """Run integration tests"""
    print("🧪 Running integration tests...")
    
    cmd = ['python3', '-m', 'unittest', 'discover', '-s', 'tests/integration', '-p', 'test_*.py', '-v']
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    print(f"Return code: {result.returncode}")
    if result.stdout:
        print("Output:")
        print(result.stdout)
    if result.stderr:
        print("Errors:")
        print(result.stderr)
    
    return result.returncode == 0

def run_benchmarks():
    """Run benchmark tests"""
    print("🧪 Running benchmark tests...")
    
    benchmark_script = Path("tests/benchmarks/run_benchmarks.py")
    if not benchmark_script.exists():
        print("❌ Benchmark script not found")
        return False
    
    cmd = ['python3', str(benchmark_script), '--scenario', 'quick']
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    print(f"Return code: {result.returncode}")
    if result.stdout:
        print("Output:")
        print(result.stdout)
    if result.stderr:
        print("Errors:")
        print(result.stderr)
    
    return result.returncode == 0

def main():
    parser = argparse.ArgumentParser(description='Simple Parallel-Mengene Test Runner')
    parser.add_argument('--categories', nargs='+', 
                       choices=['unit', 'integration', 'benchmarks'],
                       default=['unit', 'integration'],
                       help='Test categories to run')
    parser.add_argument('--quiet', action='store_true', 
                       help='Reduce output verbosity')
    
    args = parser.parse_args()
    
    print("🚀 Parallel-Mengene Test Runner")
    print("=" * 40)
    
    results = {}
    
    if 'unit' in args.categories:
        results['unit'] = run_unit_tests()
    
    if 'integration' in args.categories:
        results['integration'] = run_integration_tests()
    
    if 'benchmarks' in args.categories:
        results['benchmarks'] = run_benchmarks()
    
    # Summary
    print("\n" + "=" * 40)
    print("📊 TEST SUMMARY")
    print("=" * 40)
    
    passed = sum(1 for success in results.values() if success)
    total = len(results)
    
    for category, success in results.items():
        status = "✅ PASSED" if success else "❌ FAILED"
        print(f"{category.title()}: {status}")
    
    print(f"\nOverall: {passed}/{total} categories passed")
    
    if passed == total:
        print("🎉 All tests passed!")
        sys.exit(0)
    else:
        print("💥 Some tests failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
