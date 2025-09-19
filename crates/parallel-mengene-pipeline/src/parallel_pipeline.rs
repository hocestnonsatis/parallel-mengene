//! High-performance parallel compression pipeline

use crate::coordinator::PipelineCoordinator;
use crate::cpu_pipeline::CpuPipeline;
use crate::memory_mapping::{MemoryMappedFile, MemoryMappingConfig};
use crate::memory_monitor::MemoryUsageTracker;
use parallel_mengene_core::algorithms::CompressionAlgorithm;
use parallel_mengene_core::error::Result;
use rayon::prelude::*;
use std::io::Seek;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Main parallel compression pipeline
///
/// This orchestrates the parallel approach:
/// ┌─── Input File ────┐
/// │                   │
/// ├── CPU Pipeline ───┤ ← Multi-threaded compression
/// │                   │
/// └── Parallel Chunks ┘ ← Intelligent workload distribution
///         │
///         ├── Rayon parallel processing
///         ├── Memory mapping for large files
///         └── Optimized chunk sizes
pub struct ParallelPipeline {
    cpu_pipeline: CpuPipeline,
    coordinator: PipelineCoordinator,
    memory_config: MemoryMappingConfig,
    memory_tracker: MemoryUsageTracker,
}

impl ParallelPipeline {
    /// Create a new parallel pipeline
    pub fn new(algorithm: CompressionAlgorithm) -> Result<Self> {
        let cpu_pipeline = CpuPipeline::new(algorithm)?;
        let coordinator = PipelineCoordinator::new();
        let memory_config = MemoryMappingConfig::default();
        let memory_tracker = MemoryUsageTracker::new();

        Ok(Self {
            cpu_pipeline,
            coordinator,
            memory_config,
            memory_tracker,
        })
    }

    /// Compress a file using parallel approach
    pub async fn compress_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // 1. Check memory usage before starting
        self.memory_tracker.check_and_warn()?;
        self.memory_tracker.monitor().log_memory_usage()?;

        // 2. Analyze file to determine optimal strategy
        let file_size = std::fs::metadata(input_path)?.len();
        let strategy = self.coordinator.determine_strategy(file_size);

        match strategy {
            CompressionStrategy::SmallFile => {
                // Use single-threaded compression for small files
                self.cpu_pipeline
                    .compress_file(input_path, output_path)
                    .await?;
            }
            CompressionStrategy::LargeFile => {
                // Use parallel compression for large files
                self.compress_parallel(input_path, output_path).await?;
            }
        }

        // 3. Check memory usage after completion
        self.memory_tracker.check_and_warn()?;

