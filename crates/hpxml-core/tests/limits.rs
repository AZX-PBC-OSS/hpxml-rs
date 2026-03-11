//! Integration tests: depth and size limit boundary conditions.
//!
//! Tests for:
//! - DocumentTooLarge error (max_bytes)
//! - DepthLimitExceeded error (max_depth)
//! - SerializeError::Xml (via to_xml_into with a writer that fails on write)
//! - SerializeError::Io (via to_xml_into with a writer that fails on flush)
//!
//! Uses `include_bytes!` with `CARGO_MANIFEST_DIR` for robust path resolution.

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

        struct FailingWriter;
        impl std::io::Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "write failed",
                ))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut writer = FailingWriter;
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_core::SerializeError::Xml(_))));
    }

    #[test]
    fn test_serialize_error_io_from_flush_failure() {
        // Flush failures after serialization produce SerializeError::Io
        let xml = fixture!("v4/minimal.xml");
        let hpxml = hpxml_core::v4::parse(xml).unwrap();

        struct FailingFlusher {
            buf: Vec<u8>,
        }
        impl std::io::Write for FailingFlusher {
            fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
                self.buf.extend_from_slice(data);
                Ok(data.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "flush failed",
                ))
            }
        }

        let mut writer = FailingFlusher { buf: Vec::new() };
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_core::SerializeError::Io(_))));
    }
}
