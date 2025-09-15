//! CPU pipeline for handling metadata, small files, and coordination

use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::compression::CompressionContext;
use parallel_mengene_core::error::Result;
use std::path::Path;

/// CPU pipeline for compression operations
///
/// Handles:
/// - Small files (< 1MB)
/// - Metadata processing
/// - File coordination and management
pub struct CpuPipeline {
    algorithm: CompressionAlgorithm,
    context: CompressionContext,
}

impl CpuPipeline {
    /// Create a new CPU pipeline
    pub fn new(algorithm: CompressionAlgorithm) -> Result<Self> {
        let context = CompressionContext::new(algorithm, None);
        Ok(Self { algorithm, context })
    }

    /// Compress a file using CPU
    pub async fn compress_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Read input file
        let input_data = std::fs::read(input_path)?;

        // Compress data
        let compressed_data = self.context.compress(&input_data)?;

        // Write output file
        std::fs::write(output_path, compressed_data)?;

        Ok(())
    }

    /// Compress a data chunk using CPU (synchronous)
    pub fn compress_chunk_sync(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.context.compress(data)
    }

    /// Compress a data chunk using CPU (async wrapper)
    pub async fn compress_chunk(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.context.compress(data)
    }

    /// Decompress a data chunk using CPU (async wrapper)
    pub async fn decompress_chunk(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        self.context.decompress(compressed_data)
    }

    /// Decompress a data chunk using CPU (synchronous)
    pub fn decompress_chunk_sync(&self, compressed_data: &[u8]) -> Result<Vec<u8>> {
        self.context.decompress(compressed_data)
    }

    /// Process file metadata
    pub fn process_metadata(&self, file_path: &Path) -> Result<FileMetadata> {
        let metadata = std::fs::metadata(file_path)?;

        Ok(FileMetadata {
            size: metadata.len(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            algorithm: self.algorithm,
            compression_level: None, // Will be set based on algorithm
        })
    }
}

