//! Test data generation for comprehensive benchmarking

use parallel_mengene_core::error::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Test data generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataConfig {
    pub output_dir: PathBuf,
    pub file_sizes: Vec<usize>, // in bytes
    pub data_types: Vec<DataType>,
    pub compression_levels: Vec<CompressionLevel>,
    pub seed: Option<u64>,
}

/// Types of test data to generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Random,
    Repetitive,
    Text,
    Binary,
    Mixed,
    ZeroFilled,
    PatternBased,
}

/// Compression difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionLevel {
    Easy,   // Highly compressible
    Medium, // Moderately compressible
    Hard,   // Difficult to compress
}

impl Default for TestDataConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("test_data"),
            file_sizes: vec![
                1024,              // 1KB
                1024 * 1024,       // 1MB
                10 * 1024 * 1024,  // 10MB
                100 * 1024 * 1024, // 100MB
            ],
            data_types: vec![
                DataType::Random,
                DataType::Repetitive,
                DataType::Text,
                DataType::Binary,
                DataType::Mixed,
                DataType::ZeroFilled,
            ],
            compression_levels: vec![
                CompressionLevel::Easy,
                CompressionLevel::Medium,
                CompressionLevel::Hard,
            ],
            seed: Some(42),
        }
    }
}

/// Test data generator
pub struct TestDataGenerator {
    config: TestDataConfig,
    rng: StdRng,
}