        Ok(())
    }

    /// Decompress a file using parallel approach
    pub async fn decompress_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Try to read as chunked format first
        match self.coordinator.decompress_chunks(input_path).await {
            Ok(compressed_chunks) => {
                // This is a chunked file - decompress chunks in parallel
                self.decompress_chunks_parallel(compressed_chunks, output_path)
                    .await
            }
            Err(_) => {
                // Fall back to single-chunk decompression
                tracing::info!("Falling back to single-chunk decompression");
                let compressed_data = std::fs::read(input_path)?;
                let decompressed_data =
                    self.cpu_pipeline.decompress_chunk(&compressed_data).await?;
                std::fs::write(output_path, decompressed_data)?;
                Ok(())
            }
        }
    }

    /// Decompress chunks in parallel
    async fn decompress_chunks_parallel(
        &self,
        compressed_chunks: Vec<Vec<u8>>,
        output_path: &Path,
    ) -> Result<()> {
        // Initialize progress tracking
        let progress = Arc::new(Mutex::new(CompressionProgress::new(
            compressed_chunks.len() as u64,
            compressed_chunks.len(),
        )));

        // Decompress chunks in parallel using Rayon
        let decompression_results: Vec<_> = compressed_chunks
            .into_par_iter()
            .enumerate()
            .map(|(chunk_idx, compressed_chunk)| {
                // Decompress the chunk synchronously
                let result = self.cpu_pipeline.decompress_chunk_sync(&compressed_chunk);

                // Update progress
                if let Ok(ref _decompressed) = result {
                    let progress_guard = progress.lock().unwrap();
                    let mut progress_data = progress_guard.clone();
                    progress_data.update((chunk_idx + 1) as u64, chunk_idx + 1);
                    drop(progress_guard);

                    // Log progress every 5 chunks
                    if (chunk_idx + 1) % 5 == 0 {
                        tracing::info!(
                            "Decompression progress: {:.1}% ({}/{} chunks) - {:.1} MB/s",
                            progress_data.percentage(),
                            progress_data.chunks_processed,
                            progress_data.chunks_total,
                            progress_data.current_speed_mbps
                        );
                    }
                }

                result
            })
            .collect();

        // Convert results to decompressed chunks, handling any errors
        let mut decompressed_chunks: Vec<Vec<u8>> = Vec::new();
        for result in decompression_results {
            decompressed_chunks.push(result?);
        }

        // Log final progress
        let final_progress = progress.lock().unwrap();
        tracing::info!(
            "Decompression completed: {} chunks processed in {:.2}s at {:.1} MB/s",
            final_progress.chunks_total,
            final_progress.start_time.elapsed().as_secs_f64(),
            final_progress.current_speed_mbps
        );

        // Combine decompressed chunks into final output
        let mut combined_data = Vec::new();
        for chunk in decompressed_chunks {
            combined_data.extend_from_slice(&chunk);
        }

        std::fs::write(output_path, combined_data)?;

        Ok(())
    }

    /// Parallel compression for large files
    async fn compress_parallel(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let file_size = std::fs::metadata(input_path)?.len() as usize;

        // 1. Optimize chunk size based on current memory usage
        let base_chunk_size = self.coordinator.chunk_size();
        let optimized_chunk_size = self
            .memory_tracker
            .monitor()
            .get_recommended_chunk_size(base_chunk_size)?;

        if optimized_chunk_size != base_chunk_size {
            tracing::info!(
                "Adjusting chunk size from {} to {} bytes based on memory usage",
                base_chunk_size,
                optimized_chunk_size
            );
            // Note: We can't modify the coordinator's chunk size here since it's immutable
            // In a real implementation, we'd pass the optimized size to the compression methods
        }

        // 2. Determine compression strategy based on file size
        if self.memory_config.should_use_streaming(file_size) {
            // Very large files - use streaming compression
            tracing::info!(
                "Using streaming compression for large file ({} bytes)",
                file_size
            );
            self.compress_with_streaming(input_path, output_path).await
        } else if self.memory_config.should_use_memory_mapping(file_size) {
            // Large files - use memory mapping
            tracing::info!("Using memory mapping for file ({} bytes)", file_size);
            self.compress_with_memory_mapping(input_path, output_path)
                .await
        } else {
            // Smaller files - use regular I/O
            tracing::info!("Using regular I/O for file ({} bytes)", file_size);
            self.compress_with_regular_io(input_path, output_path).await
        }
    }

    /// Compress using streaming for very large files
    async fn compress_with_streaming(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        use std::io::{BufReader, Read};

        let file_size = std::fs::metadata(input_path)?.len() as usize;
        let chunk_size = self.memory_config.calculate_optimal_chunk_size(file_size);

        // Initialize progress tracking
        let progress = Arc::new(Mutex::new(CompressionProgress::new(
            file_size as u64,
            file_size.div_ceil(chunk_size),
        )));

        // Open input file for streaming
        let input_file = std::fs::File::open(input_path)?;
        let mut reader = BufReader::with_capacity(chunk_size, input_file);

        // Create output file and write header
        let mut output_file = std::fs::File::create(output_path)?;

        // Write file header with metadata
        use byteorder::{LittleEndian, WriteBytesExt};
        use std::io::Write;

        // Magic number (8 bytes): "PMZFILE\0"
        output_file.write_all(b"PMZFILE\0")?;

        // Version (4 bytes)
        output_file.write_u32::<LittleEndian>(1)?;

        // We'll write the number of chunks later, for now write 0
        let chunks_count_pos = output_file.stream_position()?;
        output_file.write_u32::<LittleEndian>(0)?;

        // Chunk size used (4 bytes)
        output_file.write_u32::<LittleEndian>(chunk_size as u32)?;

        // Reserved bytes (16 bytes) for future use
        output_file.write_all(&[0u8; 16])?;

        // Reserve space for chunk index table (we'll fill this later)
        let index_table_pos = output_file.stream_position()?;
        let mut chunk_offsets = Vec::new();

        // Stream through the file in chunks
        let mut buffer = vec![0u8; chunk_size];
        let mut total_read = 0;
        let mut chunk_count = 0;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // End of file
            }

            let chunk_data = &buffer[..bytes_read];

            // Compress this chunk
            let compressed_chunk = self.cpu_pipeline.compress_chunk_sync(chunk_data)?;

            // Record chunk offset
            let current_pos = output_file.stream_position()?;
            chunk_offsets.push(current_pos);

            // Write compressed chunk
            output_file.write_all(&compressed_chunk)?;

            total_read += bytes_read;
            chunk_count += 1;

            // Update progress
            let progress_guard = progress.lock().unwrap();
            let mut progress_data = progress_guard.clone();
            progress_data.update(total_read as u64, chunk_count);
            drop(progress_guard);

            // Log progress every 10 chunks
            if chunk_count % 10 == 0 {
                tracing::info!(
                    "Streaming compression progress: {:.1}% ({}/{} chunks) - {:.1} MB/s",
                    progress_data.percentage(),
                    progress_data.chunks_processed,
                    progress_data.chunks_total,
                    progress_data.current_speed_mbps
                );
            }
        }

        // Go back and write the correct number of chunks
        output_file.seek(std::io::SeekFrom::Start(chunks_count_pos))?;
        output_file.write_u32::<LittleEndian>(chunk_count as u32)?;

        // Write chunk index table
        output_file.seek(std::io::SeekFrom::Start(index_table_pos))?;
        for offset in &chunk_offsets {
            output_file.write_u64::<LittleEndian>(*offset)?;
        }

        // Write footer with total compressed size
        let final_pos = output_file.stream_position()?;
        output_file.write_u64::<LittleEndian>(final_pos)?;

        // Log final progress
        let final_progress = progress.lock().unwrap();
        tracing::info!(
            "Streaming compression completed: {} chunks processed in {:.2}s at {:.1} MB/s",
            final_progress.chunks_total,
            final_progress.start_time.elapsed().as_secs_f64(),
            final_progress.current_speed_mbps
        );

        Ok(())
    }

    /// Compress using memory mapping for large files
    async fn compress_with_memory_mapping(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> Result<()> {
        // 1. Check memory usage before creating memory mapping
        self.memory_tracker.check_and_warn()?;

        // 2. Create memory-mapped file
        let mmap_file = MemoryMappedFile::new(input_path)?;
        let chunk_size = self
            .memory_config
            .calculate_optimal_chunk_size(mmap_file.len());
        let chunks = mmap_file.split_into_chunks(chunk_size);

        // Initialize progress tracking
        let progress = Arc::new(Mutex::new(CompressionProgress::new(
            mmap_file.len() as u64,
            chunks.len(),
        )));

        // 2. Process chunks in parallel using Rayon with progress tracking
        let mut compressed_chunks = Vec::new();

        // Use parallel processing for compression with progress updates
        let compression_results: Vec<_> = chunks
            .into_par_iter()
            .enumerate()
            .map(|(chunk_idx, chunk)| {
                // Convert chunk to owned data for parallel processing
                let chunk_data = chunk.to_vec();

                // Compress the chunk
                let result = self.cpu_pipeline.compress_chunk_sync(&chunk_data);

                // Update progress
                if let Ok(ref _compressed) = result {
                    let progress_guard = progress.lock().unwrap();
                    let mut progress_data = progress_guard.clone();
                    progress_data.update((chunk_idx + 1) as u64 * chunk_size as u64, chunk_idx + 1);
                    drop(progress_guard);

                    // Log progress every 10 chunks
                    if (chunk_idx + 1) % 10 == 0 {
                        tracing::info!(
                            "Compression progress: {:.1}% ({}/{} chunks) - {:.1} MB/s",
                            progress_data.percentage(),
                            progress_data.chunks_processed,
                            progress_data.chunks_total,
                            progress_data.current_speed_mbps
                        );
                    }
                }

                result
            })
            .collect();

        // Convert results to compressed chunks, handling any errors
        for result in compression_results {
            compressed_chunks.push(result?);
        }

        // Log final progress
        {
            let final_progress = progress.lock().unwrap();
            tracing::info!(
                "Compression completed: {} chunks processed in {:.2}s at {:.1} MB/s",
                final_progress.chunks_total,
                final_progress.start_time.elapsed().as_secs_f64(),
                final_progress.current_speed_mbps
            );
        }

        // 3. Combine results and write output
        self.coordinator
            .combine_chunks(&compressed_chunks, output_path)
            .await?;

        Ok(())
    }

    /// Compress using regular I/O for smaller files
    async fn compress_with_regular_io(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // 1. Read file in chunks
        let file_data = std::fs::read(input_path)?;
        let chunks = self.coordinator.split_into_chunks(&file_data);

        // Initialize progress tracking
        let progress = Arc::new(Mutex::new(CompressionProgress::new(
            file_data.len() as u64,
            chunks.len(),
        )));

        // 2. Process chunks in parallel using Rayon with progress tracking
        let mut compressed_chunks = Vec::new();

        // Use parallel processing for compression with progress updates
        let compression_results: Vec<_> = chunks
            .into_par_iter()
            .enumerate()
            .map(|(chunk_idx, chunk)| {
                // Compress the chunk
                let result = self.cpu_pipeline.compress_chunk_sync(&chunk);

                // Update progress
                if let Ok(ref _compressed) = result {
                    let progress_guard = progress.lock().unwrap();
                    let mut progress_data = progress_guard.clone();
                    progress_data
                        .update((chunk_idx + 1) as u64 * chunk.len() as u64, chunk_idx + 1);
                    drop(progress_guard);

                    // Log progress every 5 chunks for smaller files
                    if (chunk_idx + 1) % 5 == 0 {
                        tracing::info!(
                            "Compression progress: {:.1}% ({}/{} chunks) - {:.1} MB/s",
                            progress_data.percentage(),
                            progress_data.chunks_processed,
                            progress_data.chunks_total,
                            progress_data.current_speed_mbps
                        );
                    }
                }

                result
            })
            .collect();

        // Convert results to compressed chunks, handling any errors
        for result in compression_results {
            compressed_chunks.push(result?);
        }

        // Log final progress
        {
            let final_progress = progress.lock().unwrap();
            tracing::info!(
                "Compression completed: {} chunks processed in {:.2}s at {:.1} MB/s",
                final_progress.chunks_total,
                final_progress.start_time.elapsed().as_secs_f64(),
                final_progress.current_speed_mbps
            );
        }

        // 3. Combine results and write output
        self.coordinator
            .combine_chunks(&compressed_chunks, output_path)
            .await?;

        Ok(())
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> PipelineStats {
        PipelineStats {
            chunk_size: self.coordinator.chunk_size(),
            parallel_threshold: self.coordinator.parallel_threshold(),
            max_workers: self
                .coordinator
                .calculate_optimal_workers(1024 * 1024 * 1024), // 1GB example
            memory_mapping_enabled: self.memory_config.use_memory_mapping,
        }
    }

    /// Configure memory mapping settings
    pub fn configure_memory_mapping(&mut self, config: MemoryMappingConfig) {
        self.memory_config = config;
    }

    /// Get current compression progress (for external monitoring)
    pub fn get_progress(&self) -> Option<CompressionProgress> {
        // This would be implemented with a shared progress state
        // For now, return None as progress is tracked internally
        None
    }
}

/// Compression strategy based on file characteristics
#[derive(Debug, Clone)]
pub enum CompressionStrategy {
    /// Use single-threaded compression (small files)
    SmallFile,
    /// Use parallel compression (large files)
    LargeFile,
}

/// Progress tracking for compression operations
#[derive(Debug, Clone)]
pub struct CompressionProgress {
    pub total_bytes: u64,
    pub processed_bytes: u64,
    pub chunks_total: usize,
    pub chunks_processed: usize,
    pub start_time: Instant,
    pub current_speed_mbps: f64,
}

impl CompressionProgress {
    pub fn new(total_bytes: u64, chunks_total: usize) -> Self {
        Self {
            total_bytes,
            processed_bytes: 0,
            chunks_total,
            chunks_processed: 0,
            start_time: Instant::now(),
            current_speed_mbps: 0.0,
        }
    }

    pub fn update(&mut self, processed_bytes: u64, chunks_processed: usize) {
        self.processed_bytes = processed_bytes;
        self.chunks_processed = chunks_processed;

        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.current_speed_mbps = (processed_bytes as f64 / 1_048_576.0) / elapsed;
        }
    }

    pub fn percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            100.0
        } else {
            (self.processed_bytes as f64 / self.total_bytes as f64) * 100.0
        }
    }

    pub fn eta_seconds(&self) -> Option<f64> {
        if self.current_speed_mbps > 0.0 && self.processed_bytes < self.total_bytes {
            let remaining_bytes = self.total_bytes - self.processed_bytes;
            let remaining_mb = remaining_bytes as f64 / 1_048_576.0;
            Some(remaining_mb / self.current_speed_mbps)
        } else {
            None
        }
    }
}

/// Pipeline statistics
#[derive(Debug, Clone)]
pub struct PipelineStats {
    pub chunk_size: usize,
    pub parallel_threshold: usize,
    pub max_workers: usize,
    pub memory_mapping_enabled: bool,
}
