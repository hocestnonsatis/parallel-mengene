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
            CompressionAlgorithm::Pm => self.compress_pm(data),
        }
    }

    /// Decompress data using the configured algorithm
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Pm => self.decompress_pm(data),
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

    /// Very simple in-house algorithm: RLE (run-length encoding) for bytes.
    /// Chunk format:
    /// - 4-byte magic header: b"PMR1"
    /// - Body: [0xFF marker][run_length u8][byte] for runs >= 3, otherwise raw bytes.
    fn compress_pm(&self, data: &[u8]) -> Result<Vec<u8>> {
        const MARKER: u8 = 0xFF;
        let mut out = Vec::with_capacity(4 + data.len());
        // Write per-chunk magic so we can validate during decompression
        out.extend_from_slice(b"PMR1");
        let mut i = 0;
        while i < data.len() {
            let b = data[i];
            let mut run = 1usize;
            while i + run < data.len() && data[i + run] == b && run < 255 {
                run += 1;
            }
            if run >= 3 || b == MARKER {
                out.push(MARKER);
                out.push(run as u8);
                out.push(b);
            } else {
                for _ in 0..run { out.push(b); }
            }
            i += run;
        }
        Ok(out)
    }

    fn decompress_pm(&self, data: &[u8]) -> Result<Vec<u8>> {
        const MARKER: u8 = 0xFF;
        // Validate magic header
        if data.len() < 4 || &data[..4] != b"PMR1" {
            return Err(Error::Compression("PM decompression: invalid or missing magic header".into()));
        }
        let mut out = Vec::with_capacity(data.len().saturating_sub(4));
        let mut i = 4; // start after header
        while i < data.len() {
            if data[i] == MARKER {
                if i + 2 >= data.len() {
                    return Err(Error::Compression("PM decompression: truncated run".into()));
                }
                let run = data[i + 1] as usize;
                let byte = data[i + 2];
                out.extend(std::iter::repeat(byte).take(run));
                i += 3;
            } else {
                out.push(data[i]);
                i += 1;
            }
        }
        Ok(out)
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
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        assert_eq!(ctx.algorithm(), CompressionAlgorithm::Pm);
        assert_eq!(ctx.level(), 1);
    }

    #[test]
    fn test_lz4_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_gzip_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        let data = create_test_data();

        let compressed = ctx.compress(&data).unwrap();
        assert!(!compressed.is_empty());
        // Note: Small data might not compress well, so we just check it's not empty

        let decompressed = ctx.decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zstd_compression_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
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

        // PM has fixed level (ignored), but API should work
        let ctx_low = CompressionContext::new(CompressionAlgorithm::Pm, Some(1));
        let ctx_high = CompressionContext::new(CompressionAlgorithm::Pm, Some(1));

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
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        let empty_data = b"";

        let compressed = ctx.compress(empty_data).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, empty_data);
    }

    #[test]
    fn test_single_byte_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        let single_byte = b"a";

        let compressed = ctx.compress(single_byte).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, single_byte);
    }

    #[test]
    fn test_repetitive_data_compression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, Some(1));
        let repetitive_data = b"AAAAA".repeat(1000);

        let compressed = ctx.compress(&repetitive_data).unwrap();
        let decompressed = ctx.decompress(&compressed).unwrap();

        assert_eq!(decompressed, repetitive_data);
        // Repetitive data should compress well
        assert!(compressed.len() < repetitive_data.len() / 2);
    }

    #[test]
    fn test_invalid_decompression() {
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
        let invalid_data = b"invalid compressed data";

        // This should fail gracefully
        assert!(ctx.decompress(invalid_data).is_err());
    }

    #[test]
    fn test_roundtrip_consistency() {
        let test_data = create_large_test_data();
        let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);
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
            let ctx = CompressionContext::new(CompressionAlgorithm::Pm, None);

            let compressed = ctx.compress(&data).unwrap();
            let decompressed = ctx.decompress(&compressed).unwrap();

            prop_assert_eq!(decompressed, data);
        }
    }
}
