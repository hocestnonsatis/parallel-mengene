#!/usr/bin/env python3
"""
Integration Tests for Parallel-Mengene Pipeline
Tests the complete compression pipeline and workflow
"""

import unittest
import tempfile
import os
import time
from pathlib import Path
import subprocess
import sys

# Add project root to path
project_root = Path(__file__).parent.parent.parent
sys.path.insert(0, str(project_root))

class TestCompressionPipeline(unittest.TestCase):
    """Test the complete compression pipeline"""
    
    def setUp(self):
        """Set up test environment"""
        self.binary = project_root / "target" / "release" / "parallel-mengene"
        self.temp_dir = tempfile.mkdtemp()
        
        # Create test data of different types
        self.test_files = {
            'small_repetitive': b"ABC" * 1000,
            'medium_text': b"Hello World! " * 10000,
            'large_binary': os.urandom(100000),  # 100KB
            'mixed_data': b"Text data " + os.urandom(50000) + b" More text"
        }
    
    def tearDown(self):
        """Clean up test environment"""
        import shutil
        shutil.rmtree(self.temp_dir, ignore_errors=True)
    
    def test_end_to_end_compression(self):
        """Test complete compression and decompression workflow"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        for file_type, data in self.test_files.items():
            with self.subTest(file_type=file_type):
                input_file = Path(self.temp_dir) / f"{file_type}_input.bin"
                compressed_file = Path(self.temp_dir) / f"{file_type}_compressed.pmz"
                decompressed_file = Path(self.temp_dir) / f"{file_type}_decompressed.bin"
                
                # Write input data
                with open(input_file, 'wb') as f:
                    f.write(data)
                
                # Compress
                compress_result = subprocess.run([
                    str(self.binary), 'compress', str(input_file), str(compressed_file)
                ], capture_output=True, text=True)
                
                self.assertEqual(compress_result.returncode, 0, 
                               f"Compression failed for {file_type}: {compress_result.stderr}")
                self.assertTrue(compressed_file.exists(), 
                              f"Compressed file not created for {file_type}")
                
                # Decompress
                decompress_result = subprocess.run([
                    str(self.binary), 'decompress', str(compressed_file), str(decompressed_file)
                ], capture_output=True, text=True)
                
                self.assertEqual(decompress_result.returncode, 0, 
                               f"Decompression failed for {file_type}: {decompress_result.stderr}")
                self.assertTrue(decompressed_file.exists(), 
                              f"Decompressed file not created for {file_type}")
                
                # Verify data integrity
                with open(decompressed_file, 'rb') as f:
                    decompressed_data = f.read()
                
                self.assertEqual(data, decompressed_data, 
                               f"Data integrity check failed for {file_type}")
    
    def test_compression_performance(self):
        """Test compression performance characteristics"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        # Test with larger file for performance measurement
        large_data = b"Performance test data " * 50000  # ~1MB
        input_file = Path(self.temp_dir) / "performance_test.bin"
        output_file = Path(self.temp_dir) / "performance_test.pmz"
        
        with open(input_file, 'wb') as f:
            f.write(large_data)
        
        # Measure compression time
        start_time = time.time()
        result = subprocess.run([
            str(self.binary), 'compress', str(input_file), str(output_file)
        ], capture_output=True, text=True)
        end_time = time.time()
        
        self.assertEqual(result.returncode, 0, f"Performance test failed: {result.stderr}")
        
        compression_time = end_time - start_time
        file_size_mb = len(large_data) / (1024 * 1024)
        compression_speed = file_size_mb / compression_time
        
        print(f"Compression speed: {compression_speed:.2f} MB/s")
        
        # Basic performance check (should be reasonable)
        self.assertGreater(compression_speed, 10, "Compression speed too slow")
        self.assertLess(compression_time, 60, "Compression took too long")
    
    def test_multiple_file_types(self):
        """Test compression with different file types"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        file_types = {
            'text': b"This is a text file with some content. " * 1000,
            'json': b'{"key": "value", "array": [1, 2, 3]}' * 100,
            'xml': b'<root><item>data</item></root>' * 1000,
            'csv': b'name,age,city\nJohn,25,NYC\nJane,30,LA' * 100
        }
        
        for file_type, data in file_types.items():
            with self.subTest(file_type=file_type):
                input_file = Path(self.temp_dir) / f"test.{file_type}"
                output_file = Path(self.temp_dir) / f"test.{file_type}.pmz"
                
                with open(input_file, 'wb') as f:
                    f.write(data)
                
                result = subprocess.run([
                    str(self.binary), 'compress', str(input_file), str(output_file)
                ], capture_output=True, text=True)
                
                self.assertEqual(result.returncode, 0, 
                               f"Compression failed for {file_type}: {result.stderr}")
                self.assertTrue(output_file.exists(), 
                              f"Output file not created for {file_type}")
    
    def test_error_recovery(self):
        """Test error recovery and handling"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        # Test with corrupted input (partial data)
        corrupted_data = b"Partial data that might cause issues"
        input_file = Path(self.temp_dir) / "corrupted.bin"
        output_file = Path(self.temp_dir) / "corrupted.pmz"
        
        with open(input_file, 'wb') as f:
            f.write(corrupted_data)
        
        # Should handle gracefully
        result = subprocess.run([
            str(self.binary), 'compress', str(input_file), str(output_file)
        ], capture_output=True, text=True)
        
        # Should either succeed or fail gracefully
        self.assertIn(result.returncode, [0, 1], "Should handle corrupted data gracefully")
    
    def test_concurrent_compression(self):
        """Test concurrent compression operations"""
        if not self.binary.exists():
            self.skipTest("Binary not found")
        
        import threading
        import queue
        
        results = queue.Queue()
        
        def compress_file(file_id, data):
            """Compress a file in a separate thread"""
            input_file = Path(self.temp_dir) / f"concurrent_{file_id}.bin"
            output_file = Path(self.temp_dir) / f"concurrent_{file_id}.pmz"
            
            with open(input_file, 'wb') as f:
                f.write(data)
            
            result = subprocess.run([
                str(self.binary), 'compress', str(input_file), str(output_file)
            ], capture_output=True, text=True)
            
            results.put((file_id, result.returncode == 0))
        
        # Start multiple compression threads
        threads = []
        for i in range(3):
            data = b"Concurrent test data " * (1000 * (i + 1))
            thread = threading.Thread(target=compress_file, args=(i, data))
            threads.append(thread)
            thread.start()
        
        # Wait for all threads to complete
        for thread in threads:
            thread.join()
        
        # Check results
        success_count = 0
        while not results.empty():
            file_id, success = results.get()
            if success:
                success_count += 1
        
        self.assertGreater(success_count, 0, "At least one concurrent compression should succeed")

if __name__ == '__main__':
    # Run integration tests
    unittest.main(verbosity=2)
