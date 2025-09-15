//! Error types for parallel-mengene-core

use thiserror::Error;

/// Result type alias for parallel-mengene-core operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for parallel-mengene-core
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Compression error: {0}")]
    Compression(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Memory mapping error: {0}")]
    MemoryMapping(String),
    
    #[error("Threading error: {0}")]
    Threading(String),
    
    #[error("GPU not available: {0}")]
    GpuNotAvailable(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error_display() {
        let io_error = Error::Io(IoError::new(ErrorKind::NotFound, "File not found"));
        assert!(format!("{}", io_error).contains("IO error"));
        assert!(format!("{}", io_error).contains("File not found"));

        let compression_error = Error::Compression("Test compression error".to_string());
        assert!(format!("{}", compression_error).contains("Compression error"));
        assert!(format!("{}", compression_error).contains("Test compression error"));

        let invalid_input_error = Error::InvalidInput("Invalid data".to_string());
        assert!(format!("{}", invalid_input_error).contains("Invalid input"));
        assert!(format!("{}", invalid_input_error).contains("Invalid data"));

        let memory_mapping_error = Error::MemoryMapping("Mapping failed".to_string());
        assert!(format!("{}", memory_mapping_error).contains("Memory mapping error"));
        assert!(format!("{}", memory_mapping_error).contains("Mapping failed"));

        let threading_error = Error::Threading("Thread creation failed".to_string());
        assert!(format!("{}", threading_error).contains("Threading error"));
        assert!(format!("{}", threading_error).contains("Thread creation failed"));

        let gpu_error = Error::GpuNotAvailable("No GPU found".to_string());
        assert!(format!("{}", gpu_error).contains("GPU not available"));
        assert!(format!("{}", gpu_error).contains("No GPU found"));
    }

    #[test]
    fn test_error_debug() {
        let error = Error::Compression("Debug test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Compression"));
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = IoError::new(ErrorKind::PermissionDenied, "Permission denied");
        let error: Error = io_error.into();
        
        match error {
            Error::Io(e) => {
                assert_eq!(e.kind(), ErrorKind::PermissionDenied);
                assert!(e.to_string().contains("Permission denied"));
            }
            _ => panic!("Expected IO error"),
        }
    }

    #[test]
    fn test_error_equality() {
        let error1 = Error::Compression("Test".to_string());
        let error2 = Error::Compression("Test".to_string());
        let error3 = Error::Compression("Different".to_string());
        let error4 = Error::InvalidInput("Test".to_string());

        // Note: Error doesn't implement PartialEq, so we can't test equality directly
        // But we can test that the same error types with same messages are created
        match (&error1, &error2) {
            (Error::Compression(msg1), Error::Compression(msg2)) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Expected compression errors"),
        }

        match (&error1, &error3) {
            (Error::Compression(msg1), Error::Compression(msg2)) => {
                assert_ne!(msg1, msg2);
            }
            _ => panic!("Expected compression errors"),
        }

        match (&error1, &error4) {
            (Error::Compression(_), Error::InvalidInput(_)) => {
                // Different error types
            }
            _ => panic!("Expected different error types"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        fn success_function() -> Result<u32> {
            Ok(42)
        }

        fn error_function() -> Result<u32> {
            Err(Error::Compression("Test error".to_string()))
        }

        assert_eq!(success_function().unwrap(), 42);
        assert!(error_function().is_err());
    }
}
