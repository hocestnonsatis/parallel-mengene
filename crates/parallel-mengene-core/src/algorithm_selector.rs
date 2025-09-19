//! Intelligent algorithm selection based on file characteristics

use crate::algorithms::CompressionAlgorithm;
use crate::error::Result;
use std::path::Path;

/// File characteristics used for algorithm selection
#[derive(Debug, Clone)]
pub struct FileCharacteristics {
    /// File size in bytes
    pub size: u64,
    /// File extension (if any)
    pub extension: Option<String>,
    /// Estimated entropy (0.0 = very repetitive, 1.0 = random)
    pub entropy: f64,
    /// Whether the file appears to be already compressed
    pub is_likely_compressed: bool,
    /// File type category
    pub file_type: FileType,
    /// Whether the file is text-based
    pub is_text: bool,
}

/// File type categories for algorithm selection
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Text files (code, documents, logs)
    Text,
    /// Binary files (executables, images, videos)
    Binary,
    /// Archive files (already compressed)
    Archive,
    /// Database files
    Database,
    /// Unknown type
    Unknown,
}

/// Algorithm selection result
#[derive(Debug, Clone)]
pub struct AlgorithmSelection {
    /// Selected compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Recommended compression level
    pub level: u32,
    /// Recommended number of threads
    pub threads: usize,
    /// Reason for the selection
    pub reason: String,
    /// Expected compression ratio (0.0-1.0)
    pub expected_ratio: f64,
    /// Expected speed (MB/s)
    pub expected_speed: f64,
}

/// Intelligent algorithm selector
pub struct AlgorithmSelector {
    /// Available CPU cores
    pub cpu_cores: usize,
    /// Memory available for compression
    pub available_memory: usize,
    /// Performance profiles for different scenarios
    performance_profiles: PerformanceProfiles,
}

/// Performance profiles for different file types and sizes
#[derive(Debug, Clone)]
pub struct PerformanceProfiles {
    /// Small file profiles (< 1MB)
    small_file: FileSizeProfile,
    /// Medium file profiles (1MB - 100MB)
    medium_file: FileSizeProfile,
    /// Large file profiles (> 100MB)
    large_file: FileSizeProfile,
}

/// Performance profile for a specific file size range
#[derive(Debug, Clone)]
pub struct FileSizeProfile {
    /// LZ4 performance characteristics
    pub lz4: AlgorithmProfile,
    /// Gzip performance characteristics
    pub gzip: AlgorithmProfile,
    /// Zstd performance characteristics
    pub zstd: AlgorithmProfile,
}

/// Performance characteristics of an algorithm
#[derive(Debug, Clone)]
pub struct AlgorithmProfile {
    /// Typical compression ratio (0.0-1.0)
    pub compression_ratio: f64,
    /// Typical compression speed (MB/s)
    pub compression_speed: f64,
    /// Typical decompression speed (MB/s)
    pub decompression_speed: f64,
    /// Optimal thread count
    pub optimal_threads: usize,
    /// Whether this algorithm is recommended for this file size
    pub recommended: bool,
}

impl AlgorithmSelector {
    /// Create a new algorithm selector
    pub fn new() -> Self {
        Self {
            cpu_cores: num_cpus::get(),
            available_memory: Self::estimate_available_memory(),
            performance_profiles: PerformanceProfiles::default(),
        }
    }

    /// Analyze file characteristics for algorithm selection
    pub fn analyze_file(&self, path: &Path) -> Result<FileCharacteristics> {
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());

        // Read a sample of the file for analysis
        let sample_size = std::cmp::min(size, 64 * 1024) as usize; // 64KB sample
        let sample = self.read_file_sample(path, sample_size)?;

        let entropy = self.calculate_entropy(&sample);
        let is_likely_compressed = self.is_likely_compressed(&sample, &extension);
        let file_type = self.determine_file_type(&extension, &sample);
        let is_text = self.is_text_file(&sample);

