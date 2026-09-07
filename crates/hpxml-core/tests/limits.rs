//! Integration tests: depth and size limit boundary conditions.
//!
//! Tests for:
//! - DocumentTooLarge error (max_bytes)
//! - DepthLimitExceeded error (max_depth)
//! - SerializeError::Xml (via to_xml_into with a writer that fails on write)
//! - SerializeError::Io (via to_xml_into with a writer that fails on flush)
//!
//! Uses `include_bytes!` with `CARGO_MANIFEST_DIR` for robust path resolution.
//!
//! All cases target v4 fixtures, so the whole target requires the v4
//! feature (and still compiles to empty otherwise).

#![cfg(feature = "v4")]

use hpxml_core::{HpxmlSerialize, ParseConfig, ParseError};

#[macro_use]
mod common;

// ============================================================================
// Size Limit Tests (DocumentTooLarge)
// ============================================================================

#[cfg(feature = "v4")]
mod size_limits {
    use super::*;

    #[test]
    fn test_document_too_large() {
        // Use minimal.xml and set a very low size limit
        let xml = fixture!("v4/minimal.xml");
        let config = ParseConfig {
            max_bytes: 100,
            ..Default::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::DocumentTooLarge { .. }));
    }

    #[test]
    fn test_size_limit_exact() {
        // Test with limit equal to file size - should succeed
        let xml = fixture!("v4/minimal.xml");
        let size = xml.len();
        let config = ParseConfig {
            max_bytes: size,
            ..Default::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config);
        assert!(result.is_ok(), "Should succeed at exact limit");
    }

    #[test]
    fn test_size_limit_exactly_at_boundary() {
        // Test boundary: limit == size should succeed, limit - 1 should fail
        let xml = fixture!("v4/minimal.xml");
        let size = xml.len();

        // At exact size should succeed
        let config_exact = ParseConfig {
            max_bytes: size,
            ..Default::default()
        };
        assert!(hpxml_core::v4::parse_with_config(xml, &config_exact).is_ok());

        // One byte less should fail
        let config_less = ParseConfig {
            max_bytes: size - 1,
            ..Default::default()
        };
        assert!(hpxml_core::v4::parse_with_config(xml, &config_less).is_err());
    }

    #[test]
    fn test_size_limit_with_large_file() {
        // Test with maximal.xml (largest fixture)
        let xml = fixture!("v4/maximal.xml");
        let size = xml.len();

        // Should fail with 100 byte limit
        let config = ParseConfig {
            max_bytes: 100,
            ..Default::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ParseError::DocumentTooLarge { .. }
        ));

        // Should succeed with actual size limit
        let config_large = ParseConfig {
            max_bytes: size,
            ..Default::default()
        };
        assert!(hpxml_core::v4::parse_with_config(xml, &config_large).is_ok());
    }
}

// ============================================================================
// Depth Limit Tests (DepthLimitExceeded)
// ============================================================================

#[cfg(feature = "v4")]
mod depth_limits {
    use super::*;

    #[test]
    fn test_depth_limit_exceeded() {
        // Use minimal.xml with a very low depth limit
        let xml = fixture!("v4/minimal.xml");
        let config = ParseConfig {
            max_depth: 2,
            ..ParseConfig::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::DepthLimitExceeded { .. }));
    }

    #[test]
    fn test_depth_limit_at_boundary() {
        // Find the minimum depth needed to parse minimal.xml
        let xml = fixture!("v4/minimal.xml");

        // Test with depth=1 (should fail - document has deeper nesting)
        let config_shallow = ParseConfig {
            max_depth: 1,
            ..ParseConfig::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config_shallow);
        assert!(result.is_err());

        // Test with high enough depth (should succeed)
        let config_deep = ParseConfig {
            max_depth: 128,
            ..ParseConfig::default()
        };
        assert!(hpxml_core::v4::parse_with_config(xml, &config_deep).is_ok());
    }

    #[test]
    fn test_depth_limit_synthetic() {
        // Test with minimal.xml and a low depth limit
        // (We can't create valid HPXML with deep nesting programmatically easily,
        // so we reuse minimal.xml which has nested elements)
        let xml = fixture!("v4/minimal.xml");

        // Should fail with very shallow depth limit
        let config_shallow = ParseConfig {
            max_depth: 2,
            ..ParseConfig::default()
        };
        let result = hpxml_core::v4::parse_with_config(xml, &config_shallow);
        assert!(result.is_err());
        // Expect either depth limit or parsing error
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::DepthLimitExceeded { .. }));

        // Should succeed with sufficient depth limit
        let config_deep = ParseConfig {
            max_depth: 128,
            ..ParseConfig::default()
        };
        assert!(hpxml_core::v4::parse_with_config(xml, &config_deep).is_ok());
    }
}

