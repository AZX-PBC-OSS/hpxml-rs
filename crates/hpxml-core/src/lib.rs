#[doc(hidden)]
pub use hpxml_common::version_from_namespace;
pub use hpxml_common::{
    HpxmlFileInfo, HpxmlVersion, InspectError, ParseConfig, ParseError, SerializeError,
    SerializeOptions,
};

pub(crate) mod dtd_guard;
pub mod inspect;
pub(crate) mod macros;
#[cfg(test)]
mod test_helpers;
pub mod traits;

#[cfg(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5"))]
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
/// `hpxml-core` has no default features, so a bare
/// `cargo test -p hpxml-core` build returns an empty slice.
/// The `hpxml` facade defaults to `v4`; with `features = ["full"]`
/// this returns all versions.
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
    #[cfg(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5"))]
    fn returns_nonempty() {
        assert!(!available_versions().is_empty());
    }

    #[test]
    #[cfg(not(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5")))]
    fn returns_empty_with_no_features() {
        assert!(available_versions().is_empty());
    }

    #[test]
    fn contains_v4_iff_enabled() {
        // The hpxml facade enables v4 by default; bare hpxml-core has no
        // default features, so an empty slice is correct there.
        #[cfg(feature = "v4")]
        assert!(available_versions().contains(&HpxmlVersion::V4));
        #[cfg(not(feature = "v4"))]
        assert!(!available_versions().contains(&HpxmlVersion::V4));
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
