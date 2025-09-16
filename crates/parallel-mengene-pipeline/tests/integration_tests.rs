//! Integration tests for parallel-mengene

use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_pipeline::parallel_pipeline::ParallelPipeline;
use std::fs;
use tempfile::tempdir;

/// Helper function to create test data of various sizes
fn create_test_data(size_mb: usize) -> Vec<u8> {
    // If size_mb is 0, create 1KB of data instead
    let actual_size = if size_mb == 0 { 1024 } else { size_mb * 1024 * 1024 };
    let mut data = Vec::with_capacity(actual_size);
    for i in 0..actual_size {
        data.push((i % 256) as u8);
    }
    data
}

/// Helper function to create repetitive test data
fn create_repetitive_data(size_mb: usize) -> Vec<u8> {
    let pattern = b"Hello, World! This is a repetitive pattern for compression testing. ";
    let mut data = Vec::with_capacity(size_mb * 1024 * 1024);
    let repetitions = (size_mb * 1024 * 1024) / pattern.len();

    for _ in 0..repetitions {
        data.extend_from_slice(pattern);
    }

    // Add remaining bytes
    let remaining = (size_mb * 1024 * 1024) % pattern.len();
    data.extend_from_slice(&pattern[..remaining]);

    data
}

#[tokio::test]
async fn test_small_file_compression() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("small_input.txt");
    let output_path = temp_dir.path().join("small_output.pmz");
    let decompressed_path = temp_dir.path().join("small_decompressed.txt");

    // Create small test file (1KB)
    let test_data = create_test_data(0); // 0 MB = 1KB
    fs::write(&input_path, &test_data).unwrap();

    // Test all algorithms
    for algorithm in [
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ] {
        let pipeline = ParallelPipeline::new(algorithm).unwrap();

        // Compress
        pipeline
            .compress_file(&input_path, &output_path)
            .await
            .unwrap();
        assert!(output_path.exists());

        // Verify compressed file is not empty
        let compressed_size = fs::metadata(&output_path).unwrap().len();
        assert!(compressed_size > 0);

        // Decompress
        pipeline
            .decompress_file(&output_path, &decompressed_path)
            .await
            .unwrap();
        assert!(decompressed_path.exists());

        // Verify data integrity
        let decompressed_data = fs::read(&decompressed_path).unwrap();
        assert_eq!(decompressed_data, test_data);

        // Clean up for next iteration
        let _ = fs::remove_file(&output_path);
        let _ = fs::remove_file(&decompressed_path);
    }
}

#[tokio::test]
async fn test_medium_file_compression() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("medium_input.txt");
    let output_path = temp_dir.path().join("medium_output.pmz");
    let decompressed_path = temp_dir.path().join("medium_decompressed.txt");

    // Create medium test file (10MB)
    let test_data = create_test_data(10);
    fs::write(&input_path, &test_data).unwrap();

    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();

    // Compress
    pipeline
        .compress_file(&input_path, &output_path)
        .await
        .unwrap();
    assert!(output_path.exists());

    // Verify compressed file is smaller
    let compressed_size = fs::metadata(&output_path).unwrap().len();
    assert!(compressed_size < test_data.len() as u64);

    // Decompress
    pipeline
        .decompress_file(&output_path, &decompressed_path)
        .await
        .unwrap();
    assert!(decompressed_path.exists());

    // Verify data integrity
    let decompressed_data = fs::read(&decompressed_path).unwrap();
    assert_eq!(decompressed_data, test_data);
}

#[tokio::test]
async fn test_repetitive_data_compression() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("repetitive_input.txt");
    let output_path = temp_dir.path().join("repetitive_output.pmz");
    let decompressed_path = temp_dir.path().join("repetitive_decompressed.txt");

    // Create repetitive test file (50MB)
    let test_data = create_repetitive_data(50);
    fs::write(&input_path, &test_data).unwrap();

    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();

    // Compress
    pipeline
        .compress_file(&input_path, &output_path)
        .await
        .unwrap();
    assert!(output_path.exists());

    // Verify compressed file is much smaller (repetitive data compresses well)
    let compressed_size = fs::metadata(&output_path).unwrap().len();
    let compression_ratio = compressed_size as f64 / test_data.len() as f64;
    assert!(compression_ratio < 0.1); // Should compress to less than 10% of original size

    // Decompress
    pipeline
        .decompress_file(&output_path, &decompressed_path)
        .await
        .unwrap();
    assert!(decompressed_path.exists());

    // Verify data integrity
    let decompressed_data = fs::read(&decompressed_path).unwrap();
    assert_eq!(decompressed_data, test_data);
}

