#!/usr/bin/env python3
"""
Professional Benchmark Runner for Parallel-Mengene
Comprehensive benchmarking system with multiple test scenarios
"""

import os
import sys
import time
import subprocess
import json
import csv
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
import argparse

class BenchmarkRunner:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.fixtures_dir = project_root / "tests" / "fixtures"
        self.results_dir = project_root / "tests" / "benchmarks" / "results"
        self.reports_dir = project_root / "tests" / "reports"
        
        # Create directories
        self.results_dir.mkdir(parents=True, exist_ok=True)
        self.reports_dir.mkdir(parents=True, exist_ok=True)
        
        # Benchmark configuration
        self.tools = {
            'parallel-mengene': {
                'binary': str(project_root / "target" / "release" / "parallel-mengene"),
                'compress_cmd': 'compress',
                'decompress_cmd': 'decompress',
                'ext': '.pmz'
            },
            'gzip': {
                'binary': 'gzip',
                'compress_cmd': '-c',
                'decompress_cmd': '-d -c',
                'ext': '.gz'
            },
            'bzip2': {
                'binary': 'bzip2',
                'compress_cmd': '-c',
                'decompress_cmd': '-d -c',
                'ext': '.bz2'
            },
            'xz': {
                'binary': 'xz',
                'compress_cmd': '-c',
                'decompress_cmd': '-d -c',
                'ext': '.xz'
            },
            'zstd': {
                'binary': 'zstd',
                'compress_cmd': '-c -19',
                'decompress_cmd': '-d -c',
                'ext': '.zst'
            },
            'lz4': {
                'binary': 'lz4',
                'compress_cmd': '-c',
                'decompress_cmd': '-d -c',
                'ext': '.lz4'
            }
        }
        
        # Test scenarios
        self.scenarios = {
            'quick': {
                'sizes': ['small'],
                'types': ['repetitive', 'random'],
                'files_per_type': 1
            },
            'standard': {
                'sizes': ['small', 'medium'],
                'types': ['repetitive', 'random', 'text'],
                'files_per_type': 2
            },
            'comprehensive': {
                'sizes': ['small', 'medium', 'large'],
                'types': ['repetitive', 'random', 'text', 'binary', 'mixed'],
                'files_per_type': 3
            }
        }

    def check_tools(self) -> List[str]:
        """Check which compression tools are available"""
        available = []
        for tool, config in self.tools.items():
            if tool == 'parallel-mengene':
                if os.path.exists(config['binary']):
                    available.append(tool)
            else:
                try:
                    subprocess.run([config['binary'], '--version'], 
                                 capture_output=True, check=True)
                    available.append(tool)
                except (subprocess.CalledProcessError, FileNotFoundError):
                    print(f"Warning: {tool} not available")
        return available

    def get_file_size(self, filepath: Path) -> int:
        """Get file size in bytes"""
        return filepath.stat().st_size

    def measure_compression(self, tool: str, input_file: Path, 
                          output_file: Path) -> Tuple[float, int, float]:
        """Measure compression performance"""
        config = self.tools[tool]
        
        # Build command
        if tool == 'parallel-mengene':
            cmd = [config['binary'], config['compress_cmd'], 
                   str(input_file), str(output_file)]
        else:
            cmd = [config['binary']] + config['compress_cmd'].split() + [str(input_file)]
        
        # Measure time and size
        start_time = time.time()
        try:
            with open(output_file, 'wb') as f:
                result = subprocess.run(cmd, stdout=f, stderr=subprocess.DEVNULL, 
                                      check=True, timeout=300)
            end_time = time.time()
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired) as e:
            print(f"Error compressing with {tool}: {e}")
            return 0.0, 0, 0.0
        
        duration = end_time - start_time
        input_size = self.get_file_size(input_file)
        output_size = self.get_file_size(output_file)
        compression_ratio = (output_size / input_size) * 100
        speed = (input_size / duration) / (1024 * 1024)  # MB/s
        
        return duration, output_size, speed, compression_ratio

    def measure_decompression(self, tool: str, compressed_file: Path, 
                            output_file: Path) -> Tuple[float, float]:
        """Measure decompression performance"""
        config = self.tools[tool]
        
        # Build command
        if tool == 'parallel-mengene':
            cmd = [config['binary'], config['decompress_cmd'], 
                   str(compressed_file), str(output_file)]
        else:
            cmd = [config['binary']] + config['decompress_cmd'].split() + [str(compressed_file)]
        
        # Measure time
        start_time = time.time()
        try:
            with open(output_file, 'wb') as f:
                result = subprocess.run(cmd, stdout=f, stderr=subprocess.DEVNULL, 
                                      check=True, timeout=300)
            end_time = time.time()
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired) as e:
            print(f"Error decompressing with {tool}: {e}")
            return 0.0, 0.0
        
        duration = end_time - start_time
        output_size = self.get_file_size(output_file)
        speed = (output_size / duration) / (1024 * 1024)  # MB/s
        
        return duration, speed

    def verify_integrity(self, original: Path, decompressed: Path) -> bool:
        """Verify data integrity after compression/decompression"""
        try:
            with open(original, 'rb') as f1, open(decompressed, 'rb') as f2:
                return f1.read() == f2.read()
        except Exception:
            return False

    def run_benchmark_scenario(self, scenario: str, tools: List[str]) -> Dict:
        """Run a specific benchmark scenario"""
        print(f"\n🚀 Running {scenario} benchmark scenario...")
        
        scenario_config = self.scenarios[scenario]
        results = {
            'scenario': scenario,
            'timestamp': datetime.now().isoformat(),
            'tools': tools,
            'results': []
        }
        
        for size_cat in scenario_config['sizes']:
            for file_type in scenario_config['types']:
                type_dir = self.fixtures_dir / size_cat / f"*mb_{file_type}"
                type_dirs = list(self.fixtures_dir.glob(f"{size_cat}/*mb_{file_type}"))
                
                for type_dir in type_dirs:
                    files = list(type_dir.glob("*.bin"))[:scenario_config['files_per_type']]
                    
                    for test_file in files:
                        print(f"  Testing {test_file.name}...")
                        
                        for tool in tools:
                            # Create temporary files
                            compressed_file = self.results_dir / f"{test_file.stem}_{tool}{self.tools[tool]['ext']}"
                            decompressed_file = self.results_dir / f"{test_file.stem}_{tool}_decompressed.bin"
                            
                            try:
                                # Compression
                                comp_duration, comp_size, comp_speed, comp_ratio = self.measure_compression(
                                    tool, test_file, compressed_file)
                                
                                # Decompression
                                decomp_duration, decomp_speed = self.measure_decompression(
                                    tool, compressed_file, decompressed_file)
                                
                                # Verify integrity
                                integrity_ok = self.verify_integrity(test_file, decompressed_file)
                                
                                # Record results
                                result = {
                                    'tool': tool,
                                    'file': test_file.name,
                                    'file_size': self.get_file_size(test_file),
                                    'compression_time': comp_duration,
                                    'compressed_size': comp_size,
                                    'compression_speed': comp_speed,
                                    'compression_ratio': comp_ratio,
                                    'decompression_time': decomp_duration,
                                    'decompression_speed': decomp_speed,
                                    'integrity_ok': integrity_ok
                                }
                                
                                results['results'].append(result)
                                
                                # Cleanup
                                compressed_file.unlink(missing_ok=True)
                                decompressed_file.unlink(missing_ok=True)
                                
                            except Exception as e:
                                print(f"    Error with {tool}: {e}")
                                continue
        
        return results

    def generate_report(self, results: Dict, output_file: Path):
        """Generate comprehensive benchmark report"""
        report = {
            'summary': self._generate_summary(results),
            'detailed_results': results['results'],
            'performance_analysis': self._analyze_performance(results),
            'recommendations': self._generate_recommendations(results)
        }
        
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        # Also generate CSV for easy analysis
        csv_file = output_file.with_suffix('.csv')
        self._generate_csv(results, csv_file)

    def _generate_summary(self, results: Dict) -> Dict:
        """Generate summary statistics"""
        tools = set(r['tool'] for r in results['results'])
        summary = {}
        
        for tool in tools:
            tool_results = [r for r in results['results'] if r['tool'] == tool]
            
            summary[tool] = {
                'files_tested': len(tool_results),
                'avg_compression_speed': sum(r['compression_speed'] for r in tool_results) / len(tool_results),
                'avg_decompression_speed': sum(r['decompression_speed'] for r in tool_results) / len(tool_results),
                'avg_compression_ratio': sum(r['compression_ratio'] for r in tool_results) / len(tool_results),
                'integrity_success_rate': sum(1 for r in tool_results if r['integrity_ok']) / len(tool_results)
            }
        
        return summary

    def _analyze_performance(self, results: Dict) -> Dict:
        """Analyze performance characteristics"""
        # Implementation for performance analysis
        return {"analysis": "Performance analysis results"}

    def _generate_recommendations(self, results: Dict) -> List[str]:
        """Generate recommendations based on results"""
        return ["Recommendation 1", "Recommendation 2"]

    def _generate_csv(self, results: Dict, csv_file: Path):
        """Generate CSV file for results"""
        with open(csv_file, 'w', newline='') as f:
            if results['results']:
                writer = csv.DictWriter(f, fieldnames=results['results'][0].keys())
                writer.writeheader()
                writer.writerows(results['results'])

    def run_benchmarks(self, scenario: str = 'standard', tools: Optional[List[str]] = None):
        """Run comprehensive benchmarks"""
        print("🔧 Parallel-Mengene Professional Benchmark Suite")
        print("=" * 50)
        
        # Check available tools
        available_tools = self.check_tools()
        if not available_tools:
            print("❌ No compression tools available!")
            return
        
        if tools:
            available_tools = [t for t in tools if t in available_tools]
        
        print(f"📊 Available tools: {', '.join(available_tools)}")
        print(f"🎯 Scenario: {scenario}")
        
        # Run benchmarks
        results = self.run_benchmark_scenario(scenario, available_tools)
        
        # Generate reports
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_file = self.reports_dir / f"benchmark_report_{scenario}_{timestamp}.json"
        self.generate_report(results, report_file)
        
        print(f"\n✅ Benchmark complete!")
        print(f"📊 Results saved to: {report_file}")
        print(f"📈 CSV data: {report_file.with_suffix('.csv')}")

def main():
    parser = argparse.ArgumentParser(description='Parallel-Mengene Benchmark Runner')
    parser.add_argument('--scenario', choices=['quick', 'standard', 'comprehensive'], 
                       default='standard', help='Benchmark scenario to run')
    parser.add_argument('--tools', nargs='+', help='Specific tools to test')
    parser.add_argument('--project-root', type=Path, default=Path.cwd().parent,
                       help='Project root directory')
    
    args = parser.parse_args()
    
    runner = BenchmarkRunner(args.project_root)
    runner.run_benchmarks(args.scenario, args.tools)

if __name__ == "__main__":
    main()
