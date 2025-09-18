//! Compression algorithms implementation

// Algorithm definitions

/// Supported compression algorithms (single, efficient algorithm)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompressionAlgorithm {
    /// LZ4 fast compression algorithm
    Lz4,
}

impl CompressionAlgorithm {
    /// Get the default compression level for this algorithm
    pub fn default_level(&self) -> u32 {
        match self {
            CompressionAlgorithm::Lz4 => 1,
        }
    }

    /// Get the maximum compression level for this algorithm
    pub fn max_level(&self) -> u32 {
        match self {
            CompressionAlgorithm::Lz4 => 9,
        }
    }
}

impl std::str::FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lz4" => Ok(CompressionAlgorithm::Lz4),
            _ => Err(format!("Unknown compression algorithm: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_algorithm_default_levels() {
        assert_eq!(CompressionAlgorithm::Lz4.default_level(), 1);
    }

    #[test]
    fn test_compression_algorithm_max_levels() {
        assert_eq!(CompressionAlgorithm::Lz4.max_level(), 9);
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
