//! HPXML document inspection (pre-parsing validation).
//!
//! This module provides the `inspect()` function which reads only the root XML
//! element to determine the HPXML version without fully parsing the document.

use quick_xml::NsReader;
use quick_xml::events::Event;

use crate::{HpxmlFileInfo, InspectError, ParseConfig, dtd_guard, version_from_namespace};

/// Inspect an HPXML document to extract version information from the root element.
///
/// Uses [`ParseConfig::default()`] for size limits. See [`inspect_with_config`] to
/// supply a custom configuration.
///
/// # Errors
///
/// Returns an error if:
/// - The document exceeds the size limit (`DocumentTooLarge`)
/// - The XML is malformed (`MalformedXml`)
/// - The root element is not `<HPXML>` (`NotHpxml`)
/// - The namespace is missing or unrecognized (`MissingNamespace`, `UnknownNamespace`)
/// - The schema version is missing or empty (`MissingSchemaVersion`)
/// - The namespace and schema version don't match (`NamespaceVersionMismatch`)
pub fn inspect(bytes: &[u8]) -> Result<HpxmlFileInfo, InspectError> {
    inspect_with_config(bytes, &ParseConfig::default())
}

/// Inspect an HPXML document to extract version information from the root element,
/// using a custom [`ParseConfig`] for size limits.
///
/// This is a lightweight operation that reads only the root element, making it
/// useful for routing documents to the appropriate parser version.
///
/// # Errors
///
/// Returns an error if:
/// - The document exceeds the size limit (`DocumentTooLarge`)
/// - The XML is malformed (`MalformedXml`)
/// - The root element is not `<HPXML>` (`NotHpxml`)
/// - The namespace is missing or unrecognized (`MissingNamespace`, `UnknownNamespace`)
/// - The schema version is missing or empty (`MissingSchemaVersion`)
/// - The namespace and schema version don't match (`NamespaceVersionMismatch`)
pub fn inspect_with_config(
    bytes: &[u8],
    config: &ParseConfig,
) -> Result<HpxmlFileInfo, InspectError> {
    // Step 1: Check document size
    if bytes.len() > config.max_bytes {
        return Err(InspectError::DocumentTooLarge {
            size: bytes.len(),
            limit: config.max_bytes,
        });
    }

    // Step 2: Reject DTD declarations before any XML parsing
    if dtd_guard::has_doctype(bytes) {
        return Err(InspectError::DtdNotAllowed);
    }

    // Step 3: Create XML reader (no depth limit needed: we stop after the root element)
    let mut reader = NsReader::from_reader(bytes);
    reader.config_mut().trim_text(true);

    // Buffer for reading events
    let mut buf = Vec::new();

    // Step 4: Read events until we find the root element
    let mut root_namespace: Option<String> = None;
    let mut root_schema_version: Option<String> = None;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                // Step 5: Check if root element is HPXML
                let local_name = e.local_name();
                let local_name_str = String::from_utf8_lossy(local_name.as_ref()).to_string();

                if local_name.as_ref() != b"HPXML" {
                    return Err(InspectError::NotHpxml {
                        found_element: local_name_str,
                    });
                }

                // Step 6: Extract xmlns attribute (default namespace) and schemaVersion
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| InspectError::MalformedXml(e.to_string()))?;
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|e| InspectError::MalformedXml(e.to_string()))?
                        .into_owned();

                    if attr.key.as_ref() == b"xmlns" {
                        root_namespace = Some(value);
                    } else if attr.key.as_ref() == b"schemaVersion" {
                        root_schema_version = Some(value);
                    }
                }

                // We've found the root element, no need to read more
                break;
            }
            Ok(Event::Eof) => {
                return Err(InspectError::MalformedXml("unexpected end of file".into()));
            }
            Err(e) => {
                return Err(InspectError::MalformedXml(e.to_string()));
            }
            _ => {
                // Skip other events (comments, processing instructions, etc.)
            }
        }
    }

    // Step 7: Check namespace is present
    let namespace = root_namespace.ok_or(InspectError::MissingNamespace)?;

    // Step 8: Look up version from namespace
    let version = version_from_namespace(&namespace)
        .ok_or_else(|| InspectError::UnknownNamespace(namespace.clone()))?;

    // Step 9: Extract and validate schemaVersion
    let schema_version = root_schema_version.ok_or(InspectError::MissingSchemaVersion)?;

    // Empty schemaVersion is treated as missing
    if schema_version.is_empty() {
        return Err(InspectError::MissingSchemaVersion);
    }

    // Step 10: Cross-check namespace implies version range
    let schema_version_major = schema_version
        .split('.')
        .next()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    if schema_version_major != version.major() {
        return Err(InspectError::NamespaceVersionMismatch {
            schema_version,
            namespace,
        });
    }

    // Step 11: Return HpxmlFileInfo
    Ok(HpxmlFileInfo {
        version,
        schema_version,
        namespace,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HpxmlVersion;

    #[test]
    fn test_inspect_v2() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2014/6" schemaVersion="2.0"></HPXML>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V2);
        assert_eq!(info.schema_version, "2.0");
    }

    #[test]
    fn test_inspect_v3() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion="3.0"></HPXML>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
        assert_eq!(info.schema_version, "3.0");
    }

    #[test]
    fn test_inspect_v4() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0"></HPXML>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert_eq!(info.schema_version, "4.0");
    }

    #[test]
    fn test_inspect_v5() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2025/12" schemaVersion="5.0"></HPXML>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V5);
        assert_eq!(info.schema_version, "5.0");
    }

    #[test]
    fn test_inspect_not_hpxml() {
        let xml = br#"<?xml version="1.0"?>
<Root xmlns="http://example.com/"></Root>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::NotHpxml { .. })));
    }

    #[test]
    fn test_inspect_unknown_namespace() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://example.com/" schemaVersion="3.0"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::UnknownNamespace(_))));
    }

    #[test]
    fn test_inspect_missing_namespace() {
        let xml = br#"<?xml version="1.0"?>
<HPXML schemaVersion="3.0"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn test_inspect_missing_schema_version() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn test_inspect_empty_schema_version() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion=""></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn test_inspect_namespace_version_mismatch() {
        // OCHRE pre-release bug: namespace says 2019/10 (v3) but schemaVersion says 4.0
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion="4.0"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(
            result,
            Err(InspectError::NamespaceVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_inspect_non_numeric_schema_version_mismatch() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="x.y"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(
            result,
            Err(InspectError::NamespaceVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_inspect_malformed_xml() {
        let xml = br#"<?xml version="1.0"?><HPXML"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MalformedXml(_))));
    }

    #[test]
    fn test_inspect_document_too_large() {
        let xml = vec![0u8; ParseConfig::default().max_bytes + 1];
        let result = inspect(&xml);
        assert!(matches!(result, Err(InspectError::DocumentTooLarge { .. })));
    }

    #[test]
    fn test_inspect_empty_element() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion="3.0"/>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
    }

    // Integration tests using actual test files from tests/data

    #[test]
    fn test_inspect_v2_file() {
        let xml = include_bytes!("../tests/data/v2/audit.xml");
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V2);
        assert!(info.schema_version.starts_with("2."));
    }

    #[test]
    fn test_inspect_v3_file() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
        assert!(info.schema_version.starts_with("3."));
    }

    #[test]
    fn test_inspect_v4_file() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));
    }

    #[test]
    fn test_inspect_v5_file() {
        let xml = include_bytes!("../tests/data/v5/audit.xml");
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V5);
        assert!(info.schema_version.starts_with("5."));
    }

    #[test]
    fn test_inspect_negative_not_hpxml_file() {
        let xml = include_bytes!("../tests/data/negative/not-hpxml.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::NotHpxml { .. })));
    }

    #[test]
    fn test_inspect_negative_unknown_namespace_file() {
        let xml = include_bytes!("../tests/data/negative/unknown-namespace.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::UnknownNamespace(_))));
    }

    #[test]
    fn test_inspect_negative_missing_namespace_file() {
        let xml = include_bytes!("../tests/data/negative/missing-namespace.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn test_inspect_negative_missing_schema_version_file() {
        let xml = include_bytes!("../tests/data/negative/missing-schema-version.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn test_inspect_negative_bad_schema_version_file() {
        let xml = include_bytes!("../tests/data/negative/bad-schema-version.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
    }

    #[test]
    fn test_inspect_negative_malformed_file() {
        let xml = include_bytes!("../tests/data/negative/malformed.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn test_inspect_edge_ochre_mismatch_file() {
        let xml = include_bytes!("../tests/data/edge/ochre-mismatch.xml");
        let result = inspect(xml);
        assert!(matches!(
            result,
            Err(InspectError::NamespaceVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_inspect_rejects_billion_laughs() {
        let xml = include_bytes!("../tests/data/negative/billion-laughs.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }

    #[test]
    fn test_inspect_rejects_xxe() {
        let xml = include_bytes!("../tests/data/negative/xxe.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }

    #[test]
    fn test_inspect_rejects_dtd_internal() {
        let xml = include_bytes!("../tests/data/negative/dtd-internal.xml");
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::DtdNotAllowed)));
    }
}
