//! Compression algorithms implementation

// Algorithm definitions

/// Supported compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    Lz4,
    Gzip,
    Zstd,
}

impl CompressionAlgorithm {
    /// Get the default compression level for this algorithm
    pub fn default_level(&self) -> u32 {
        match self {
            CompressionAlgorithm::Lz4 => 1,
            CompressionAlgorithm::Gzip => 6,
            CompressionAlgorithm::Zstd => 3,
        }
    }
    
    /// Get the maximum compression level for this algorithm
    pub fn max_level(&self) -> u32 {
        match self {
            CompressionAlgorithm::Lz4 => 16,
            CompressionAlgorithm::Gzip => 9,
            CompressionAlgorithm::Zstd => 22,
        }
    }
}

impl std::str::FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lz4" => Ok(CompressionAlgorithm::Lz4),
            "gzip" => Ok(CompressionAlgorithm::Gzip),
            "zstd" => Ok(CompressionAlgorithm::Zstd),
            _ => Err(format!("Unknown compression algorithm: {}", s)),
        }
    }
}
