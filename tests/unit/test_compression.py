#!/usr/bin/env python3
"""
Unit Tests for Parallel-Mengene Compression
Tests individual components and algorithms
"""

import unittest
import tempfile
import os
from pathlib import Path
import subprocess
import sys

# Add project root to path
project_root = Path(__file__).parent.parent.parent
sys.path.insert(0, str(project_root))

class TestCompressionAlgorithms(unittest.TestCase):
    """Test compression algorithms"""
    
    def setUp(self):
        """Set up test environment"""
        self.binary = project_root / "target" / "release" / "parallel-mengene"
        self.temp_dir = tempfile.mkdtemp()
        self.test_data = b"Hello, World! " * 1000  # Repetitive data for good compression
    
    def tearDown(self):
        """Clean up test environment"""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_compression_basic(self):
        """Test basic compression functionality"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        input_file = Path(self.temp_dir) / "test_input.txt"
        output_file = Path(self.temp_dir) / "test_output.pmz"
        
        # Write test data
        with open(input_file, 'wb') as f:
            f.write(self.test_data)
        
        # Compress
        result = subprocess.run([
            str(self.binary), 'compress', str(input_file), str(output_file)
        ], capture_output=True, text=True)
        
        self.assertEqual(result.returncode, 0, f"Compression failed: {result.stderr}")
        self.assertTrue(output_file.exists(), "Output file not created")
        self.assertLess(output_file.stat().st_size, input_file.stat().st_size, 
                       "Compressed file not smaller than original")
    
    def test_decompression_basic(self):
        """Test basic decompression functionality"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        input_file = Path(self.temp_dir) / "test_input.txt"
        compressed_file = Path(self.temp_dir) / "test_compressed.pmz"
        decompressed_file = Path(self.temp_dir) / "test_decompressed.txt"
        
        # Write test data
        with open(input_file, 'wb') as f:
            f.write(self.test_data)
        
        # Compress
        subprocess.run([
            str(self.binary), 'compress', str(input_file), str(compressed_file)
        ], capture_output=True)
        
        # Decompress
        result = subprocess.run([
            str(self.binary), 'decompress', str(compressed_file), str(decompressed_file)
        ], capture_output=True, text=True)
        
        self.assertEqual(result.returncode, 0, f"Decompression failed: {result.stderr}")
        self.assertTrue(decompressed_file.exists(), "Decompressed file not created")
        
        # Verify data integrity
        with open(decompressed_file, 'rb') as f:
            decompressed_data = f.read()
        
        self.assertEqual(self.test_data, decompressed_data, "Data integrity check failed")
    
    def test_compression_ratio(self):
        """Test compression ratio for different data types"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        test_cases = [
            (b"A" * 10000, "Highly repetitive data"),
            (os.urandom(10000), "Random data"),
            (b"Hello World! " * 1000, "Text with repetition")
        ]
        
        for test_data, description in test_cases:
            with self.subTest(description=description):
                input_file = Path(self.temp_dir) / f"test_{description.replace(' ', '_')}.txt"
                output_file = Path(self.temp_dir) / f"test_{description.replace(' ', '_')}.pmz"
                
                # Write test data
                with open(input_file, 'wb') as f:
                    f.write(test_data)
                
                # Compress
                result = subprocess.run([
                    str(self.binary), 'compress', str(input_file), str(output_file)
                ], capture_output=True)
                
                self.assertEqual(result.returncode, 0, f"Compression failed for {description}")
                
                # Check compression ratio
                original_size = input_file.stat().st_size
                compressed_size = output_file.stat().st_size
                compression_ratio = compressed_size / original_size
                
                print(f"{description}: {compression_ratio:.2%} compression ratio")
                
                # For repetitive data, expect good compression
                if "repetitive" in description:
                    self.assertLess(compression_ratio, 0.5, 
                                   "Repetitive data should compress well")
    
    def test_large_file_handling(self):
        """Test handling of larger files"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        # Create a larger test file (1MB)
        large_data = b"Test data for large file handling. " * 50000
        input_file = Path(self.temp_dir) / "large_test.txt"
        output_file = Path(self.temp_dir) / "large_test.pmz"
        
        with open(input_file, 'wb') as f:
            f.write(large_data)
        
        # Compress
        result = subprocess.run([
            str(self.binary), 'compress', str(input_file), str(output_file)
        ], capture_output=True, text=True, timeout=30)
        
        self.assertEqual(result.returncode, 0, f"Large file compression failed: {result.stderr}")
        self.assertTrue(output_file.exists(), "Large file output not created")

class TestErrorHandling(unittest.TestCase):
    """Test error handling and edge cases"""
    
    def setUp(self):
        """Set up test environment"""
        self.binary = project_root / "target" / "release" / "parallel-mengene"
        self.temp_dir = tempfile.mkdtemp()
    
    def tearDown(self):
        """Clean up test environment"""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_nonexistent_input_file(self):
        """Test handling of nonexistent input file"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        nonexistent_file = Path(self.temp_dir) / "nonexistent.txt"
        output_file = Path(self.temp_dir) / "output.pmz"
        
        result = subprocess.run([
            str(self.binary), 'compress', str(nonexistent_file), str(output_file)
        ], capture_output=True, text=True)
        
        self.assertNotEqual(result.returncode, 0, "Should fail for nonexistent file")
    
    def test_invalid_arguments(self):
        """Test handling of invalid command line arguments"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        result = subprocess.run([
            str(self.binary), 'invalid_command'
        ], capture_output=True, text=True)
        
        self.assertNotEqual(result.returncode, 0, "Should fail for invalid command")
    
    def test_empty_file(self):
        """Test handling of empty files"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        empty_file = Path(self.temp_dir) / "empty.txt"
        output_file = Path(self.temp_dir) / "empty.pmz"
        
        # Create empty file
        empty_file.touch()
        
        result = subprocess.run([
            str(self.binary), 'compress', str(empty_file), str(output_file)
        ], capture_output=True, text=True)
        
        # Should handle empty files gracefully
        self.assertEqual(result.returncode, 0, f"Empty file handling failed: {result.stderr}")

if __name__ == '__main__':
    # Run tests
    unittest.main(verbosity=2)
