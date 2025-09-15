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
        return Err(Error::InvalidInput(format!(
            "File does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(Error::InvalidInput(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    Ok(())
}

/// Get the number of available CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_compression_ratio() {
        assert_eq!(compression_ratio(100, 50), 0.5);
        assert_eq!(compression_ratio(100, 100), 1.0);
        assert_eq!(compression_ratio(100, 200), 2.0);
        assert_eq!(compression_ratio(0, 100), 0.0);
    }

    #[test]
    fn test_space_savings() {
        assert_eq!(space_savings(100, 50), 50.0); // 50% savings
        assert_eq!(space_savings(100, 100), 0.0); // 0% savings
        assert_eq!(space_savings(100, 200), -100.0); // -100% (expansion)
        assert_eq!(space_savings(0, 100), 0.0); // 0% for zero original size
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1023), "1023 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_format_file_size_edge_cases() {
        // Test exact thresholds
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");

        // Test just under thresholds
        assert_eq!(format_file_size(1023), "1023 B");
        assert_eq!(format_file_size(1048575), "1024.0 KB");

        // Test large numbers
        assert_eq!(format_file_size(1099511627776), "1.0 TB");
    }

    #[test]
    fn test_validate_file_path_success() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");

        // Create a test file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();
        drop(file);

        // Should succeed
        assert!(validate_file_path(&file_path).is_ok());
    }

    #[test]
    fn test_validate_file_path_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");

        // Should fail with InvalidInput error
        let result = validate_file_path(&file_path);
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::InvalidInput(msg) => {
                assert!(msg.contains("File does not exist"));
                assert!(msg.contains("nonexistent.txt"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_validate_file_path_directory() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        // Should fail because it's a directory, not a file
        let result = validate_file_path(dir_path);
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::InvalidInput(msg) => {
                assert!(msg.contains("Path is not a file"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_get_cpu_count() {
        let cpu_count = get_cpu_count();
        assert!(cpu_count > 0);
        assert!(cpu_count <= 1024); // Reasonable upper bound
    }

    #[test]
    fn test_compression_ratio_precision() {
        // Test floating point precision
        let ratio = compression_ratio(3, 1);
        assert!((ratio - 0.3333333333333333).abs() < 1e-10);
    }

    #[test]
    fn test_space_savings_precision() {
        // Test floating point precision
        let savings = space_savings(3, 1);
        assert!((savings - 66.66666666666667).abs() < 1e-10);
    }

    #[test]
    fn test_format_file_size_precision() {
        // Test that we get reasonable precision
        let formatted = format_file_size(1536);
        assert_eq!(formatted, "1.5 KB");

        let formatted = format_file_size(1537);
        assert_eq!(formatted, "1.5 KB");

        let formatted = format_file_size(1538);
        assert_eq!(formatted, "1.5 KB");
    }
}