impl TestDataGenerator {
    /// Create a new test data generator
    pub fn new(config: TestDataConfig) -> Self {
        let seed = config.seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });

        let rng = StdRng::seed_from_u64(seed);

        Self { config, rng }
    }

    /// Generate all test data according to configuration
    pub fn generate_all(&mut self) -> Result<()> {
        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        let mut generated_files = Vec::new();

        let file_sizes = self.config.file_sizes.clone();
        let data_types = self.config.data_types.clone();
        let compression_levels = self.config.compression_levels.clone();

        for size in file_sizes {
            for data_type in &data_types {
                for compression_level in &compression_levels {
                    let file_path = self.generate_file(size, data_type, compression_level)?;
                    generated_files.push(file_path);
                }
            }
        }

        // Generate metadata file
        self.generate_metadata(&generated_files)?;

        println!("✅ Generated {} test files", generated_files.len());
        println!("📁 Output directory: {:?}", self.config.output_dir);

        Ok(())
    }

    /// Generate a single test file
    fn generate_file(
        &mut self,
        size: usize,
        data_type: &DataType,
        compression_level: &CompressionLevel,
    ) -> Result<PathBuf> {
        let file_name = format!(
            "test_{}_{}_{}.bin",
            self.format_size(size),
            format!("{:?}", data_type).to_lowercase(),
            format!("{:?}", compression_level).to_lowercase()
        );

        let file_path = self.config.output_dir.join(file_name);

        let data = match data_type {
            DataType::Random => self.generate_random_data(size, compression_level),
            DataType::Repetitive => self.generate_repetitive_data(size, compression_level),
            DataType::Text => self.generate_text_data(size, compression_level),
            DataType::Binary => self.generate_binary_data(size, compression_level),
            DataType::Mixed => self.generate_mixed_data(size, compression_level),
            DataType::ZeroFilled => self.generate_zero_filled_data(size),
            DataType::PatternBased => self.generate_pattern_data(size, compression_level),
        };

        fs::write(&file_path, data)?;
        Ok(file_path)
    }

    /// Generate random data
    fn generate_random_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        let mut data = vec![0u8; size];

        match level {
            CompressionLevel::Easy => {
                // Use limited byte range for better compression
                for byte in data.iter_mut() {
                    *byte = self.rng.random_range(0..16);
                }
            }
            CompressionLevel::Medium => {
                // Use wider byte range
                for byte in data.iter_mut() {
                    *byte = self.rng.random_range(0..64);
                }
            }
            CompressionLevel::Hard => {
                // Use full byte range
                for byte in data.iter_mut() {
                    *byte = self.rng.random();
                }
            }
        }

        data
    }

    /// Generate repetitive data
    fn generate_repetitive_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);

        match level {
            CompressionLevel::Easy => {
                // Long repeated patterns
                let pattern_size = 1024;
                let pattern = self.generate_random_data(pattern_size, level);
                let repetitions = size / pattern_size;

                for _ in 0..repetitions {
                    data.extend_from_slice(&pattern);
                }

                // Fill remaining bytes
                let remaining = size % pattern_size;
                if remaining > 0 {
                    data.extend_from_slice(&pattern[..remaining]);
                }
            }
            CompressionLevel::Medium => {
                // Medium-sized repeated patterns
                let pattern_size = 256;
                let pattern = self.generate_random_data(pattern_size, level);
                let repetitions = size / pattern_size;

                for _ in 0..repetitions {
                    data.extend_from_slice(&pattern);
                }

                let remaining = size % pattern_size;
                if remaining > 0 {
                    data.extend_from_slice(&pattern[..remaining]);
                }
            }
            CompressionLevel::Hard => {
                // Short repeated patterns
                let pattern_size = 16;
                let pattern = self.generate_random_data(pattern_size, level);
                let repetitions = size / pattern_size;

                for _ in 0..repetitions {
                    data.extend_from_slice(&pattern);
                }

                let remaining = size % pattern_size;
                if remaining > 0 {
                    data.extend_from_slice(&pattern[..remaining]);
                }
            }
        }

        data
    }

    /// Generate text data
    fn generate_text_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);

        match level {
            CompressionLevel::Easy => {
                // Highly repetitive text
                let words = [
                    "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog",
                ];
                while data.len() < size {
                    let word = words[self.rng.random_range(0..words.len())];
                    data.extend_from_slice(word.as_bytes());
                    data.push(b' ');
                }
            }
            CompressionLevel::Medium => {
                // Medium complexity text
                let words = [
                    "the",
                    "quick",
                    "brown",
                    "fox",
                    "jumps",
                    "over",
                    "lazy",
                    "dog",
                    "hello",
                    "world",
                    "this",
                    "is",
                    "a",
                    "test",
                    "file",
                    "generated",
                    "for",
                    "benchmarking",
                    "purposes",
                    "with",
                    "various",
                    "compression",
                    "algorithms",
                    "and",
                    "performance",
                    "analysis",
                ];
                while data.len() < size {
                    let word = words[self.rng.random_range(0..words.len())];
                    data.extend_from_slice(word.as_bytes());
                    data.push(b' ');
                }
            }
            CompressionLevel::Hard => {
                // Random text-like data
                let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \n\t";
                while data.len() < size {
                    data.push(chars[self.rng.random_range(0..chars.len())]);
                }
            }
        }

        data.truncate(size);
        data
    }

    /// Generate binary data
    fn generate_binary_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        match level {
            CompressionLevel::Easy => {
                // Structured binary data with patterns
                let mut data = vec![0u8; size];
                for (i, byte) in data.iter_mut().enumerate().take(size) {
                    *byte = (i % 256) as u8;
                }
                data
            }
            CompressionLevel::Medium => {
                // Semi-structured binary data
                let mut data = vec![0u8; size];
                for chunk in data.chunks_mut(64) {
                    let pattern = self.rng.random::<u64>();
                    for (i, byte) in chunk.iter_mut().enumerate() {
                        *byte = (pattern >> ((i % 8) * 8)) as u8;
                    }
                }
                data
            }
            CompressionLevel::Hard => {
                // Random binary data
                self.generate_random_data(size, level)
            }
        }
    }

    /// Generate mixed data
    fn generate_mixed_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        let chunk_size = size / 4; // Divide into 4 chunks

        // Text chunk
        let text_chunk = self.generate_text_data(chunk_size, level);
        data.extend_from_slice(&text_chunk);

        // Binary chunk
        let binary_chunk = self.generate_binary_data(chunk_size, level);
        data.extend_from_slice(&binary_chunk);

        // Repetitive chunk
        let repetitive_chunk = self.generate_repetitive_data(chunk_size, level);
        data.extend_from_slice(&repetitive_chunk);

        // Random chunk
        let random_chunk = self.generate_random_data(size - data.len(), level);
        data.extend_from_slice(&random_chunk);

        data
    }

    /// Generate zero-filled data
    fn generate_zero_filled_data(&mut self, size: usize) -> Vec<u8> {
        vec![0u8; size]
    }

    /// Generate pattern-based data
    fn generate_pattern_data(&mut self, size: usize, level: &CompressionLevel) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);

        match level {
            CompressionLevel::Easy => {
                // Simple arithmetic progression
                for i in 0..size {
                    data.push((i % 256) as u8);
                }
            }
            CompressionLevel::Medium => {
                // Fibonacci-like pattern
                let mut a = 0u8;
                let mut b = 1u8;
                for _ in 0..size {
                    data.push(a);
                    let temp = a;
                    a = b;
                    b = temp.wrapping_add(b);
                }
            }
            CompressionLevel::Hard => {
                // Complex mathematical pattern
                for i in 0..size {
                    let value = ((i as f64).sin() * 127.0 + 127.0) as u8;
                    data.push(value);
                }
            }
        }

        data
    }

    /// Format file size for filename
    fn format_size(&self, size: usize) -> String {
        if size < 1024 {
            format!("{}b", size)
        } else if size < 1024 * 1024 {
            format!("{}kb", size / 1024)
        } else if size < 1024 * 1024 * 1024 {
            format!("{}mb", size / (1024 * 1024))
        } else {
            format!("{}gb", size / (1024 * 1024 * 1024))
        }
    }

    /// Generate metadata file
    fn generate_metadata(&self, files: &[PathBuf]) -> Result<()> {
        let metadata = serde_json::to_string_pretty(&TestDataMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            total_files: files.len(),
            files: files
                .iter()
                .map(|f| f.file_name().unwrap().to_string_lossy().to_string())
                .collect(),
            config: self.config.clone(),
        })
        .map_err(|e| parallel_mengene_core::error::Error::InvalidInput(e.to_string()))?;

        let metadata_path = self.config.output_dir.join("metadata.json");
        fs::write(metadata_path, metadata)?;

        Ok(())
    }
}