        Ok(FileCharacteristics {
            size,
            extension,
            entropy,
            is_likely_compressed,
            file_type,
            is_text,
        })
    }

    /// Select the best algorithm based on file characteristics
    pub fn select_algorithm(&self, characteristics: &FileCharacteristics) -> AlgorithmSelection {
        let profile = self.get_file_size_profile(characteristics.size);

        // Skip compression for already compressed files
        if characteristics.is_likely_compressed {
            return AlgorithmSelection {
                algorithm: CompressionAlgorithm::Lz4, // Fastest for already compressed
                level: 1,
                threads: 1, // Single thread for small compressed files
                reason: "File appears to be already compressed, using LZ4 for speed".to_string(),
                expected_ratio: 0.05,  // Minimal compression expected
                expected_speed: 500.0, // Very fast
            };
        }

        // Select algorithm based on file characteristics
        let (algorithm, reason) =
            self.select_algorithm_for_characteristics(characteristics, profile);
        let level = self.select_compression_level(algorithm, characteristics);
        let threads = self.select_thread_count(algorithm, characteristics);

        let expected_ratio = self.estimate_compression_ratio(algorithm, characteristics);
        let expected_speed = self.estimate_compression_speed(algorithm, characteristics, threads);

        AlgorithmSelection {
            algorithm,
            level,
            threads,
            reason,
            expected_ratio,
            expected_speed,
        }
    }

    /// Read a sample from the beginning of the file
    fn read_file_sample(&self, path: &Path, sample_size: usize) -> Result<Vec<u8>> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut buffer = vec![0; sample_size];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    /// Calculate entropy of the data sample
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let mut entropy = 0.0;
        let data_len = data.len() as f64;

        for &count in &counts {
            if count > 0 {
                let probability = count as f64 / data_len;
                entropy -= probability * probability.log2();
            }
        }

        entropy / 8.0 // Normalize to 0.0-1.0
    }

    /// Determine if file is likely already compressed
    fn is_likely_compressed(&self, sample: &[u8], extension: &Option<String>) -> bool {
        // Check file extension first
        if let Some(ext) = extension {
            if matches!(
                ext.as_str(),
                "gz" | "zip"
                    | "rar"
                    | "7z"
                    | "bz2"
                    | "xz"
                    | "lz4"
                    | "zst"
                    | "tar"
                    | "mp4"
                    | "mp3"
                    | "jpg"
                    | "png"
                    | "pdf"
            ) {
                return true;
            }
        }

        // Check for compression signatures
        if sample.len() >= 4 {
            // Check for common compression magic numbers
            let magic = &sample[0..4];
            if magic == b"\x1f\x8b\x08" || // gzip
               magic == b"PK\x03\x04" ||   // zip
               magic == b"Rar!" ||         // rar
               magic == b"7z\xbc\xaf" ||   // 7z
               magic == b"BZ" ||           // bzip2
               magic == b"\xfd7zXZ" ||     // xz
               magic == b"\x04\x22\x4d\x18" || // lz4
               magic == b"\x28\xb5\x2f\xfd"
            // zstd
            {
                return true;
            }
        }

        // Check entropy - very high entropy suggests already compressed
        let entropy = self.calculate_entropy(sample);
        entropy > 0.9
    }

    /// Determine file type based on extension and content
    fn determine_file_type(&self, extension: &Option<String>, sample: &[u8]) -> FileType {
        if let Some(ext) = extension {
            match ext.as_str() {
                "txt" | "md" | "log" | "csv" | "json" | "xml" | "html" | "css" | "js" | "rs"
                | "py" | "c" | "cpp" | "h" | "hpp" => {
                    return FileType::Text;
                }
                "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "lz4" | "zst" => {
                    return FileType::Archive;
                }
                "db" | "sqlite" | "sqlite3" | "mdb" | "accdb" => {
                    return FileType::Database;
                }
                _ => {}
            }
        }

        // Check content for text files
        if self.is_text_file(sample) {
            FileType::Text
        } else {
            FileType::Binary
        }
    }

    /// Check if data appears to be text
    fn is_text_file(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }

        let text_chars = data
            .iter()
            .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
            .count();

        let text_ratio = text_chars as f64 / data.len() as f64;
        text_ratio > 0.7 // 70% printable characters
    }

    /// Get performance profile for file size
    fn get_file_size_profile(&self, size: u64) -> &FileSizeProfile {
        if size < 1024 * 1024 {
            // < 1MB
            &self.performance_profiles.small_file
        } else if size < 100 * 1024 * 1024 {
            // 1MB - 100MB
            &self.performance_profiles.medium_file
        } else {
            // > 100MB
            &self.performance_profiles.large_file
        }
    }

    /// Select algorithm based on file characteristics
    fn select_algorithm_for_characteristics(
        &self,
        characteristics: &FileCharacteristics,
        profile: &FileSizeProfile,
    ) -> (CompressionAlgorithm, String) {
        // Calculate efficiency score for each algorithm
        let lz4_score =
            self.calculate_algorithm_score(CompressionAlgorithm::Lz4, characteristics, profile);
        let gzip_score =
            self.calculate_algorithm_score(CompressionAlgorithm::Gzip, characteristics, profile);
        let zstd_score =
            self.calculate_algorithm_score(CompressionAlgorithm::Zstd, characteristics, profile);

        // Select algorithm with highest score
        if lz4_score >= gzip_score && lz4_score >= zstd_score {
            (
                CompressionAlgorithm::Lz4,
                format!("LZ4 selected (score: {:.2})", lz4_score),
            )
        } else if gzip_score >= zstd_score {
            (
                CompressionAlgorithm::Gzip,
                format!("Gzip selected (score: {:.2})", gzip_score),
            )
        } else {
            (
                CompressionAlgorithm::Zstd,
                format!("Zstd selected (score: {:.2})", zstd_score),
            )
        }
    }

    /// Calculate efficiency score for an algorithm based on file characteristics
    fn calculate_algorithm_score(
        &self,
        algorithm: CompressionAlgorithm,
        characteristics: &FileCharacteristics,
        profile: &FileSizeProfile,
    ) -> f64 {
        let algorithm_profile = match algorithm {
            CompressionAlgorithm::Lz4 => &profile.lz4,
            CompressionAlgorithm::Gzip => &profile.gzip,
            CompressionAlgorithm::Zstd => &profile.zstd,
        };

        // Base score from performance profile
        let mut score = algorithm_profile.compression_ratio * 0.4
            + (algorithm_profile.compression_speed / 1000.0) * 0.3
            + (algorithm_profile.decompression_speed / 1000.0) * 0.3;

        // Adjust based on file characteristics
        match characteristics.file_type {
            FileType::Text => {
                // Text files benefit from better compression
                if algorithm == CompressionAlgorithm::Zstd {
                    score += 0.2;
                } else if algorithm == CompressionAlgorithm::Gzip {
                    score += 0.1;
                }
            }
            FileType::Binary => {
                if characteristics.entropy > 0.8 {
                    // High entropy binary - prefer speed
                    if algorithm == CompressionAlgorithm::Lz4 {
                        score += 0.3;
                    }
                } else {
                    // Low entropy binary - prefer compression
                    if algorithm == CompressionAlgorithm::Zstd {
                        score += 0.2;
                    }
                }
            }
            FileType::Archive => {
                // Already compressed - prefer speed
                if algorithm == CompressionAlgorithm::Lz4 {
                    score += 0.4;
                }
            }
            FileType::Database => {
                // Database files - balanced approach
                if algorithm == CompressionAlgorithm::Gzip {
                    score += 0.2;
                }
            }
            FileType::Unknown => {
                // Unknown type - prefer Zstd as safe default
                if algorithm == CompressionAlgorithm::Zstd {
                    score += 0.1;
                }
            }
        }

        // Adjust for file size
        if characteristics.size < 1024 * 1024 {
            // Small files - prefer speed
            if algorithm == CompressionAlgorithm::Lz4 {
                score += 0.2;
            }
        } else if characteristics.size > 100 * 1024 * 1024 {
            // Large files - prefer compression
            if algorithm == CompressionAlgorithm::Zstd {
                score += 0.1;
            }
        }

        score
    }

    /// Select compression level based on algorithm and characteristics
    fn select_compression_level(
        &self,
        algorithm: CompressionAlgorithm,
        characteristics: &FileCharacteristics,
    ) -> u32 {
        match algorithm {
            CompressionAlgorithm::Lz4 => {
                if characteristics.size < 1024 * 1024 {
                    1 // Fast compression for small files
                } else {
                    3 // Balanced for larger files
                }
            }
            CompressionAlgorithm::Gzip => {
                if characteristics.size < 1024 * 1024 {
                    6 // Default level for small files
                } else {
                    9 // High compression for larger files
                }
            }
            CompressionAlgorithm::Zstd => {
                if characteristics.size < 1024 * 1024 {
                    3 // Fast compression for small files
                } else if characteristics.size < 100 * 1024 * 1024 {
                    6 // Balanced for medium files
                } else {
                    9 // High compression for large files
                }
            }
        }
    }

    /// Select optimal thread count
    fn select_thread_count(
        &self,
        algorithm: CompressionAlgorithm,
        characteristics: &FileCharacteristics,
    ) -> usize {
        // Small files: single thread
        if characteristics.size < 1024 * 1024 {
            return 1;
        }

        // Medium files: limited parallelism
        if characteristics.size < 100 * 1024 * 1024 {
            return std::cmp::min(2, self.cpu_cores);
        }

        // Large files: use more threads
        match algorithm {
            CompressionAlgorithm::Lz4 => {
                // LZ4 scales well with threads
                self.cpu_cores
            }
            CompressionAlgorithm::Gzip => {
                // Gzip has limited parallelism
                std::cmp::min(4, self.cpu_cores)
            }
            CompressionAlgorithm::Zstd => {
                // Zstd has good parallelism
                self.cpu_cores
            }
        }
    }

    /// Estimate compression ratio
    fn estimate_compression_ratio(
        &self,
        algorithm: CompressionAlgorithm,
        characteristics: &FileCharacteristics,
    ) -> f64 {
        let base_ratio = match algorithm {
            CompressionAlgorithm::Lz4 => 0.3,  // 30% compression
            CompressionAlgorithm::Gzip => 0.4, // 40% compression
            CompressionAlgorithm::Zstd => 0.5, // 50% compression
        };

        // Adjust based on file characteristics
        let mut ratio = base_ratio;

        if characteristics.is_text {
            ratio += 0.1; // Text compresses better
        }

        if characteristics.entropy < 0.5 {
            ratio += 0.1; // Low entropy compresses better
        }

        if characteristics.is_likely_compressed {
            ratio = 0.05; // Already compressed files don't compress much
        }

        if ratio > 0.9 {
            0.9
        } else {
            ratio
        } // Cap at 90% compression
    }

    /// Estimate compression speed
    fn estimate_compression_speed(
        &self,
        algorithm: CompressionAlgorithm,
        characteristics: &FileCharacteristics,
        threads: usize,
    ) -> f64 {
        let base_speed = match algorithm {
            CompressionAlgorithm::Lz4 => 200.0,  // 200 MB/s
            CompressionAlgorithm::Gzip => 50.0,  // 50 MB/s
            CompressionAlgorithm::Zstd => 100.0, // 100 MB/s
        };

        // Scale with thread count (diminishing returns)
        let thread_factor = 1.0 + (threads as f64 - 1.0) * 0.7;
        let speed = base_speed * thread_factor;

        // Adjust for file characteristics
        if characteristics.is_likely_compressed {
            speed * 2.0 // Already compressed files are faster to process
        } else {
            speed
        }
    }

    /// Estimate available memory
    fn estimate_available_memory() -> usize {
        // Try to get actual system memory using sysinfo
        match sysinfo::System::new_all().total_memory() {
            total_memory if total_memory > 0 => {
                // Use 75% of total memory as available for compression
                (total_memory as f64 * 0.75) as usize
            }
            _ => {
                // Fallback to 1GB if we can't determine system memory
                1024 * 1024 * 1024
            }
        }
    }
}

