#!/usr/bin/env python3
"""
Professional Test Runner for Parallel-Mengene
Runs all tests with proper organization and reporting
"""

import os
import sys
import subprocess
import argparse
from pathlib import Path
from datetime import datetime
import json

class TestRunner:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.tests_dir = project_root / "tests"
        self.results_dir = self.tests_dir / "results"
        self.reports_dir = self.tests_dir / "reports"
        
        # Create directories
        self.results_dir.mkdir(parents=True, exist_ok=True)
        self.reports_dir.mkdir(parents=True, exist_ok=True)
        
        # Test categories
        self.test_categories = {
            'unit': {
                'path': 'tests/unit',
                'description': 'Unit tests for individual components',
                'command': 'python3 -m unittest discover -s {} -p "test_*.py" -v'
            },
            'integration': {
                'path': 'tests/integration', 
                'description': 'Integration tests for complete workflows',
                'command': 'python3 -m unittest discover -s {} -p "test_*.py" -v'
            },
            'benchmarks': {
                'path': self.tests_dir / 'benchmarks',
                'description': 'Performance benchmark tests',
                'command': 'python3 {}'
            }
        }

    def check_prerequisites(self):
        """Check if all prerequisites are met"""
        print("🔍 Checking prerequisites...")
        
        # Check if binary exists
        binary = self.project_root / "target" / "release" / "parallel-mengene"
        if not binary.exists():
            print("❌ Binary not found. Building...")
            result = subprocess.run(['cargo', 'build', '--release'], 
                                  cwd=self.project_root, capture_output=True)
            if result.returncode != 0:
                print(f"❌ Build failed: {result.stderr.decode()}")
                return False
            print("✅ Binary built successfully")
        else:
            print("✅ Binary found")
        
        # Check Python dependencies
        try:
            import unittest
            print("✅ unittest available")
        except ImportError:
            print("❌ unittest not available")
            return False
        
        return True

    def run_test_category(self, category: str, verbose: bool = True) -> dict:
        """Run tests for a specific category"""
        if category not in self.test_categories:
            raise ValueError(f"Unknown test category: {category}")
        
        config = self.test_categories[category]
        print(f"\n🧪 Running {category} tests...")
        print(f"   {config['description']}")
        
        start_time = datetime.now()
        
        if category == 'benchmarks':
            # Special handling for benchmarks
            benchmark_script = config['path'] / 'run_benchmarks.py'
            if benchmark_script.exists():
                cmd = config['command'].format(benchmark_script)
                result = subprocess.run(cmd.split(), cwd=self.project_root, 
                                      capture_output=True, text=True)
            else:
                return {
                    'category': category,
                    'status': 'skipped',
                    'reason': 'Benchmark script not found',
                    'duration': 0
                }
        else:
            # Standard unittest discovery - run from project root
            cmd = config['command'].format(config['path'])
            result = subprocess.run(cmd.split(), cwd=self.project_root, 
                                  capture_output=True, text=True)
        
        end_time = datetime.now()
        duration = (end_time - start_time).total_seconds()
        
        # Parse results
        if result.returncode == 0:
            status = 'passed'
            message = f"All {category} tests passed"
        else:
            status = 'failed'
            message = f"{category} tests failed"
        
        if verbose:
            print(f"   Status: {status}")
            print(f"   Duration: {duration:.2f}s")
            if result.stdout:
                print(f"   Output:\n{result.stdout}")
            if result.stderr:
                print(f"   Errors:\n{result.stderr}")
        
        return {
            'category': category,
            'status': status,
            'message': message,
            'duration': duration,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'returncode': result.returncode
        }

    def run_all_tests(self, categories: list = None, verbose: bool = True) -> dict:
        """Run all tests or specified categories"""
        if not self.check_prerequisites():
            return {
                'status': 'failed', 
                'reason': 'Prerequisites not met',
                'summary': {'overall_status': 'failed'}
            }
        
        if categories is None:
            categories = list(self.test_categories.keys())
        
        print(f"\n🚀 Running tests for categories: {', '.join(categories)}")
        print("=" * 60)
        
        results = {
            'timestamp': datetime.now().isoformat(),
            'categories': {},
            'summary': {}
        }
        
        total_duration = 0
        passed_categories = 0
        
        for category in categories:
            try:
                category_result = self.run_test_category(category, verbose)
                results['categories'][category] = category_result
                total_duration += category_result['duration']
                
                if category_result['status'] == 'passed':
                    passed_categories += 1
                    
            except Exception as e:
                results['categories'][category] = {
                    'category': category,
                    'status': 'error',
                    'message': str(e),
                    'duration': 0
                }
        
        # Generate summary
        results['summary'] = {
            'total_categories': len(categories),
            'passed_categories': passed_categories,
            'failed_categories': len(categories) - passed_categories,
            'total_duration': total_duration,
            'overall_status': 'passed' if passed_categories == len(categories) else 'failed'
        }
        
        # Print summary
        print("\n" + "=" * 60)
        print("📊 TEST SUMMARY")
        print("=" * 60)
        print(f"Total categories: {results['summary']['total_categories']}")
        print(f"Passed: {results['summary']['passed_categories']}")
        print(f"Failed: {results['summary']['failed_categories']}")
        print(f"Total duration: {results['summary']['total_duration']:.2f}s")
        print(f"Overall status: {results['summary']['overall_status'].upper()}")
        
        # Save results
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        results_file = self.results_dir / f"test_results_{timestamp}.json"
        
        with open(results_file, 'w') as f:
            json.dump(results, f, indent=2)
        
        print(f"\n📁 Results saved to: {results_file}")
        
        return results

    def generate_test_report(self, results: dict):
        """Generate a comprehensive test report"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        report_file = self.reports_dir / f"test_report_{timestamp}.md"
        
        with open(report_file, 'w') as f:
            f.write("# Parallel-Mengene Test Report\n\n")
            f.write(f"**Generated:** {results['timestamp']}\n\n")
            
            f.write("## Summary\n\n")
            summary = results['summary']
            f.write(f"- **Total Categories:** {summary['total_categories']}\n")
            f.write(f"- **Passed:** {summary['passed_categories']}\n")
            f.write(f"- **Failed:** {summary['failed_categories']}\n")
            f.write(f"- **Duration:** {summary['total_duration']:.2f}s\n")
            f.write(f"- **Status:** {summary['overall_status'].upper()}\n\n")
            
            f.write("## Detailed Results\n\n")
            for category, result in results['categories'].items():
                f.write(f"### {category.title()} Tests\n\n")
                f.write(f"- **Status:** {result['status'].upper()}\n")
                f.write(f"- **Duration:** {result['duration']:.2f}s\n")
                f.write(f"- **Message:** {result['message']}\n\n")
                
                if result.get('stderr'):
                    f.write("**Errors:**\n```\n")
                    f.write(result['stderr'])
                    f.write("\n```\n\n")
        
        print(f"📋 Report generated: {report_file}")

def main():
    parser = argparse.ArgumentParser(description='Parallel-Mengene Test Runner')
    parser.add_argument('--categories', nargs='+', 
                       choices=['unit', 'integration', 'benchmarks'],
                       help='Specific test categories to run')
    parser.add_argument('--quiet', action='store_true', 
                       help='Reduce output verbosity')
    parser.add_argument('--project-root', type=Path, default=Path.cwd(),
                       help='Project root directory')
    
    args = parser.parse_args()
    
    runner = TestRunner(args.project_root)
    results = runner.run_all_tests(args.categories, not args.quiet)
    
    if not args.quiet:
        runner.generate_test_report(results)
    
    # Exit with appropriate code
    sys.exit(0 if results['summary']['overall_status'] == 'passed' else 1)

if __name__ == "__main__":
    main()
