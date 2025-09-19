//! Intelligent compression pipeline with automatic algorithm selection

use crate::coordinator::PipelineCoordinator;
use crate::memory_mapping::MemoryMappingConfig;
use crate::memory_monitor::MemoryUsageTracker;
use parallel_mengene_core::algorithm_selector::AlgorithmSelector;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::binary_pma_format::BinaryPmaFile;
use parallel_mengene_core::error::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

/// Intelligent compression pipeline that automatically selects algorithms
pub struct IntelligentPipeline {
    algorithm_selector: AlgorithmSelector,
    coordinator: PipelineCoordinator,
    memory_config: MemoryMappingConfig,
    memory_tracker: MemoryUsageTracker,
}

/// Compression job for a single file
#[derive(Debug, Clone)]
pub struct CompressionJob {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub algorithm: CompressionAlgorithm,
    pub level: u32,
    pub threads: usize,
    pub reason: String,
}

/// Compression results
#[derive(Debug, Clone)]
pub struct CompressionResults {
    pub jobs: Vec<CompressionJob>,
    pub total_original_size: u64,
    pub total_compressed_size: u64,
    pub compression_time: std::time::Duration,
    pub pma_file_path: PathBuf,
}

impl IntelligentPipeline {
    /// Create a new intelligent pipeline
    pub fn new() -> Result<Self> {
        let algorithm_selector = AlgorithmSelector::new();
        let coordinator = PipelineCoordinator::new();
        let memory_config = MemoryMappingConfig::default();
        let memory_tracker = MemoryUsageTracker::new();

        Ok(Self {
            algorithm_selector,
            coordinator,
            memory_config,
            memory_tracker,
        })
    }

