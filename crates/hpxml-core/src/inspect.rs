//! HPXML document inspection (pre-parsing validation).
//!
//! This module provides the `inspect()` function which reads only the root XML
//! element to determine the HPXML version without fully parsing the document.

use quick_xml::NsReader;
use quick_xml::events::Event;
use quick_xml::name::ResolveResult;

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
    // Size gate.
    if bytes.len() > config.max_bytes {
        return Err(InspectError::DocumentTooLarge {
            size: bytes.len(),
            limit: config.max_bytes,
        });
    }

    // DTD gate: reject before any XML parsing.
    if dtd_guard::has_doctype(bytes) {
        return Err(InspectError::DtdNotAllowed);
    }

    // Lightweight reader: we stop after the root element, so no depth limit.
    let mut reader = NsReader::from_reader(bytes);
    reader.config_mut().trim_text(true);

    // Buffer for reading events
    let mut buf = Vec::new();

    // Read events until the root element.
    let mut root_namespace: Option<String> = None;
    let mut root_schema_version: Option<String> = None;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                // Resolve the element name against in-scope namespace
                // declarations (NsReader already pushed this element's own
                // xmlns attributes). This accepts the default-namespace form
                // `<HPXML xmlns="...">` and prefixed forms like
                // `<h:HPXML xmlns:h="...">` identically.
                let (ns_result, local) = reader.resolver().resolve_element(e.name());
                let local_name_str = String::from_utf8_lossy(local.into_inner()).into_owned();

                // Root element must be HPXML.
                if local_name_str != "HPXML" {
                    return Err(InspectError::NotHpxml {
                        found_element: local_name_str,
                    });
                }

                // Record the resolved namespace. Unbound (no declaration in
                // scope) and Unknown (undeclared prefix) both surface
                // downstream as MissingNamespace: the document declares no
                // usable HPXML namespace.
                if let ResolveResult::Bound(ns) = ns_result {
                    root_namespace = Some(String::from_utf8_lossy(ns.into_inner()).into_owned());
                }

                // Extract schemaVersion (unaffected by the element's prefix).
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| InspectError::MalformedXml(e.to_string()))?;
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|e| InspectError::MalformedXml(e.to_string()))?
                        .into_owned();

                    if attr.key.as_ref() == b"schemaVersion" {
                        root_schema_version = Some(value);
                    }
                }

                // Root element found; nothing further to read.
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

    // Namespace must be present and recognized.
    let namespace = root_namespace.ok_or(InspectError::MissingNamespace)?;

    // Look up version from namespace.
    let version = version_from_namespace(&namespace)
        .ok_or_else(|| InspectError::UnknownNamespace(namespace.clone()))?;

    // schemaVersion must be present.
    let schema_version = root_schema_version.ok_or(InspectError::MissingSchemaVersion)?;

    // Whitespace-only counts as missing; surrounding whitespace or a BOM
    // around a real value is tolerated.
    if schema_version.trim().trim_matches('\u{FEFF}').is_empty() {
        return Err(InspectError::MissingSchemaVersion);
    }

    // Require a numeric `major.minor` shape. Anything else is malformed
    // input, not a namespace/version mismatch.
    let major: u32 = {
        let value = schema_version.trim().trim_matches('\u{FEFF}');
        let mut parts = value.split('.');
        let shaped = match (parts.next(), parts.next()) {
            (Some(major), Some(minor))
                if !major.is_empty()
                    && major.bytes().all(|b| b.is_ascii_digit())
                    && minor.bytes().next().is_some_and(|b| b.is_ascii_digit()) =>
            {
                major.parse::<u32>().ok()
            }
            _ => None,
        };
        match shaped {
            Some(major) => major,
            None => {
                return Err(InspectError::MalformedXml(format!(
                    "bad schemaVersion {schema_version:?}"
                )));
            }
        }
    };

    // Cross-check: namespace major version must match schemaVersion major.
    if major != version.major() {
        return Err(InspectError::NamespaceVersionMismatch {
            schema_version,
            namespace,
        });
    }

    // Return HpxmlFileInfo
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
    fn test_inspect_prefixed_namespace() {
        let xml = br#"<?xml version="1.0"?>
<h:HPXML xmlns:h="http://hpxmlonline.com/2023/09" schemaVersion="4.0"></h:HPXML>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V4);
        assert_eq!(info.schema_version, "4.0");
        assert_eq!(info.namespace, "http://hpxmlonline.com/2023/09");
    }

    #[test]
    fn test_inspect_prefixed_empty_root() {
        let xml = br#"<?xml version="1.0"?>
<h:HPXML xmlns:h="http://hpxmlonline.com/2019/10" schemaVersion="3.1"/>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
    }

    #[test]
    fn test_inspect_undeclared_prefix_is_missing_namespace() {
        let xml = br#"<?xml version="1.0"?>
<h:HPXML schemaVersion="4.0"></h:HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingNamespace)));
    }

    #[test]
    fn test_inspect_prefixed_not_hpxml_reports_local_name() {
        let xml = br#"<?xml version="1.0"?>
<h:Root xmlns:h="http://example.com/"></h:Root>"#;
        let result = inspect(xml);
        assert!(matches!(
            result,
            Err(InspectError::NotHpxml {
                found_element,
            }) if found_element == "Root"
        ));
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
    fn test_inspect_non_numeric_schema_version_is_malformed() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="x.y"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MalformedXml(_))));
    }

    #[test]
    fn test_inspect_bare_major_schema_version_is_malformed() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4"></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MalformedXml(_))));
    }

    #[test]
    fn test_inspect_whitespace_schema_version_is_missing() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="   "></HPXML>"#;
        let result = inspect(xml);
        assert!(matches!(result, Err(InspectError::MissingSchemaVersion)));
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
    fn test_inspect_with_config_size_limit() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let config = ParseConfig {
            max_bytes: 10,
            ..ParseConfig::default()
        };
        let result = inspect_with_config(xml, &config);
        assert!(
            matches!(result, Err(InspectError::DocumentTooLarge { .. })),
            "undersized limit must be rejected, got: {result:?}"
        );
        let ok_config = ParseConfig {
            max_bytes: xml.len(),
            ..ParseConfig::default()
        };
        assert!(inspect_with_config(xml, &ok_config).is_ok());
    }

    #[test]
    fn test_inspect_empty_element() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion="3.0"/>"#;
        let info = inspect(xml).unwrap();
        assert_eq!(info.version, HpxmlVersion::V3);
    }
}