/// Test data metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestDataMetadata {
    generated_at: String,
    total_files: usize,
    files: Vec<String>,
    config: TestDataConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_test_data_generator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = TestDataConfig {
            output_dir: temp_dir.path().to_path_buf(),
            file_sizes: vec![1024],
            data_types: vec![DataType::Random],
            compression_levels: vec![CompressionLevel::Medium],
            seed: Some(42),
        };

        let mut generator = TestDataGenerator::new(config);
        let _value = generator.rng.random::<u8>();
        // Just test that rng works - no assertion needed
    }

    #[test]
    fn test_data_generation() {
        let temp_dir = TempDir::new().unwrap();
        let config = TestDataConfig {
            output_dir: temp_dir.path().to_path_buf(),
            file_sizes: vec![1024],
            data_types: vec![DataType::Random, DataType::Text],
            compression_levels: vec![CompressionLevel::Medium],
            seed: Some(42),
        };

        let mut generator = TestDataGenerator::new(config);
        let result = generator.generate_all();
        assert!(result.is_ok());

        // Check that files were created
        let files: Vec<_> = std::fs::read_dir(temp_dir.path()).unwrap().collect();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_size_formatting() {
        let config = TestDataConfig::default();
        let generator = TestDataGenerator::new(config);

        assert_eq!(generator.format_size(512), "512b");
        assert_eq!(generator.format_size(2048), "2kb");
        assert_eq!(generator.format_size(2 * 1024 * 1024), "2mb");
    }
}