/// File metadata for compression tracking
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub size: u64,
    pub created: Option<std::time::SystemTime>,
    pub modified: Option<std::time::SystemTime>,
    pub algorithm: CompressionAlgorithm,
    pub compression_level: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_data() -> Vec<u8> {
        b"Hello, World! This is a test string for compression testing.".to_vec()
    }

    fn create_large_test_data() -> Vec<u8> {
        let mut data = Vec::new();
        for i in 0..1000 {
            data.extend_from_slice(
                format!("Test data chunk {} with some repetitive content. ", i).as_bytes(),
            );
        }
        data
    }

    #[test]
    fn test_cpu_pipeline_creation() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        assert_eq!(pipeline.algorithm, CompressionAlgorithm::Lz4);
    }

    #[test]
    fn test_cpu_pipeline_creation_all_algorithms() {
        let algorithms = [
            CompressionAlgorithm::Lz4,
            CompressionAlgorithm::Gzip,
            CompressionAlgorithm::Zstd,
        ];

        for algorithm in algorithms {
            let pipeline = CpuPipeline::new(algorithm).unwrap();
            assert_eq!(pipeline.algorithm, algorithm);
        }
    }

    #[tokio::test]
    async fn test_compress_file() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.txt");
        let output_path = temp_dir.path().join("output.pmz");

        // Create test file
        let test_data = create_test_data();
        std::fs::write(&input_path, &test_data).unwrap();

        // Compress file
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        pipeline
            .compress_file(&input_path, &output_path)
            .await
            .unwrap();

        // Verify output file exists
        assert!(output_path.exists());
        let compressed_data = std::fs::read(&output_path).unwrap();
        assert!(!compressed_data.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty
    }

    #[test]
    fn test_compress_chunk_sync() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Gzip).unwrap();
        let test_data = create_test_data();

        let compressed = pipeline.compress_chunk_sync(&test_data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty
    }

    #[tokio::test]
    async fn test_compress_chunk_async() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Zstd).unwrap();
        let test_data = create_test_data();

        let compressed = pipeline.compress_chunk(&test_data).await.unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty
    }

    #[tokio::test]
    async fn test_decompress_chunk() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        let test_data = create_test_data();

        // Compress first
        let compressed = pipeline.compress_chunk_sync(&test_data).unwrap();

        // Decompress
        let decompressed = pipeline.decompress_chunk(&compressed).await.unwrap();
        assert_eq!(decompressed, test_data);
    }

    #[test]
    fn test_decompress_chunk_sync() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Gzip).unwrap();
        let test_data = create_test_data();

        // Compress first
        let compressed = pipeline.compress_chunk_sync(&test_data).unwrap();

        // Decompress
        let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();
        assert_eq!(decompressed, test_data);
    }

    #[test]
    fn test_roundtrip_compression() {
        let algorithms = [
            CompressionAlgorithm::Lz4,
            CompressionAlgorithm::Gzip,
            CompressionAlgorithm::Zstd,
        ];

        let test_data = create_large_test_data();

        for algorithm in algorithms {
            let pipeline = CpuPipeline::new(algorithm).unwrap();

            // Compress
            let compressed = pipeline.compress_chunk_sync(&test_data).unwrap();

            // Decompress
            let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();

            assert_eq!(decompressed, test_data);
        }
    }

    #[test]
    fn test_process_metadata() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");

        // Create test file
        let test_data = create_test_data();
        std::fs::write(&file_path, &test_data).unwrap();

        // Process metadata
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Zstd).unwrap();
        let metadata = pipeline.process_metadata(&file_path).unwrap();

        assert_eq!(metadata.size, test_data.len() as u64);
        assert_eq!(metadata.algorithm, CompressionAlgorithm::Zstd);
        assert!(metadata.created.is_some());
        assert!(metadata.modified.is_some());
    }

    #[test]
    fn test_process_metadata_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");

        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        let result = pipeline.process_metadata(&file_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_data_compression() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        let empty_data = b"";

        let compressed = pipeline.compress_chunk_sync(empty_data).unwrap();
        let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();

        assert_eq!(decompressed, empty_data);
    }

    #[test]
    fn test_single_byte_compression() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Gzip).unwrap();
        let single_byte = b"a";

        let compressed = pipeline.compress_chunk_sync(single_byte).unwrap();
        let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();

        assert_eq!(decompressed, single_byte);
    }

    #[test]
    fn test_repetitive_data_compression() {
        let pipeline = CpuPipeline::new(CompressionAlgorithm::Zstd).unwrap();
        let repetitive_data = b"AAAAA".repeat(1000);

        let compressed = pipeline.compress_chunk_sync(&repetitive_data).unwrap();
        let decompressed = pipeline.decompress_chunk_sync(&compressed).unwrap();

        assert_eq!(decompressed, repetitive_data);
        // Repetitive data should compress well
        assert!(compressed.len() < repetitive_data.len() / 2);
    }

    #[test]
    fn test_file_metadata_debug() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("debug_test.txt");

        std::fs::write(&file_path, b"test").unwrap();

        let pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        let metadata = pipeline.process_metadata(&file_path).unwrap();

        let debug_str = format!("{:?}", metadata);
        assert!(debug_str.contains("FileMetadata"));
        assert!(debug_str.contains("Lz4"));
    }

    #[test]
    fn test_file_metadata_clone() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("clone_test.txt");

        std::fs::write(&file_path, b"test").unwrap();

        let pipeline = CpuPipeline::new(CompressionAlgorithm::Gzip).unwrap();
        let metadata = pipeline.process_metadata(&file_path).unwrap();
        let cloned_metadata = metadata.clone();

        assert_eq!(metadata.size, cloned_metadata.size);
        assert_eq!(metadata.algorithm, cloned_metadata.algorithm);
    }

    #[test]
    fn test_compression_with_different_levels() {
        let test_data = create_large_test_data();

        // Test that different algorithms produce different results
        let lz4_pipeline = CpuPipeline::new(CompressionAlgorithm::Lz4).unwrap();
        let gzip_pipeline = CpuPipeline::new(CompressionAlgorithm::Gzip).unwrap();
        let zstd_pipeline = CpuPipeline::new(CompressionAlgorithm::Zstd).unwrap();

        let lz4_compressed = lz4_pipeline.compress_chunk_sync(&test_data).unwrap();
        let gzip_compressed = gzip_pipeline.compress_chunk_sync(&test_data).unwrap();
        let zstd_compressed = zstd_pipeline.compress_chunk_sync(&test_data).unwrap();

        // All should be different (different algorithms)
        assert_ne!(lz4_compressed, gzip_compressed);
        assert_ne!(gzip_compressed, zstd_compressed);
        assert_ne!(lz4_compressed, zstd_compressed);

        // All should decompress to original data
        assert_eq!(
            lz4_pipeline.decompress_chunk_sync(&lz4_compressed).unwrap(),
            test_data
        );
        assert_eq!(
            gzip_pipeline
                .decompress_chunk_sync(&gzip_compressed)
                .unwrap(),
            test_data
        );
        assert_eq!(
            zstd_pipeline
                .decompress_chunk_sync(&zstd_compressed)
                .unwrap(),
            test_data
        );
    }
}
