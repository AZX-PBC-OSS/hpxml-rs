//! Integration tests: inspect() function tests.
//!
//! Tests the pre-parsing inspection functionality that extracts version
//! information from the root element.
//!
//! Uses `include_bytes!` with `CARGO_MANIFEST_DIR` for robust path resolution.

use hpxml_core::{HpxmlVersion, InspectError};

#[macro_use]
mod common;

// ============================================================================
// V4 Inspection Tests
// ============================================================================

#[cfg(feature = "v4")]
mod v4_inspect {
    use super::*;

    #[test]
    fn inspect_minimal() {
        let xml = fixture!("v4/minimal.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }

    #[test]
    fn inspect_audit() {
        let xml = fixture!("v4/audit.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }

    #[test]
    fn inspect_bpi2101() {
        let xml = fixture!("v4/bpi2101.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }

    #[test]
    fn inspect_maximal() {
        let xml = fixture!("v4/maximal.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }

    #[test]
    fn inspect_upgrade() {
        let xml = fixture!("v4/upgrade.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }
}

// ============================================================================
// V2 Inspection Tests
// ============================================================================

#[cfg(feature = "v2")]
mod v2_inspect {
    use super::*;

    #[test]
    fn inspect_audit() {
        let xml = fixture!("v2/audit.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V2);
        assert!(info.schema_version.starts_with("2."));
    }

    #[test]
    fn inspect_bpi2101() {
        let xml = fixture!("v2/bpi2101.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V2);
        assert!(info.schema_version.starts_with("2."));
    }

    #[test]
    fn inspect_upgrade() {
        let xml = fixture!("v2/upgrade.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V2);
        assert!(info.schema_version.starts_with("2."));
    }
}

// ============================================================================
// V3 Inspection Tests
// ============================================================================

#[cfg(feature = "v3")]
mod v3_inspect {
    use super::*;

    #[test]
    fn inspect_audit() {
        let xml = fixture!("v3/audit.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
        assert!(info.schema_version.starts_with("3."));
    }

    #[test]
    fn inspect_bpi2101() {
        let xml = fixture!("v3/bpi2101.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
        assert!(info.schema_version.starts_with("3."));
    }

    #[test]
    fn inspect_upgrade() {
        let xml = fixture!("v3/upgrade.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
        assert!(info.schema_version.starts_with("3."));
    }
}

// ============================================================================
// V5 Inspection Tests
// ============================================================================

#[cfg(feature = "v5")]
mod v5_inspect {
    use super::*;

    #[test]
    fn inspect_audit() {
        let xml = fixture!("v5/audit.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V5);
        assert!(info.schema_version.starts_with("5."));
    }

    #[test]
    fn inspect_bpi2101() {
        let xml = fixture!("v5/bpi2101.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V5);
        assert!(info.schema_version.starts_with("5."));
    }

    #[test]
    fn inspect_upgrade() {
        let xml = fixture!("v5/upgrade.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V5);
        assert!(info.schema_version.starts_with("5."));
    }
}

// ============================================================================
// Negative Test Cases (InspectError variants)
// ============================================================================

mod negative_inspect {
    use super::*;

    #[test]
    fn inspect_not_hpxml() {
        let xml = fixture!("negative/not-hpxml.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::NotHpxml { .. })));
    }

    #[test]
    fn inspect_unknown_namespace() {
        let xml = fixture!("negative/unknown-namespace.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::UnknownNamespace(_))));
    }

    #[test]
    fn inspect_missing_namespace() {
        let xml = fixture!("negative/missing-namespace.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn inspect_missing_schema_version() {
        let xml = fixture!("negative/missing-schema-version.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn inspect_bad_schema_version() {
        let xml = fixture!("negative/bad-schema-version.xml");
        let result = hpxml_core::inspect::inspect(xml);
        // Empty schemaVersion is treated as missing
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn inspect_malformed() {
        let xml = fixture!("negative/malformed.xml");
        let result = hpxml_core::inspect::inspect(xml);
        // Malformed XML without namespace returns MissingNamespace first
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn inspect_rejects_billion_laughs() {
        let xml = fixture!("negative/billion-laughs.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }

    #[test]
    fn inspect_rejects_xxe() {
        let xml = fixture!("negative/xxe.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }

    #[test]
    fn inspect_rejects_dtd_internal() {
        let xml = fixture!("negative/dtd-internal.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

mod edge_inspect {
    use super::*;

    #[test]
    fn inspect_bom_prefixed() {
        // BOM-prefixed file should parse successfully
        let xml = fixture!("edge/bom-minimal.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
    }

    #[test]
    fn inspect_ochre_mismatch() {
        // OCHRE pre-release bug: namespace says 2019/10 (v3) but schemaVersion says 4.0
        let xml = fixture!("edge/ochre-mismatch.xml");
        let result = hpxml_core::inspect::inspect(xml);
        assert!(matches!(
            result,
            Err(InspectError::NamespaceVersionMismatch { .. })
        ));
    }

    #[test]
    fn inspect_entity_refs() {
        // Entity references should parse successfully
        let xml = fixture!("edge/entity-refs.xml");
        let info = hpxml_core::inspect::inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
    }
}
