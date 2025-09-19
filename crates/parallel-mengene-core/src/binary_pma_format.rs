//! Binary PMA (Parallel Mengene Archive) file format
//!
//! This module provides an efficient binary format for storing compressed files
//! with metadata. The format is designed for fast reading/writing and minimal
//! overhead compared to JSON-based formats.

use crate::algorithms::CompressionAlgorithm;
use crate::error::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Binary PMA file header
#[derive(Debug, Clone)]
pub struct BinaryPmaHeader {
    /// Magic number: "PMAFILE\0"
    pub magic: [u8; 8],
    /// Version number
    pub version: u32,
    /// Creation timestamp
    pub created_at: u64,
    /// Total number of files in archive
    pub file_count: u32,
    /// Reserved bytes for future use
    pub reserved: [u8; 16],
}

/// Binary PMA file entry
#[derive(Debug, Clone)]
pub struct BinaryPmaEntry {
    /// Original file path (UTF-8, null-terminated)
    pub path: String,
    /// File size before compression
    pub original_size: u64,
    /// File size after compression
    pub compressed_size: u64,
    /// Compression algorithm used
    pub algorithm: CompressionAlgorithm,
    /// Compression level used
    pub level: u32,
    /// Number of threads used for compression
    pub threads: u32,
    /// File modification time
    pub modified_at: u64,
    /// File permissions (Unix-style)
    pub permissions: u32,
    /// Compression ratio achieved (as percentage * 1000)
    pub compression_ratio: u32,
    /// Checksum of original file (CRC32)
    pub checksum: u32,
    /// Offset in the compressed data stream
    pub data_offset: u64,
    /// Size of compressed data chunk
    pub data_size: u64,
}

/// Binary PMA file
#[derive(Debug, Clone)]
pub struct BinaryPmaFile {
    pub header: BinaryPmaHeader,
    pub entries: Vec<BinaryPmaEntry>,
    pub compressed_data: Vec<u8>,
}

