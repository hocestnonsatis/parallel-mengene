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
