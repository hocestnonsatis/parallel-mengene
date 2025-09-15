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
