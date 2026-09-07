pub use hpxml_common::*;

pub(crate) mod dtd_guard;
pub mod inspect;
pub(crate) mod macros;
#[cfg(test)]
mod test_helpers;
pub mod traits;

#[cfg(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5"))]
#[allow(dead_code)]
pub(crate) mod reader;

pub use traits::HpxmlSerialize;

#[cfg(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5"))]
pub use xsd_parser_types::xml::Nillable;

#[cfg(feature = "v2")]
pub mod v2;

#[cfg(feature = "v3")]
pub mod v3;

#[cfg(feature = "v4")]
pub mod v4;

#[cfg(feature = "v5")]
pub mod v5;

/// Returns the HPXML versions available in this build.
///
/// The result depends on which cargo features are enabled.
/// With default features, this returns `[V4]`.
/// With `features = ["full"]`, this returns all versions.
pub fn available_versions() -> &'static [HpxmlVersion] {
    &[
        #[cfg(feature = "v2")]
        HpxmlVersion::V2,
        #[cfg(feature = "v3")]
        HpxmlVersion::V3,
        #[cfg(feature = "v4")]
        HpxmlVersion::V4,
        #[cfg(feature = "v5")]
        HpxmlVersion::V5,
    ]
}

#[cfg(test)]
mod available_versions_tests {
    use super::*;

    #[test]
    fn returns_nonempty() {
        assert!(!available_versions().is_empty());
    }

    #[test]
    fn contains_v4_with_default_features() {
        assert!(available_versions().contains(&HpxmlVersion::V4));
    }

    #[test]
    #[cfg(feature = "full")]
    fn contains_all_with_full() {
        let versions = available_versions();
        assert!(versions.contains(&HpxmlVersion::V2));
        assert!(versions.contains(&HpxmlVersion::V3));
        assert!(versions.contains(&HpxmlVersion::V4));
        assert!(versions.contains(&HpxmlVersion::V5));
    }
}