// ============================================================================
// UTF-8 Error Tests (ParseError::Utf8)
// ============================================================================

#[cfg(feature = "v4")]
mod utf8_error {

    #[test]
    fn test_invalid_utf8_in_element_text() {
        // Create XML with invalid UTF-8 bytes inside an element's text content
        // Using bytes that are invalid UTF-8
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0">
  <Building>
    <BuildingID id="a">test\xFF\xFE</BuildingID>
  </Building>
</HPXML>"#;
        let result = hpxml_core::v4::parse(xml);
        // Should fail with either Utf8 or Xml error (depends on quick-xml behavior)
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_utf8_in_attribute() {
        // Create XML with invalid UTF-8 in an attribute value
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0">
  <Building>
    <BuildingID id="bad\xFF\xFE">test</BuildingID>
  </Building>
</HPXML>"#;
        let result = hpxml_core::v4::parse(xml);
        // Should fail with either Utf8 or Xml error
        assert!(result.is_err());
    }
}

// ============================================================================
// XML Parsing Error Tests (ParseError::Xml)
// ============================================================================

#[cfg(feature = "v4")]
mod xml_error {
    use super::*;

    #[test]
    fn test_malformed_xml() {
        // Unclosed tag
        let xml = b"<HPXML><unclosed>";
        let result = hpxml_core::v4::parse(xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ParseError::Xml { .. }));
    }

