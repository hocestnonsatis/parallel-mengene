//! Compression algorithms implementation

// Algorithm definitions

/// Supported compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CompressionAlgorithm {
    /// LZ4 fast compression algorithm (speed-focused)
    Lz4,
    /// Gzip compression algorithm (balanced speed/ratio)
    Gzip,
    /// Zstandard compression algorithm (modern, efficient)
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
            CompressionAlgorithm::Lz4 => 9,
            CompressionAlgorithm::Gzip => 9,
            CompressionAlgorithm::Zstd => 22,
        }
    }

    /// Get the recommended compression level for this algorithm
    pub fn recommended_level(&self) -> u32 {
        match self {
            CompressionAlgorithm::Lz4 => 1,  // Speed-focused
            CompressionAlgorithm::Gzip => 6, // Balanced
            CompressionAlgorithm::Zstd => 3, // Balanced
        }
    }

    /// Get the file extension for this algorithm
    pub fn file_extension(&self) -> &'static str {
        match self {
            CompressionAlgorithm::Lz4 => "lz4",
            CompressionAlgorithm::Gzip => "gz",
            CompressionAlgorithm::Zstd => "zst",
        }
    }

    /// Get a human-readable description of this algorithm
    pub fn description(&self) -> &'static str {
        match self {
            CompressionAlgorithm::Lz4 => "LZ4 - Fast compression with moderate ratio",
            CompressionAlgorithm::Gzip => "Gzip - Balanced compression with good ratio",
            CompressionAlgorithm::Zstd => "Zstandard - Modern compression with excellent ratio",
        }
    }
}

impl std::str::FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lz4" => Ok(CompressionAlgorithm::Lz4),
            "gzip" | "gz" => Ok(CompressionAlgorithm::Gzip),
            "zstd" | "zstandard" => Ok(CompressionAlgorithm::Zstd),
            _ => Err(format!(
                "Unknown compression algorithm: {}. Supported: lz4, gzip, zstd",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_algorithm_default_levels() {
        assert_eq!(CompressionAlgorithm::Lz4.default_level(), 1);
        assert_eq!(CompressionAlgorithm::Gzip.default_level(), 6);
        assert_eq!(CompressionAlgorithm::Zstd.default_level(), 3);
    }

    #[test]
    fn test_compression_algorithm_max_levels() {
        assert_eq!(CompressionAlgorithm::Lz4.max_level(), 9);
        assert_eq!(CompressionAlgorithm::Gzip.max_level(), 9);
        assert_eq!(CompressionAlgorithm::Zstd.max_level(), 22);
    }

    #[test]
    fn test_compression_algorithm_from_str() {
        assert_eq!(
            "lz4".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Lz4
        );
        assert_eq!(
            "LZ4".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Lz4
        );
        assert_eq!(
            "gzip".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Gzip
        );
        assert_eq!(
            "gz".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Gzip
        );
        assert_eq!(
            "zstd".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Zstd
        );
        assert_eq!(
            "zstandard".parse::<CompressionAlgorithm>().unwrap(),
            CompressionAlgorithm::Zstd
        );
    }

    #[test]
    fn test_compression_algorithm_from_str_invalid() {
        assert!("invalid".parse::<CompressionAlgorithm>().is_err());
        assert!("".parse::<CompressionAlgorithm>().is_err());
        assert!("pm".parse::<CompressionAlgorithm>().is_err());
    }

    #[test]
    fn test_compression_algorithm_serialization() {
        let algorithm = CompressionAlgorithm::Lz4;
        let serialized = serde_json::to_string(&algorithm).unwrap();
        let deserialized: CompressionAlgorithm = serde_json::from_str(&serialized).unwrap();
        assert_eq!(algorithm, deserialized);
    }

    #[test]
    fn test_compression_algorithm_equality() {
        assert_eq!(CompressionAlgorithm::Lz4, CompressionAlgorithm::Lz4);
    }

    #[test]
    fn test_compression_algorithm_debug() {
        let algorithm = CompressionAlgorithm::Lz4;
        let debug_str = format!("{:?}", algorithm);
        assert!(debug_str.contains("Lz4"));
    }

    #[test]
    fn test_compression_algorithm_clone() {
        let original = CompressionAlgorithm::Lz4;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_compression_algorithm_copy() {
        let algorithm = CompressionAlgorithm::Lz4;
        let copied = algorithm;
        assert_eq!(algorithm, copied);
    }
}
