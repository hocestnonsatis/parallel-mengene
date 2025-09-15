//! Memory mapping utilities for efficient large file handling

use memmap2::Mmap;
use parallel_mengene_core::error::Result;
use std::fs::File;
use std::path::Path;

/// Memory-mapped file reader for efficient large file processing
pub struct MemoryMappedFile {
    mmap: Mmap,
    #[allow(dead_code)]
    file: File,
}

impl MemoryMappedFile {
    /// Create a new memory-mapped file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        Ok(Self { mmap, file })
    }
    
    /// Get the memory-mapped data as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.mmap
    }
    
    /// Get the file size
    pub fn len(&self) -> usize {
        self.mmap.len()
    }
    
    /// Check if the file is empty
    pub fn is_empty(&self) -> bool {
        self.mmap.is_empty()
    }
    
    /// Split the memory-mapped data into chunks
    pub fn split_into_chunks(&self, chunk_size: usize) -> Vec<&[u8]> {
        let mut chunks = Vec::new();
        let data = self.as_slice();
        let mut offset = 0;
        
        while offset < data.len() {
            let end = std::cmp::min(offset + chunk_size, data.len());
            chunks.push(&data[offset..end]);
            offset = end;
        }
        
        chunks
    }
    
    /// Get a specific range of the memory-mapped data
    pub fn get_range(&self, start: usize, end: usize) -> Result<&[u8]> {
        if start >= self.len() || end > self.len() || start >= end {
            return Err(parallel_mengene_core::error::Error::InvalidInput(
                "Invalid range for memory-mapped file".to_string()
            ));
        }
        
        Ok(&self.as_slice()[start..end])
    }
}

/// Memory mapping statistics
#[derive(Debug, Clone)]
pub struct MemoryMappingStats {
    pub file_size: usize,
    pub chunk_size: usize,
    pub num_chunks: usize,
    pub memory_usage: usize,
}

impl MemoryMappedFile {
    /// Get memory mapping statistics
    pub fn get_stats(&self, chunk_size: usize) -> MemoryMappingStats {
        let file_size = self.len();
        let num_chunks = (file_size + chunk_size - 1) / chunk_size;
        let memory_usage = file_size; // Memory-mapped files use virtual memory
        
        MemoryMappingStats {
            file_size,
            chunk_size,
            num_chunks,
            memory_usage,
        }
    }
}

/// Memory mapping configuration
#[derive(Debug, Clone)]
pub struct MemoryMappingConfig {
    pub chunk_size: usize,
    pub max_memory_usage: usize,
    pub use_memory_mapping: bool,
}

impl Default for MemoryMappingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 4 * 1024 * 1024, // 4MB chunks
            max_memory_usage: 1024 * 1024 * 1024, // 1GB max
            use_memory_mapping: true,
        }
    }
}

impl MemoryMappingConfig {
    /// Create a new memory mapping configuration
    pub fn new(chunk_size: usize, max_memory_usage: usize) -> Self {
        Self {
            chunk_size,
            max_memory_usage,
            use_memory_mapping: true,
        }
    }
    
    /// Determine if memory mapping should be used for a given file size
    pub fn should_use_memory_mapping(&self, file_size: usize) -> bool {
        self.use_memory_mapping && file_size > self.chunk_size
    }
    
    /// Calculate optimal chunk size based on file size and available memory
    pub fn calculate_optimal_chunk_size(&self, file_size: usize) -> usize {
        if file_size <= self.max_memory_usage {
            // Small file, use single chunk
            file_size
        } else {
            // Large file, use configured chunk size
            self.chunk_size
        }
    }
    
    /// Determine if streaming should be used for a given file size
    pub fn should_use_streaming(&self, file_size: usize) -> bool {
        file_size > self.max_memory_usage * 2 // Use streaming for files larger than 2x max memory
    }
}
