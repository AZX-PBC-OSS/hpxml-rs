//! Integration tests: parse -> to_xml -> parse round-trip tests.
//!
//! Verifies that parsed documents can be serialized and re-parsed to produce
//! equivalent documents.
//!
//! Uses `include_bytes!` with `CARGO_MANIFEST_DIR` for robust path resolution.

#[macro_use]
mod common;

// ============================================================================
// V4 Round-trip Tests
// ============================================================================

#[cfg(feature = "v4")]
mod v4_roundtrip {
    use hpxml_core::HpxmlSerialize;

    #[test]
    fn roundtrip_minimal() {
        let bytes = fixture!("v4/minimal.xml");
        let doc1 = hpxml_core::v4::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v4::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn roundtrip_audit() {
        let bytes = fixture!("v4/audit.xml");
        let doc1 = hpxml_core::v4::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v4::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    #[ignore = "https://github.com/Bergmann89/xsd-parser/issues/256 - xsd-parser codegen unconditionally emits xmlns:xsi on root even when no nillable values exist, breaking round-trip equality"]
    fn roundtrip_openst_base() {
        let bytes = fixture!("v4/openst-base.xml");
        let doc1 = hpxml_core::v4::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();

        // Round-trip: serialize and re-parse
        let doc2 = hpxml_core::v4::parse(&xml).unwrap();

        // Check key fields are equal
        assert_eq!(doc1.schema_version, doc2.schema_version);
        assert_eq!(doc1.building.len(), doc2.building.len());
        assert_eq!(
            doc1.xml_transaction_header_information,
            doc2.xml_transaction_header_information
        );
        assert_eq!(doc1.software_info, doc2.software_info);
        assert_eq!(doc1.contractor, doc2.contractor);
        assert_eq!(doc1.customer, doc2.customer);

        // Check building details
        if let (Some(b1), Some(b2)) = (doc1.building.get(0), doc2.building.get(0)) {
            let b1 = b1.as_ref().expect("building should not be nil");
            let b2 = b2.as_ref().expect("building should not be nil");
            assert_eq!(b1.building_id, b2.building_id);
            assert_eq!(b1.building_details, b2.building_details);
        } else {
            panic!("building should exist");
        }
    }

    #[test]
    fn roundtrip_maximal() {
        let bytes = fixture!("v4/maximal.xml");
        let doc1 = hpxml_core::v4::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v4::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }
}

// ============================================================================
// V2 Round-trip Tests
// ============================================================================

#[cfg(feature = "v2")]
mod v2_roundtrip {
    use hpxml_core::HpxmlSerialize;

    #[test]
    fn roundtrip_audit() {
        let bytes = fixture!("v2/audit.xml");
        let doc1 = hpxml_core::v2::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v2::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn roundtrip_bpi2101() {
        let bytes = fixture!("v2/bpi2101.xml");
        let doc1 = hpxml_core::v2::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v2::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn roundtrip_upgrade() {
        let bytes = fixture!("v2/upgrade.xml");
        let doc1 = hpxml_core::v2::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v2::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }
}

// ============================================================================
// V3 Round-trip Tests
// ============================================================================

#[cfg(feature = "v3")]
mod v3_roundtrip {
    use hpxml_core::HpxmlSerialize;

    #[test]
    fn roundtrip_audit() {
        let bytes = fixture!("v3/audit.xml");
        let doc1 = hpxml_core::v3::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v3::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    #[ignore = "https://github.com/Bergmann89/xsd-parser/issues/256 - xsd-parser codegen unconditionally emits xmlns:xsi on root even when no nillable values exist, breaking round-trip equality"]
    fn roundtrip_bpi2101() {
        let bytes = fixture!("v3/bpi2101.xml");
        let doc1 = hpxml_core::v3::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v3::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    #[ignore = "https://github.com/Bergmann89/xsd-parser/issues/256 - xsd-parser codegen unconditionally emits xmlns:xsi on root even when no nillable values exist, breaking round-trip equality"]
    fn roundtrip_upgrade() {
        let bytes = fixture!("v3/upgrade.xml");
        let doc1 = hpxml_core::v3::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v3::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }
}

// ============================================================================
// V5 Round-trip Tests
// ============================================================================

#[cfg(feature = "v5")]
mod v5_roundtrip {
    use hpxml_core::HpxmlSerialize;

    #[test]
    fn roundtrip_audit() {
        let bytes = fixture!("v5/audit.xml");
        let doc1 = hpxml_core::v5::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v5::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    #[ignore = "https://github.com/Bergmann89/xsd-parser/issues/256 - xsd-parser codegen unconditionally emits xmlns:xsi on root even when no nillable values exist, breaking round-trip equality"]
    fn roundtrip_bpi2101() {
        let bytes = fixture!("v5/bpi2101.xml");
        let doc1 = hpxml_core::v5::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v5::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    #[ignore = "https://github.com/Bergmann89/xsd-parser/issues/256 - xsd-parser codegen unconditionally emits xmlns:xsi on root even when no nillable values exist, breaking round-trip equality"]
    fn roundtrip_upgrade() {
        let bytes = fixture!("v5/upgrade.xml");
        let doc1 = hpxml_core::v5::parse(bytes).unwrap();
        let xml = doc1.to_xml().unwrap();
        let doc2 = hpxml_core::v5::parse(&xml).unwrap();
        assert_eq!(doc1, doc2);
    }
}