    /// Compress a directory or file with intelligent algorithm selection
    pub async fn compress_intelligently(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<CompressionResults> {
        let start_time = Instant::now();

        // Check memory usage before starting
        self.memory_tracker.check_and_warn()?;

        // Analyze input to determine what to compress
        let files_to_compress = self.analyze_input(input_path).await?;

        // Create binary PMA file
        let mut pma_file = BinaryPmaFile::new();

        // Process each file with intelligent algorithm selection
        let mut jobs = Vec::new();
        let mut total_original_size = 0u64;
        let mut total_compressed_size = 0u64;

        for file_path in files_to_compress {
            // Analyze file characteristics
            let characteristics = self.algorithm_selector.analyze_file(&file_path)?;

            // Select optimal algorithm and settings
            let selection = self.algorithm_selector.select_algorithm(&characteristics);

            // Read file data with memory management
            let file_data = if characteristics.size > self.memory_config.large_file_threshold() {
                // For large files, use memory-mapped reading
                self.read_file_with_memory_mapping(&file_path)?
            } else {
                // For small files, use regular reading
                std::fs::read(&file_path)?
            };

            // Create compression job
            let relative_path = self.get_relative_path(input_path, &file_path);
            let job = CompressionJob {
                input_path: file_path.clone(),
                output_path: PathBuf::new(), // Not used in PMA format
                algorithm: selection.algorithm,
                level: selection.level,
                threads: selection.threads,
                reason: selection.reason,
            };

            // Add file to PMA archive
            pma_file.add_file(
                relative_path,
                &file_data,
                selection.algorithm,
                selection.level,
                selection.threads,
            )?;

            // Update memory tracking
            self.memory_tracker.record_operation(
                "compression",
                file_data.len(),
                std::time::Instant::now(),
            )?;

            jobs.push(job);
            total_original_size += characteristics.size;
            total_compressed_size +=
                (selection.expected_ratio * characteristics.size as f64) as u64;
        }

        // Save PMA file
        pma_file.save(output_path)?;

        let compression_time = start_time.elapsed();

        Ok(CompressionResults {
            jobs,
            total_original_size,
            total_compressed_size,
            compression_time,
            pma_file_path: output_path.to_path_buf(),
        })
    }

    /// Analyze input to determine files to compress
    async fn analyze_input(&self, input_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if input_path.is_file() {
            files.push(input_path.to_path_buf());
        } else if input_path.is_dir() {
            // Walk directory and collect files with parallel processing
            let walker = WalkDir::new(input_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());

            // Collect files in parallel using rayon
            files = walker.map(|entry| entry.path().to_path_buf()).collect();
        } else {
            return Err(parallel_mengene_core::error::Error::InvalidInput(
                "Input path does not exist or is not a file or directory".to_string(),
            ));
        }

        Ok(files)
    }

    /// Get relative path from base to file
    fn get_relative_path(&self, base: &Path, file: &Path) -> String {
        if base.is_file() {
            // If base is a file, use just the filename
            file.file_name()
                .unwrap_or(file.as_os_str())
                .to_string_lossy()
                .to_string()
        } else {
            // If base is a directory, get relative path
            file.strip_prefix(base)
                .unwrap_or(file)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Decompress a PMA file
    pub async fn decompress_pma(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Check memory usage before starting
        self.memory_tracker.check_and_warn()?;

        // Load PMA file
        let pma_file = BinaryPmaFile::load(input_path)?;

        // Extract all files
        pma_file.extract_all(output_path)?;

        Ok(())
    }

    /// Read file with memory mapping for large files
    fn read_file_with_memory_mapping(&self, file_path: &Path) -> Result<Vec<u8>> {
        // For now, fall back to regular reading
        // In a full implementation, this would use memory mapping
        std::fs::read(file_path).map_err(parallel_mengene_core::error::Error::Io)
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> String {
        format!(
            "Intelligent Pipeline Stats:\n\
             CPU Cores: {}\n\
             Available Memory: {:.2} MB\n\
             Chunk Size: {} bytes\n\
             Parallel Threshold: {} bytes",
            self.algorithm_selector.cpu_cores,
            self.algorithm_selector.available_memory as f64 / 1_048_576.0,
            self.coordinator.chunk_size(),
            self.coordinator.parallel_threshold()
        )
    }
}

impl Default for IntelligentPipeline {
    fn default() -> Self {
        Self::new().expect("Failed to create intelligent pipeline")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_intelligent_compression() {
        let temp_dir = TempDir::new().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        // Create test files
        std::fs::create_dir_all(&input_dir).unwrap();
        write(
            input_dir.join("test.txt"),
            "Hello, World! This is a test file.",
        )
        .unwrap();
        write(input_dir.join("data.bin"), vec![0u8; 1000]).unwrap();

        let pipeline = IntelligentPipeline::new().unwrap();
        let results = pipeline
            .compress_intelligently(&input_dir, &output_dir)
            .await
            .unwrap();

        assert_eq!(results.jobs.len(), 2);
        assert!(results.total_original_size > 0);
        assert!(results.total_compressed_size > 0);

        // Verify PMA file was created (it should be the output path itself)
        assert!(output_dir.exists());
    }

    #[tokio::test]
    async fn test_pma_decompression() {
        let temp_dir = TempDir::new().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        let decompress_dir = temp_dir.path().join("decompressed");

        // Create test files
        std::fs::create_dir_all(&input_dir).unwrap();
        write(
            input_dir.join("test.txt"),
            "Hello, World! This is a test file.",
        )
        .unwrap();

        let pipeline = IntelligentPipeline::new().unwrap();

        // Compress
        let _results = pipeline
            .compress_intelligently(&input_dir, &output_dir)
            .await
            .unwrap();

        // Decompress
        pipeline
            .decompress_pma(&output_dir, &decompress_dir)
            .await
            .unwrap();

        // Verify decompression
        let decompressed_file = decompress_dir.join("test.txt");
        assert!(decompressed_file.exists());

        let content = std::fs::read_to_string(decompressed_file).unwrap();
        assert_eq!(content, "Hello, World! This is a test file.");
    }
}
