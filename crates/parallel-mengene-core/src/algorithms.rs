//! Compression algorithms implementation

// Algorithm definitions

/// Supported compression algorithms (single, in-house algorithm)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompressionAlgorithm {
    /// Parallel Mengene custom algorithm (PM)
    Pm,
}

impl CompressionAlgorithm {
    /// Get the default compression level for this algorithm
    pub fn default_level(&self) -> u32 { 1 }

    /// Get the maximum compression level for this algorithm
    pub fn max_level(&self) -> u32 { 1 }
}

impl std::str::FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pm" => Ok(CompressionAlgorithm::Pm),
            _ => Err(format!("Unknown compression algorithm: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_algorithm_default_levels() {
        assert_eq!(CompressionAlgorithm::Pm.default_level(), 1);
    }

    #[test]
    fn test_compression_algorithm_max_levels() {
        assert_eq!(CompressionAlgorithm::Pm.max_level(), 1);
    }

    #[test]
    fn test_compression_algorithm_from_str() {
        assert_eq!("pm".parse::<CompressionAlgorithm>().unwrap(), CompressionAlgorithm::Pm);
        assert_eq!("PM".parse::<CompressionAlgorithm>().unwrap(), CompressionAlgorithm::Pm);
    }

    #[test]
    fn test_compression_algorithm_from_str_invalid() {
        assert!("invalid".parse::<CompressionAlgorithm>().is_err());
        assert!("".parse::<CompressionAlgorithm>().is_err());
        assert!("lz5".parse::<CompressionAlgorithm>().is_err());
    }

    #[test]
    fn test_compression_algorithm_serialization() {
        let algorithm = CompressionAlgorithm::Pm;
        let serialized = serde_json::to_string(&algorithm).unwrap();
        let deserialized: CompressionAlgorithm = serde_json::from_str(&serialized).unwrap();
        assert_eq!(algorithm, deserialized);
    }

    #[test]
    fn test_compression_algorithm_equality() {
        assert_eq!(CompressionAlgorithm::Pm, CompressionAlgorithm::Pm);
    }

    #[test]
    fn test_compression_algorithm_debug() {
        let algorithm = CompressionAlgorithm::Pm;
        let debug_str = format!("{:?}", algorithm);
        assert!(debug_str.contains("Pm"));
    }

    #[test]
    fn test_compression_algorithm_clone() {
        let original = CompressionAlgorithm::Pm;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_compression_algorithm_copy() {
        let algorithm = CompressionAlgorithm::Pm;
        let copied = algorithm;
        assert_eq!(algorithm, copied);
    }
}
