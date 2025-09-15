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
                    if estimated_size > data.len() * 1000 { // Safety limit
                        return Err(Error::Compression("LZ4 decompression: unable to determine original size".to_string()));
                    }
                }
                Err(e) => return Err(Error::Compression(format!("LZ4 decompression error: {}", e))),
            }
        }
    }
    
    fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(self.level as u32));
        encoder.write_all(data)
            .map_err(|e| Error::Compression(format!("Gzip compression error: {}", e)))?;
        encoder.finish()
            .map_err(|e| Error::Compression(format!("Gzip compression finish error: {}", e)))
    }
    
    fn decompress_gzip(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;
        
        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)
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
