//! Configuration for HPXML parsing.

/// Configuration for parsing HPXML documents.
///
/// The limits are security-relevant, not just performance knobs: raise them
/// only for trusted input. See [`ParseConfig::new`] for the valid ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseConfig {
    /// Maximum allowed document size in bytes (default: 50 MiB).
    pub max_bytes: usize,
    /// Maximum allowed XML nesting depth (default: 128).
    pub max_depth: usize,
}

impl ParseConfig {
    /// Maximum accepted `max_bytes` (1 GiB). Larger values offer no
    /// meaningful protection against in-memory expansion.
    pub const MAX_BYTES_LIMIT: usize = 1 << 30;
    /// Maximum accepted `max_depth` (1024). Real HPXML nests a few dozen
    /// levels; anything higher only enables stack exhaustion.
    pub const MAX_DEPTH_LIMIT: usize = 1024;

    /// Builds a validated configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if `max_bytes` is zero or above
    /// [`ParseConfig::MAX_BYTES_LIMIT`], or if `max_depth` is zero or above
    /// [`ParseConfig::MAX_DEPTH_LIMIT`].
    pub fn new(max_bytes: usize, max_depth: usize) -> Result<Self, &'static str> {
        if max_bytes == 0 || max_bytes > Self::MAX_BYTES_LIMIT {
            return Err("max_bytes must be within 1..=ParseConfig::MAX_BYTES_LIMIT");
        }
        if max_depth == 0 || max_depth > Self::MAX_DEPTH_LIMIT {
            return Err("max_depth must be within 1..=ParseConfig::MAX_DEPTH_LIMIT");
        }
        Ok(Self {
            max_bytes,
            max_depth,
        })
    }
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

    #[test]
    fn test_new_accepts_defaults() {
        let defaults = ParseConfig::default();
        let config = ParseConfig::new(defaults.max_bytes, defaults.max_depth).unwrap();
        assert_eq!(config, defaults);
    }

    #[test]
    fn test_new_rejects_zero() {
        assert!(ParseConfig::new(0, 128).is_err());
        assert!(ParseConfig::new(1024, 0).is_err());
    }

    #[test]
    fn test_new_rejects_absurd() {
        assert!(ParseConfig::new(ParseConfig::MAX_BYTES_LIMIT + 1, 128).is_err());
        assert!(ParseConfig::new(1024, ParseConfig::MAX_DEPTH_LIMIT + 1).is_err());
    }
}
