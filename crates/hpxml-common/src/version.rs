//! HPXML version types and namespace mapping.

use std::fmt;

/// Supported HPXML versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HpxmlVersion {
    /// HPXML version 2.x
    V2,
    /// HPXML version 3.x
    V3,
    /// HPXML version 4.x
    V4,
    /// HPXML version 5.x
    V5,
}

impl HpxmlVersion {
    /// Returns the major version number (e.g. `V4` returns `4`).
    ///
    /// Returns `0` for unknown variants added in future versions of the library
    /// (the enum is `#[non_exhaustive]`).
    #[allow(unreachable_patterns)]
    pub fn major(&self) -> u32 {
        match self {
            HpxmlVersion::V2 => 2,
            HpxmlVersion::V3 => 3,
            HpxmlVersion::V4 => 4,
            HpxmlVersion::V5 => 5,
            _ => 0,
        }
    }
}

impl fmt::Display for HpxmlVersion {
    #[allow(unreachable_patterns)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HpxmlVersion::V2 => write!(f, "2"),
            HpxmlVersion::V3 => write!(f, "3"),
            HpxmlVersion::V4 => write!(f, "4"),
            HpxmlVersion::V5 => write!(f, "5"),
            _ => write!(f, "unknown"),
        }
    }
}

/// Information extracted from an HPXML document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HpxmlFileInfo {
    /// The HPXML version.
    pub version: HpxmlVersion,
    /// The schema version string from the document.
    pub schema_version: String,
    /// The namespace URI from the document.
    pub namespace: String,
}

#[doc(hidden)]
pub const NAMESPACE_MAP: &[(&str, HpxmlVersion)] = &[
    ("http://hpxmlonline.com/2014/6", HpxmlVersion::V2),
    ("http://hpxmlonline.com/2019/10", HpxmlVersion::V3),
    ("http://hpxmlonline.com/2023/09", HpxmlVersion::V4),
    ("http://hpxmlonline.com/2025/12", HpxmlVersion::V5),
];

#[doc(hidden)]
pub fn version_from_namespace(namespace: &str) -> Option<HpxmlVersion> {
    NAMESPACE_MAP
        .iter()
        .find(|(ns, _)| *ns == namespace)
        .map(|(_, v)| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_map_v2() {
        assert_eq!(
            version_from_namespace("http://hpxmlonline.com/2014/6"),
            Some(HpxmlVersion::V2)
        );
    }

    #[test]
    fn test_namespace_map_v3() {
        assert_eq!(
            version_from_namespace("http://hpxmlonline.com/2019/10"),
            Some(HpxmlVersion::V3)
        );
    }

    #[test]
    fn test_namespace_map_v4() {
        assert_eq!(
            version_from_namespace("http://hpxmlonline.com/2023/09"),
            Some(HpxmlVersion::V4)
        );
    }

    #[test]
    fn test_namespace_map_v5() {
        assert_eq!(
            version_from_namespace("http://hpxmlonline.com/2025/12"),
            Some(HpxmlVersion::V5)
        );
    }

    #[test]
    fn test_namespace_map_unknown() {
        assert_eq!(version_from_namespace("http://example.com/"), None);
    }

    #[test]
    fn test_major() {
        assert_eq!(HpxmlVersion::V2.major(), 2);
        assert_eq!(HpxmlVersion::V3.major(), 3);
        assert_eq!(HpxmlVersion::V4.major(), 4);
        assert_eq!(HpxmlVersion::V5.major(), 5);
    }
}
