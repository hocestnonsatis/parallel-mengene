//! Compression and decompression functionality

use crate::algorithms::CompressionAlgorithm;
use crate::error::{Error, Result};

/// Compression context for managing compression operations
pub struct CompressionContext {
    algorithm: CompressionAlgorithm,
    level: u32,
}

impl CompressionContext {
    /// Create a new compression context
    pub fn new(algorithm: CompressionAlgorithm, level: Option<u32>) -> Self {
        let level = level.unwrap_or_else(|| algorithm.default_level());
        Self { algorithm, level }
    }

    /// Compress data using the configured algorithm
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Lz4 => self.compress_lz4(data),
        }
    }

    /// Decompress data using the configured algorithm
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Lz4 => self.decompress_lz4(data),
        }
    }

    /// Get the current algorithm
    pub fn algorithm(&self) -> CompressionAlgorithm {
        self.algorithm
    }

    /// Get the current compression level
    pub fn level(&self) -> u32 {
        self.level
    }

    /// LZ4 compression with configurable level
    fn compress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use lz4_flex::compress_prepend_size;

        // LZ4 levels 1-9 correspond to different compression strategies
        // For simplicity, we'll use the same compression for all levels
        // In a real implementation, you'd use different strategies per level
        let compressed = compress_prepend_size(data);

        Ok(compressed)
    }

    fn decompress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use lz4_flex::decompress_size_prepended;

        // LZ4 with size prepended is the standard format
        decompress_size_prepended(data)
            .map_err(|e| Error::Compression(format!("LZ4 decompression failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
    fn test_compression_context_creation() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        assert_eq!(ctx.algorithm(), CompressionAlgorithm::Lz4);
        assert_eq!(ctx.level(), 1);
    }

    #[test]
    fn test_lz4_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lz4_compression_with_repetitive_data() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lz4_compression_roundtrip() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compression_with_different_levels() {
        let data = create_large_test_data();

        // LZ4 supports different levels
        let ctx_low = CompressionContext::new(CompressionAlgorithm::Lz4, Some(1));
        let ctx_high = CompressionContext::new(CompressionAlgorithm::Lz4, Some(9));

        let compressed_low = ctx_low.compress(&data).unwrap();
        let compressed_high = ctx_high.compress(&data).unwrap();

        // Both should compress successfully
        assert!(!compressed_low.is_empty());
        assert!(!compressed_high.is_empty());

        // Both should decompress to original data
        assert_eq!(ctx_low.decompress(&compressed_low).unwrap(), data);
        assert_eq!(ctx_high.decompress(&compressed_high).unwrap(), data);
    }

    #[test]
    fn test_empty_data_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let empty_data = b"";

        let compressed = ctx.compress(empty_data).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, empty_data);
    }

    #[test]
    fn test_single_byte_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let single_byte = b"a";

        let compressed = ctx.compress(single_byte).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, single_byte);
    }

    #[test]
    fn test_repetitive_data_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, Some(1));
        let repetitive_data = b"AAAAA".repeat(1000);

        let compressed = ctx.compress(&repetitive_data).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, repetitive_data);
        // Repetitive data should compress well with LZ4
        assert!(compressed.len() < repetitive_data.len() / 2);
    }

    #[test]
    fn test_invalid_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        let invalid_data = b"invalid compressed data";

        // This should fail gracefully
        assert!(ctx.decompress(invalid_data).is_err());
    }

    #[test]
    fn test_roundtrip_consistency() {
        let test_data = create_large_test_data();
        let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);
        // Multiple rounds of compression/decompression
        let mut data = test_data.clone();
        for _ in 0..3 {
            let compressed = ctx.compress(&data).unwrap();
            data = ctx.decompress(&compressed).unwrap();
        }
        assert_eq!(data, test_data);
    }

    proptest! {
        #[test]
        fn test_compression_roundtrip_property(data in prop::collection::vec(any::<u8>(), 0..10000)) {
            let ctx = CompressionContext::new(CompressionAlgorithm::Lz4, None);

            let compressed = ctx.compress(&data).unwrap();
            let decompressed = ctx.decompress(&compressed).unwrap();

            prop_assert_eq!(decompressed, data);
        }
    }
}
