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
            CompressionAlgorithm::Gzip => self.compress_gzip(data),
            CompressionAlgorithm::Zstd => self.compress_zstd(data),
        }
    }

    /// Decompress data using the configured algorithm
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Lz4 => self.decompress_lz4(data),
            CompressionAlgorithm::Gzip => self.decompress_gzip(data),
            CompressionAlgorithm::Zstd => self.decompress_zstd(data),
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

    fn compress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        use lz4_flex::compress;
        Ok(compress(data))
    }

    fn decompress_lz4(&self, data: &[u8]) -> Result<Vec<u8>> {
        // For LZ4, we need to know the original size. Since we don't have it,
        // we'll try with a reasonable estimate and increase if needed
        let mut estimated_size = data.len() * 10; // Start with 10x the compressed size
        loop {
            match lz4_flex::decompress(data, estimated_size) {
                Ok(result) => return Ok(result),
                Err(e) if e.to_string().contains("too small") => {
                    estimated_size *= 2;
                    if estimated_size > data.len() * 1000 {
                        // Safety limit
                        return Err(Error::Compression(
                            "LZ4 decompression: unable to determine original size".to_string(),
                        ));
                    }
                }
                Err(e) => {
                    return Err(Error::Compression(format!(
                        "LZ4 decompression error: {}",
                        e
                    )))
                }
            }
        }
    }

    fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(self.level));
        encoder
            .write_all(data)
            .map_err(|e| Error::Compression(format!("Gzip compression error: {}", e)))?;
        encoder
            .finish()
            .map_err(|e| Error::Compression(format!("Gzip compression finish error: {}", e)))
    }

    fn decompress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder
            .read_to_end(&mut result)
            .map_err(|e| Error::Compression(format!("Gzip decompression error: {}", e)))?;
        Ok(result)
    }

    fn compress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::encode_all(data, self.level as i32)
            .map_err(|e| Error::Compression(format!("Zstd compression error: {}", e)))
    }

    fn decompress_zstd(&self, data: &[u8]) -> Result<Vec<u8>> {
        zstd::decode_all(data)
            .map_err(|e| Error::Compression(format!("Zstd decompression error: {}", e)))
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
        assert_eq!(ctx.level(), 1); // Default level for LZ4

        let ctx = CompressionContext::new(CompressionAlgorithm::Gzip, Some(9));
        assert_eq!(ctx.algorithm(), CompressionAlgorithm::Gzip);
        assert_eq!(ctx.level(), 9);
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
    fn test_gzip_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Gzip, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zstd_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, None);
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

        // Test LZ4 with different levels
        let ctx_low = CompressionContext::new(CompressionAlgorithm::Lz4, Some(1));
        let ctx_high = CompressionContext::new(CompressionAlgorithm::Lz4, Some(16));

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
        let ctx = CompressionContext::new(CompressionAlgorithm::Gzip, None);
        let single_byte = b"a";

        let compressed = ctx.compress(single_byte).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, single_byte);
    }

    #[test]
    fn test_repetitive_data_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, Some(22));
        let repetitive_data = b"AAAAA".repeat(1000);

        let compressed = ctx.compress(&repetitive_data).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, repetitive_data);
        // Repetitive data should compress well
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
        let algorithms = [
            CompressionAlgorithm::Lz4,
            CompressionAlgorithm::Gzip,
            CompressionAlgorithm::Zstd,
        ];

        let test_data = create_large_test_data();

        for algorithm in algorithms {
            let ctx = CompressionContext::new(algorithm, None);

            // Multiple rounds of compression/decompression
            let mut data = test_data.clone();
            for _ in 0..3 {
                let compressed = ctx.compress(&data).unwrap();
                data = ctx.decompress(&compressed).unwrap();
            }

            assert_eq!(data, test_data);
        }
    }

    proptest! {
        #[test]
        fn test_compression_roundtrip_property(data in prop::collection::vec(any::<u8>(), 0..10000)) {
            let ctx = CompressionContext::new(CompressionAlgorithm::Zstd, None);

            let compressed = ctx.compress(&data).unwrap();
            let decompressed = ctx.decompress(&compressed).unwrap();

            prop_assert_eq!(decompressed, data);
        }
    }
}