impl BinaryPmaFile {
    /// Create a new binary PMA file
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            header: BinaryPmaHeader {
                magic: *b"PMAFILE\0",
                version: 1,
                created_at: now,
                file_count: 0,
                reserved: [0; 16],
            },
            entries: Vec::new(),
            compressed_data: Vec::new(),
        }
    }

    /// Add a file to the PMA archive
    pub fn add_file(
        &mut self,
        path: String,
        original_data: &[u8],
        algorithm: CompressionAlgorithm,
        level: u32,
        threads: usize,
    ) -> Result<()> {
        // Compress the data
        let compression_context =
            crate::compression::CompressionContext::new(algorithm, Some(level));
        let compressed_data = compression_context.compress(original_data)?;

        // Calculate compression ratio (as percentage * 1000)
        let compression_ratio = if !original_data.is_empty() {
            ((1.0 - compressed_data.len() as f64 / original_data.len() as f64) * 1000.0) as u32
        } else {
            0
        };

        // Calculate CRC32 checksum
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(original_data);
        let checksum = hasher.finalize();

        // Record data offset
        let data_offset = self.compressed_data.len() as u64;

        // Create entry
        let entry = BinaryPmaEntry {
            path,
            original_size: original_data.len() as u64,
            compressed_size: compressed_data.len() as u64,
            algorithm,
            level,
            threads: threads as u32,
            modified_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            permissions: 0o644, // Default permissions
            compression_ratio,
            checksum,
            data_offset,
            data_size: compressed_data.len() as u64,
        };

        // Add compressed data
        self.compressed_data.extend_from_slice(&compressed_data);

        // Add entry
        self.entries.push(entry);
        self.header.file_count += 1;

        Ok(())
    }

    /// Save PMA file to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = std::fs::File::create(path)?;

        // Write header
        self.write_header(&mut file)?;

        // Write entries
        for entry in &self.entries {
            self.write_entry(&mut file, entry)?;
        }

        // Write compressed data
        file.write_all(&self.compressed_data)?;

        Ok(())
    }

    /// Load PMA file from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;

        // Read header
        let header = Self::read_header(&mut file)?;

        // Read entries
        let mut entries = Vec::new();
        for _ in 0..header.file_count {
            let entry = Self::read_entry(&mut file)?;
            entries.push(entry);
        }

        // Read compressed data
        let mut compressed_data = Vec::new();
        file.read_to_end(&mut compressed_data)?;

        Ok(Self {
            header,
            entries,
            compressed_data,
        })
    }

    /// Extract all files to a directory
    pub fn extract_all(&self, output_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;

        for entry in &self.entries {
            // Create output path
            let output_path = output_dir.join(&entry.path);

            // Create output directory if needed
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Extract compressed data
            let start = entry.data_offset as usize;
            let end = start + entry.data_size as usize;
            let compressed_data = &self.compressed_data[start..end];

            // Decompress data
            let compression_context =
                crate::compression::CompressionContext::new(entry.algorithm, Some(entry.level));
            let decompressed_data = compression_context.decompress(compressed_data)?;

            // Verify checksum
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(&decompressed_data);
            let calculated_checksum = hasher.finalize();
            if calculated_checksum != entry.checksum {
                return Err(crate::error::Error::InvalidInput(format!(
                    "Checksum mismatch for file: {}",
                    entry.path
                )));
            }

            // Write decompressed data
            std::fs::write(&output_path, &decompressed_data)?;
        }

        Ok(())
    }

    /// Get file statistics
    pub fn get_stats(&self) -> BinaryPmaStats {
        let total_original_size: u64 = self.entries.iter().map(|e| e.original_size).sum();
        let total_compressed_size: u64 = self.entries.iter().map(|e| e.compressed_size).sum();

        let mut algorithm_usage: HashMap<CompressionAlgorithm, u32> = HashMap::new();
        for entry in &self.entries {
            *algorithm_usage.entry(entry.algorithm).or_insert(0) += 1;
        }

        BinaryPmaStats {
            file_count: self.entries.len(),
            total_original_size,
            total_compressed_size,
            overall_ratio: if total_original_size > 0 {
                1.0 - (total_compressed_size as f64 / total_original_size as f64)
            } else {
                0.0
            },
            algorithm_usage,
        }
    }

    /// List files in the archive
    pub fn list_files(&self) -> Vec<&BinaryPmaEntry> {
        self.entries.iter().collect()
    }

    /// Write header to file
    fn write_header<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.header.magic)?;
        writer.write_u32::<LittleEndian>(self.header.version)?;
        writer.write_u64::<LittleEndian>(self.header.created_at)?;
        writer.write_u32::<LittleEndian>(self.header.file_count)?;
        writer.write_all(&self.header.reserved)?;
        Ok(())
    }

    /// Read header from file
    fn read_header<R: Read>(reader: &mut R) -> Result<BinaryPmaHeader> {
        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;
        if &magic != b"PMAFILE\0" {
            return Err(crate::error::Error::InvalidInput(
                "Invalid PMA magic number".to_string(),
            ));
        }

        let version = reader.read_u32::<LittleEndian>()?;
        let created_at = reader.read_u64::<LittleEndian>()?;
        let file_count = reader.read_u32::<LittleEndian>()?;
        let mut reserved = [0u8; 16];
        reader.read_exact(&mut reserved)?;

        Ok(BinaryPmaHeader {
            magic,
            version,
            created_at,
            file_count,
            reserved,
        })
    }

    /// Write entry to file
    fn write_entry<W: Write>(&self, writer: &mut W, entry: &BinaryPmaEntry) -> Result<()> {
        // Write path (UTF-8, null-terminated)
        let path_bytes = entry.path.as_bytes();
        writer.write_all(path_bytes)?;
        writer.write_all(&[0])?; // Null terminator

        writer.write_u64::<LittleEndian>(entry.original_size)?;
        writer.write_u64::<LittleEndian>(entry.compressed_size)?;
        writer.write_u32::<LittleEndian>(entry.algorithm as u32)?;
        writer.write_u32::<LittleEndian>(entry.level)?;
        writer.write_u32::<LittleEndian>(entry.threads)?;
        writer.write_u64::<LittleEndian>(entry.modified_at)?;
        writer.write_u32::<LittleEndian>(entry.permissions)?;
        writer.write_u32::<LittleEndian>(entry.compression_ratio)?;
        writer.write_u32::<LittleEndian>(entry.checksum)?;
        writer.write_u64::<LittleEndian>(entry.data_offset)?;
        writer.write_u64::<LittleEndian>(entry.data_size)?;

        Ok(())
    }

    /// Read entry from file
    fn read_entry<R: Read>(reader: &mut R) -> Result<BinaryPmaEntry> {
        // Read path (UTF-8, null-terminated)
        let mut path_bytes = Vec::new();
        loop {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            path_bytes.push(byte[0]);
        }
        let path = String::from_utf8(path_bytes).map_err(|e| {
            crate::error::Error::InvalidInput(format!("Invalid UTF-8 in path: {}", e))
        })?;

        let original_size = reader.read_u64::<LittleEndian>()?;
        let compressed_size = reader.read_u64::<LittleEndian>()?;
        let algorithm_raw = reader.read_u32::<LittleEndian>()?;
        let algorithm = match algorithm_raw {
            0 => CompressionAlgorithm::Lz4,
            1 => CompressionAlgorithm::Gzip,
            2 => CompressionAlgorithm::Zstd,
            _ => {
                return Err(crate::error::Error::InvalidInput(format!(
                    "Invalid algorithm: {}",
                    algorithm_raw
                )))
            }
        };
        let level = reader.read_u32::<LittleEndian>()?;
        let threads = reader.read_u32::<LittleEndian>()?;
        let modified_at = reader.read_u64::<LittleEndian>()?;
        let permissions = reader.read_u32::<LittleEndian>()?;
        let compression_ratio = reader.read_u32::<LittleEndian>()?;
        let checksum = reader.read_u32::<LittleEndian>()?;
        let data_offset = reader.read_u64::<LittleEndian>()?;
        let data_size = reader.read_u64::<LittleEndian>()?;

        Ok(BinaryPmaEntry {
            path,
            original_size,
            compressed_size,
            algorithm,
            level,
            threads,
            modified_at,
            permissions,
            compression_ratio,
            checksum,
            data_offset,
            data_size,
        })
    }
}

