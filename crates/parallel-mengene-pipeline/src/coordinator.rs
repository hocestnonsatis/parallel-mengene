//! Pipeline coordinator for managing CPU/GPU workload distribution

// CompressionStrategy is now defined in parallel_pipeline module
use parallel_mengene_core::error::Result;
use std::io::{Read, Seek, Write};
use std::path::Path;

/// Pipeline coordinator for managing parallel compression
///
/// Responsibilities:
/// - Determine optimal compression strategy
/// - Split files into appropriate chunks
/// - Coordinate parallel workload distribution
/// - Combine results from parallel processing
pub struct PipelineCoordinator {
    parallel_threshold_bytes: usize,
    chunk_size_bytes: usize,
}

impl PipelineCoordinator {
    /// Create a new pipeline coordinator
    pub fn new() -> Self {
        Self {
            parallel_threshold_bytes: 10 * 1024 * 1024, // 10MB threshold for parallel processing
            chunk_size_bytes: 4 * 1024 * 1024,          // 4MB chunks for optimal performance
        }
    }

    /// Determine compression strategy based on file characteristics
    pub fn determine_strategy(
        &self,
        file_size: u64,
    ) -> crate::parallel_pipeline::CompressionStrategy {
        if file_size < self.parallel_threshold_bytes as u64 {
            crate::parallel_pipeline::CompressionStrategy::SmallFile
        } else {
            crate::parallel_pipeline::CompressionStrategy::LargeFile
        }
    }

    /// Split data into chunks for parallel processing
    pub fn split_into_chunks(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let mut chunks = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            let chunk_size = std::cmp::min(self.chunk_size_bytes, data.len() - offset);
            let chunk = data[offset..offset + chunk_size].to_vec();
            chunks.push(chunk);
            offset += chunk_size;
        }

        chunks
    }

    /// Combine compressed chunks into final output
    pub async fn combine_chunks(
        &self,
        compressed_chunks: &[Vec<u8>],
        output_path: &Path,
    ) -> Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};

        // Create output file with metadata header
        let mut output_file = std::fs::File::create(output_path)?;

        // Write file header with metadata
        // Magic number (8 bytes): "PMZFILE\0"
        output_file.write_all(b"PMZFILE\0")?;

        // Version (4 bytes)
        output_file.write_u32::<LittleEndian>(1)?;

        // Number of chunks (4 bytes)
        output_file.write_u32::<LittleEndian>(compressed_chunks.len() as u32)?;

        // Chunk size used (4 bytes)
        output_file.write_u32::<LittleEndian>(self.chunk_size_bytes as u32)?;

        // Reserved bytes (16 bytes) for future use
        output_file.write_all(&[0u8; 16])?;

        // Write chunk index table
        let mut chunk_offsets = Vec::new();
        let mut current_offset = 8 + 4 + 4 + 4 + 16; // Header size
        current_offset += (compressed_chunks.len() * 8) as u64; // Index table size

        for chunk in compressed_chunks {
            chunk_offsets.push(current_offset);
            current_offset += chunk.len() as u64;
        }

        // Write chunk offsets
        for offset in &chunk_offsets {
            output_file.write_u64::<LittleEndian>(*offset)?;
        }

        // Write compressed chunks
        for chunk in compressed_chunks {
            output_file.write_all(chunk)?;
        }

        // Write footer with total compressed size
        output_file.write_u64::<LittleEndian>(current_offset)?;

        tracing::info!(
            "Combined {} chunks into output file with {} bytes total",
            compressed_chunks.len(),
            current_offset
        );

        Ok(())
    }

    /// Decompress chunks from a combined output file
    pub async fn decompress_chunks(&self, input_path: &Path) -> Result<Vec<Vec<u8>>> {
        use byteorder::{LittleEndian, ReadBytesExt};

        let mut input_file = std::fs::File::open(input_path)?;

        // Read and validate file header
        let mut magic = [0u8; 8];
        input_file.read_exact(&mut magic)?;
        if &magic != b"PMZFILE\0" {
            return Err(parallel_mengene_core::error::Error::InvalidInput(
                "Invalid file format: missing PMZ magic number".to_string(),
            ));
        }

        // Read metadata
        let _version = input_file.read_u32::<LittleEndian>()?;
        let num_chunks = input_file.read_u32::<LittleEndian>()? as usize;
        let _chunk_size = input_file.read_u32::<LittleEndian>()?;

        // Skip reserved bytes
        let mut reserved = [0u8; 16];
        input_file.read_exact(&mut reserved)?;

        // Read chunk offsets
        let mut chunk_offsets = Vec::new();
        for _ in 0..num_chunks {
            chunk_offsets.push(input_file.read_u64::<LittleEndian>()?);
        }

        // Read compressed chunks
        let mut compressed_chunks = Vec::new();
        for i in 0..num_chunks {
            let chunk_start = chunk_offsets[i] as usize;
            let chunk_end = if i + 1 < num_chunks {
                chunk_offsets[i + 1] as usize
            } else {
                // Last chunk - read to end of file minus footer
                let file_size = std::fs::metadata(input_path)?.len() as usize;
                file_size - 8 // Minus 8 bytes for footer
            };

            let chunk_size = chunk_end - chunk_start;
            let mut chunk_data = vec![0u8; chunk_size];

            input_file.seek(std::io::SeekFrom::Start(chunk_start as u64))?;
            input_file.read_exact(&mut chunk_data)?;

            compressed_chunks.push(chunk_data);
        }

        tracing::info!(
            "Decompressed {} chunks from input file",
            compressed_chunks.len()
        );

        Ok(compressed_chunks)
    }

    /// Get parallel threshold in bytes
    pub fn parallel_threshold(&self) -> usize {
        self.parallel_threshold_bytes
    }

    /// Get chunk size in bytes
    pub fn chunk_size(&self) -> usize {
        self.chunk_size_bytes
    }

    /// Set custom parallel threshold
    pub fn set_parallel_threshold(&mut self, threshold_bytes: usize) {
        self.parallel_threshold_bytes = threshold_bytes;
    }

    /// Set custom chunk size
    pub fn set_chunk_size(&mut self, chunk_size_bytes: usize) {
        self.chunk_size_bytes = chunk_size_bytes;
    }

    /// Calculate optimal number of parallel workers
    pub fn calculate_optimal_workers(&self, file_size: u64) -> usize {
        let num_chunks = (file_size as usize).div_ceil(self.chunk_size_bytes);
        std::cmp::min(num_chunks, num_cpus::get())
    }
}

impl Default for PipelineCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
