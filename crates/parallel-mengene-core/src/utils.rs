//! Utility functions for parallel-mengene-core

use crate::error::{Error, Result};
use std::path::Path;

/// Calculate the compression ratio
pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    if original_size == 0 {
        0.0
    } else {
        compressed_size as f64 / original_size as f64
    }
}

/// Calculate the space savings percentage
pub fn space_savings(original_size: usize, compressed_size: usize) -> f64 {
    if original_size == 0 {
        0.0
    } else {
        (1.0 - compression_ratio(original_size, compressed_size)) * 100.0
    }
}

/// Get file size in a human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate that a path exists and is a file
pub fn validate_file_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(Error::InvalidInput(format!("File does not exist: {}", path.display())));
    }
    
    if !path.is_file() {
        return Err(Error::InvalidInput(format!("Path is not a file: {}", path.display())));
    }
    
    Ok(())
}

/// Get the number of available CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}