/// Binary PMA statistics
#[derive(Debug, Clone)]
pub struct BinaryPmaStats {
    pub file_count: usize,
    pub total_original_size: u64,
    pub total_compressed_size: u64,
    pub overall_ratio: f64,
    pub algorithm_usage: HashMap<CompressionAlgorithm, u32>,
}

impl Default for BinaryPmaFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_binary_pma_creation() {
        let temp_dir = TempDir::new().unwrap();
        let pma_path = temp_dir.path().join("test.pma");

        let mut pma_file = BinaryPmaFile::new();
        pma_file
            .add_file(
                "test.txt".to_string(),
                b"Hello, World!",
                CompressionAlgorithm::Lz4,
                1,
                1,
            )
            .unwrap();
        pma_file.save(&pma_path).unwrap();

        assert!(pma_path.exists());
    }

    #[test]
    fn test_binary_pma_loading() {
        let temp_dir = TempDir::new().unwrap();
        let pma_path = temp_dir.path().join("test.pma");

        // Create PMA file
        let mut pma_file = BinaryPmaFile::new();
        pma_file
            .add_file(
                "test.txt".to_string(),
                b"Hello, World!",
                CompressionAlgorithm::Lz4,
                1,
                1,
            )
            .unwrap();
        pma_file.save(&pma_path).unwrap();

        // Load PMA file
        let loaded_pma = BinaryPmaFile::load(&pma_path).unwrap();
        assert_eq!(loaded_pma.entries.len(), 1);
        assert_eq!(loaded_pma.entries[0].path, "test.txt");
    }

    #[test]
    fn test_binary_pma_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let pma_path = temp_dir.path().join("test.pma");
        let extract_dir = temp_dir.path().join("extracted");

        // Create PMA file
        let mut pma_file = BinaryPmaFile::new();
        pma_file
            .add_file(
                "test.txt".to_string(),
                b"Hello, World!",
                CompressionAlgorithm::Lz4,
                1,
                1,
            )
            .unwrap();
        pma_file.save(&pma_path).unwrap();

        // Extract files
        let loaded_pma = BinaryPmaFile::load(&pma_path).unwrap();
        loaded_pma.extract_all(&extract_dir).unwrap();

        let extracted_file = extract_dir.join("test.txt");
        assert!(extracted_file.exists());

        let content = std::fs::read_to_string(extracted_file).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_checksum_verification() {
        let temp_dir = TempDir::new().unwrap();
        let pma_path = temp_dir.path().join("test.pma");
        let extract_dir = temp_dir.path().join("extracted");

        // Create PMA file
        let mut pma_file = BinaryPmaFile::new();
        pma_file
            .add_file(
                "test.txt".to_string(),
                b"Hello, World!",
                CompressionAlgorithm::Lz4,
                1,
                1,
            )
            .unwrap();
        pma_file.save(&pma_path).unwrap();

        // Extract files
        let loaded_pma = BinaryPmaFile::load(&pma_path).unwrap();
        loaded_pma.extract_all(&extract_dir).unwrap();

        // Verify checksum was calculated
        assert!(loaded_pma.entries[0].checksum > 0);
    }
}