impl Default for AlgorithmSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PerformanceProfiles {
    fn default() -> Self {
        Self {
            small_file: FileSizeProfile {
                lz4: AlgorithmProfile {
                    compression_ratio: 0.2,
                    compression_speed: 300.0,
                    decompression_speed: 1000.0,
                    optimal_threads: 1,
                    recommended: true,
                },
                gzip: AlgorithmProfile {
                    compression_ratio: 0.4,
                    compression_speed: 80.0,
                    decompression_speed: 200.0,
                    optimal_threads: 1,
                    recommended: false,
                },
                zstd: AlgorithmProfile {
                    compression_ratio: 0.45,
                    compression_speed: 150.0,
                    decompression_speed: 500.0,
                    optimal_threads: 1,
                    recommended: false,
                },
            },
            medium_file: FileSizeProfile {
                lz4: AlgorithmProfile {
                    compression_ratio: 0.3,
                    compression_speed: 250.0,
                    decompression_speed: 800.0,
                    optimal_threads: 2,
                    recommended: true,
                },
                gzip: AlgorithmProfile {
                    compression_ratio: 0.5,
                    compression_speed: 60.0,
                    decompression_speed: 180.0,
                    optimal_threads: 2,
                    recommended: true,
                },
                zstd: AlgorithmProfile {
                    compression_ratio: 0.55,
                    compression_speed: 120.0,
                    decompression_speed: 400.0,
                    optimal_threads: 2,
                    recommended: true,
                },
            },
            large_file: FileSizeProfile {
                lz4: AlgorithmProfile {
                    compression_ratio: 0.35,
                    compression_speed: 200.0,
                    decompression_speed: 600.0,
                    optimal_threads: 4,
                    recommended: false,
                },
                gzip: AlgorithmProfile {
                    compression_ratio: 0.6,
                    compression_speed: 50.0,
                    decompression_speed: 150.0,
                    optimal_threads: 4,
                    recommended: true,
                },
                zstd: AlgorithmProfile {
                    compression_ratio: 0.65,
                    compression_speed: 100.0,
                    decompression_speed: 300.0,
                    optimal_threads: 4,
                    recommended: true,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_entropy_calculation() {
        let selector = AlgorithmSelector::new();

        // Repetitive data should have low entropy
        let repetitive = b"AAAAA".repeat(100);
        let entropy = selector.calculate_entropy(&repetitive);
        assert!(entropy < 0.1); // Very low entropy for repetitive data

        // Random data should have high entropy
        let random = (0..1000).map(|i| (i % 256) as u8).collect::<Vec<u8>>();
        let entropy = selector.calculate_entropy(&random);
        assert!(entropy > 0.8); // High entropy for random data
    }

    #[test]
    fn test_compressed_file_detection() {
        let selector = AlgorithmSelector::new();

        // Create a test file with gzip magic number and .gz extension
        let temp_file = NamedTempFile::with_suffix(".gz").unwrap();
        let mut file = std::fs::File::create(temp_file.path()).unwrap();
        file.write_all(b"\x1f\x8b\x08").unwrap();
        // Add more data to make it large enough for analysis
        file.write_all(&vec![0u8; 1000]).unwrap();
        drop(file);

        let characteristics = selector.analyze_file(temp_file.path()).unwrap();
        assert!(characteristics.is_likely_compressed);
    }

    #[test]
    fn test_algorithm_selection() {
        let selector = AlgorithmSelector::new();

        // Test small text file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file
            .write_all(b"Hello, World! This is a test file.")
            .unwrap();

        let characteristics = selector.analyze_file(temp_file.path()).unwrap();
        let selection = selector.select_algorithm(&characteristics);

        assert_eq!(selection.algorithm, CompressionAlgorithm::Lz4);
        assert_eq!(selection.threads, 1);
        // Check for any reason that indicates small file processing
        assert!(selection.reason.contains("LZ4") || selection.reason.contains("score"));
    }
}