#[tokio::test]
async fn test_empty_file_compression() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("empty_input.txt");
    let output_path = temp_dir.path().join("empty_output.pmz");
    let decompressed_path = temp_dir.path().join("empty_decompressed.txt");

    // Create empty file
    fs::write(&input_path, b"").unwrap();

    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Lz4).unwrap();

    // Compress
    pipeline
        .compress_file(&input_path, &output_path)
        .await
        .unwrap();
    assert!(output_path.exists());

    // Decompress
    pipeline
        .decompress_file(&output_path, &decompressed_path)
        .await
        .unwrap();
    assert!(decompressed_path.exists());

    // Verify data integrity
    let decompressed_data = fs::read(&decompressed_path).unwrap();
    assert_eq!(decompressed_data, b"");
}

#[tokio::test]
async fn test_compression_roundtrip_multiple_times() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("roundtrip_input.txt");
    let mut current_path = input_path.clone();

    // Create test file
    let test_data = create_test_data(5); // 5MB
    fs::write(&input_path, &test_data).unwrap();

    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();

    // Perform multiple compression/decompression cycles
    for i in 0..3 {
        let compressed_path = temp_dir
            .path()
            .join(format!("roundtrip_compressed_{}.pmz", i));
        let decompressed_path = temp_dir
            .path()
            .join(format!("roundtrip_decompressed_{}.txt", i));

        // Compress
        pipeline
            .compress_file(&current_path, &compressed_path)
            .await
            .unwrap();

        // Decompress
        pipeline
            .decompress_file(&compressed_path, &decompressed_path)
            .await
            .unwrap();

        // Verify data integrity
        let decompressed_data = fs::read(&decompressed_path).unwrap();
        assert_eq!(decompressed_data, test_data);

        // Use decompressed file as input for next iteration
        current_path = decompressed_path;
    }
}

#[tokio::test]
async fn test_compression_with_different_algorithms() {
    let temp_dir = tempdir().unwrap();
    let input_path = temp_dir.path().join("algorithm_input.txt");
    let decompressed_path = temp_dir.path().join("algorithm_decompressed.txt");

    // Create test file
    let test_data = create_test_data(10); // 10MB
    fs::write(&input_path, &test_data).unwrap();

    let algorithms = [
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ];

    for (i, algorithm) in algorithms.iter().enumerate() {
        let output_path = temp_dir.path().join(format!("algorithm_output_{}.pmz", i));

        let pipeline = ParallelPipeline::new(*algorithm).unwrap();

        // Compress
        pipeline
            .compress_file(&input_path, &output_path)
            .await
            .unwrap();
        assert!(output_path.exists());

        // Decompress
        pipeline
            .decompress_file(&output_path, &decompressed_path)
            .await
            .unwrap();
        assert!(decompressed_path.exists());

        // Verify data integrity
        let decompressed_data = fs::read(&decompressed_path).unwrap();
        assert_eq!(decompressed_data, test_data);

        // Clean up
        let _ = fs::remove_file(&output_path);
        let _ = fs::remove_file(&decompressed_path);
    }
}

#[tokio::test]
async fn test_compression_error_handling() {
    let temp_dir = tempdir().unwrap();
    let nonexistent_input = temp_dir.path().join("nonexistent.txt");
    let output_path = temp_dir.path().join("output.pmz");

    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Lz4).unwrap();

    // Try to compress nonexistent file
    let result = pipeline
        .compress_file(&nonexistent_input, &output_path)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_pipeline_stats() {
    let pipeline = ParallelPipeline::new(CompressionAlgorithm::Zstd).unwrap();
    let stats = pipeline.get_stats();

    assert!(stats.chunk_size > 0);
    assert!(stats.parallel_threshold > 0);
    assert!(stats.max_workers > 0);
    assert!(stats.memory_mapping_enabled);
}