    #[test]
    fn test_malformed_xml_mismatched_tags() {
        // Mismatched closing tags
        let xml = b"<HPXML><Building></House></HPXML>";
        let result = hpxml_core::v4::parse(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_malformed_xml_no_root_close() {
        // No closing root tag
        let xml = b"<HPXML><Building>";
        let result = hpxml_core::v4::parse(xml);
        assert!(result.is_err());
    }
}

// ============================================================================
// DTD Rejection Tests (a late declaration must not slip past the guard)
// ============================================================================

#[cfg(feature = "v4")]
mod dtd_limits {
    use super::*;
    use hpxml_core::InspectError;

    fn late_doctype_doc() -> Vec<u8> {
        let mut xml = b"<?xml version=\"1.0\"?>".to_vec();
        xml.extend(std::iter::repeat_n(b' ', 8192));
        xml.extend_from_slice(
            b"<!DOCTYPE HPXML [<!ENTITY x \"x\">]><HPXML xmlns=\"http://hpxmlonline.com/2023/09\" schemaVersion=\"4.2\"/>",
        );
        xml
    }

    #[test]
    fn test_parse_rejects_late_doctype() {
        let xml = late_doctype_doc();
        let result = hpxml_core::v4::parse(&xml);
        assert!(
            matches!(result, Err(ParseError::DtdNotAllowed)),
            "late DTD must be rejected, got: {result:?}"
        );
    }

    #[test]
    fn test_inspect_rejects_late_doctype() {
        let xml = late_doctype_doc();
        let result = hpxml_core::inspect::inspect(&xml);
        assert!(
            matches!(result, Err(InspectError::DtdNotAllowed)),
            "late DTD must be rejected, got: {result:?}"
        );
    }

    #[test]
    fn test_lowercase_doctype_is_not_a_dtd() {
        let minimal = fixture!("v4/minimal.xml");
        let text = String::from_utf8_lossy(minimal);
        let (decl, rest) = text
            .split_once("?>")
            .expect("minimal.xml has an XML declaration");
        let xml = format!("{decl}?><!doctype HPXML []>{rest}");
        let result = hpxml_core::v4::parse(xml.as_bytes());
        assert!(
            result.is_ok(),
            "lowercase doctype must not trigger DTD rejection, got: {result:?}"
        );
    }
}

// ============================================================================
// Malformed and Edge Input Tests (parse-level error taxonomy)
// ============================================================================

#[cfg(feature = "v4")]
mod parse_errors {
    use super::*;

    #[test]
    fn test_parse_empty_is_xml_error() {
        let result = hpxml_core::v4::parse(b"");
        assert!(
            matches!(result, Err(ParseError::Xml { .. })),
            "empty input must be an XML error, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_malformed_fixture_is_xml_error() {
        let result = hpxml_core::v4::parse(fixture!("negative/malformed.xml"));
        assert!(
            matches!(result, Err(ParseError::Xml { .. })),
            "malformed fixture must be an XML error, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_unknown_namespace_fixture_is_xml_error() {
        let result = hpxml_core::v4::parse(fixture!("negative/unknown-namespace.xml"));
        assert!(
            matches!(result, Err(ParseError::Xml { .. })),
            "unknown namespace must fail parsing, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_missing_namespace_fixture_is_xml_error() {
        let result = hpxml_core::v4::parse(fixture!("negative/missing-namespace.xml"));
        assert!(
            matches!(result, Err(ParseError::Xml { .. })),
            "missing namespace must fail parsing, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_ochre_mismatch_fixture_is_xml_error() {
        let result = hpxml_core::v4::parse(fixture!("edge/ochre-mismatch.xml"));
        assert!(
            matches!(result, Err(ParseError::Xml { .. })),
            "namespace-foreign fixture must fail parsing, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_zero_depth_limit() {
        let config = ParseConfig {
            max_depth: 0,
            ..ParseConfig::default()
        };
        let result = hpxml_core::v4::parse_with_config(fixture!("v4/minimal.xml"), &config);
        assert!(
            matches!(
                result,
                Err(ParseError::DepthLimitExceeded { depth: 1, limit: 0 })
            ),
            "zero depth limit must reject the root element, got: {result:?}"
        );
    }

    #[test]
    fn test_parse_bom_fixture() {
        let doc = hpxml_core::v4::parse(fixture!("edge/bom-minimal.xml"))
            .expect("BOM-prefixed fixture must parse");
        assert!(!doc.building.is_empty());
    }

    #[test]
    fn test_parse_entity_refs_decoded() {
        let doc = hpxml_core::v4::parse(fixture!("edge/entity-refs.xml"))
            .expect("entity-refs fixture must parse");
        assert_eq!(
            doc.xml_transaction_header_information
                .xml_generated_by
                .content,
            "Test & ampersand <less-than>"
        );
    }
}

// ============================================================================
// Serialization Error Tests
// ============================================================================

#[cfg(feature = "v4")]
mod serialize_limits {
    use super::*;

    #[test]
    fn test_serialize_error_xml_from_write_failure() {
        // Write failures during serialization are wrapped by quick_xml as Xml errors
        let xml = fixture!("v4/minimal.xml");
        let hpxml = hpxml_core::v4::parse(xml).unwrap();

        let mut writer = common::io_fail::FailingWriter;
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_core::SerializeError::Xml(_))));
    }

    #[test]
    fn test_serialize_error_io_from_flush_failure() {
        // Flush failures after serialization produce SerializeError::Io
        let xml = fixture!("v4/minimal.xml");
        let hpxml = hpxml_core::v4::parse(xml).unwrap();

        let mut writer = common::io_fail::FailingFlusher { buf: Vec::new() };
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_core::SerializeError::Io(_))));
    }
}
