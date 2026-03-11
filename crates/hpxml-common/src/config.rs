//! Configuration for HPXML parsing.

/// Configuration for parsing HPXML documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseConfig {
    /// Maximum allowed document size in bytes.
    pub max_bytes: usize,
    /// Maximum allowed XML nesting depth.
    pub max_depth: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            max_bytes: 52_428_800, // 50 MiB
            max_depth: 128,
        }
    }
}

/// Options for HPXML serialization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SerializeOptions {
    /// Whether to emit an XML declaration (`<?xml version="1.0" encoding="UTF-8"?>`)
    /// at the start of the output. Default: `false`.
    pub xml_declaration: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_max_bytes() {
        assert_eq!(ParseConfig::default().max_bytes, 52_428_800);
    }

    #[test]
    fn test_default_max_depth() {
        assert_eq!(ParseConfig::default().max_depth, 128);
    }

    #[test]
    fn test_default_serialize_options() {
        let opts = SerializeOptions::default();
        assert!(!opts.xml_declaration);
    }
}
