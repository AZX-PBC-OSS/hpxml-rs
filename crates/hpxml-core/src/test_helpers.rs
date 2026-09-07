/// Fails every `write`; surfaces as `SerializeError::Xml`.
/// Unused when no version feature is enabled.
#[allow(dead_code)]
pub(crate) struct FailingWriter;

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

#[allow(dead_code)]
pub(crate) struct FailingFlusher {
    #[allow(dead_code)]
    pub(crate) buf: Vec<u8>,
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

/// Version-agnostic conformance tests shared by every HPXML version module.
///
/// Each `vN.rs` test module invokes this once. The generated tests exercise
/// only behavior identical across versions, so conformance cannot drift.
///
/// Invocation-site requirements (`use super::*` provides the first three):
/// `parse`, `parse_with_config`, `HpxmlType`, plus
/// `crate::traits::HpxmlSerialize` and `hpxml_common::ParseConfig`.
///
/// ```text
/// use crate::test_helpers::conformance_tests;
///
/// conformance_tests!("v4", "http://hpxmlonline.com/2023/09", "4.2");
/// ```
///
/// Coverage that genuinely differs per version (extra fixtures,
/// `invalid.xml`, declaration options, enum variants) stays in the version
/// module next to the invocation.
///
/// Unused when no version feature is enabled.
#[allow(unused_macros)]
macro_rules! conformance_tests {
    ($dir:literal, $namespace:literal, $schema_version:literal) => {
        #[test]
        fn test_parse_audit_xml() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let result = parse(xml);
            assert!(result.is_ok(), "Failed to parse audit.xml: {:?}", result);
            assert!(!result.unwrap().building.is_empty());
        }

        #[test]
        fn test_to_xml_no_declaration() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let hpxml = parse(xml).unwrap();
            let bytes = hpxml.to_xml().unwrap();
            let xml_str = String::from_utf8_lossy(&bytes);
            assert!(!xml_str.starts_with("<?xml"));
            assert!(xml_str.starts_with("<HPXML"));
        }

        #[test]
        fn test_to_xml_into_no_declaration() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let hpxml = parse(xml).unwrap();
            let mut out = std::io::Cursor::new(Vec::<u8>::new());
            hpxml.to_xml_into(&mut out).unwrap();
            let bytes = out.into_inner();
            let xml_str = String::from_utf8_lossy(&bytes);
            assert!(!xml_str.starts_with("<?xml"));
            assert!(xml_str.starts_with("<HPXML"));
        }

        #[test]
        fn test_roundtrip() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let doc1 = parse(xml).unwrap();
            let bytes = doc1.to_xml().unwrap();
            let doc2 = parse(&bytes).unwrap();
            assert_eq!(doc1, doc2);
        }

        #[test]
        fn test_size_limit() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let config = ParseConfig {
                max_bytes: 10,
                ..Default::default()
            };
            let result = parse_with_config(xml, &config);
            assert!(
                matches!(
                    result,
                    Err(hpxml_common::ParseError::DocumentTooLarge { .. })
                ),
                "undersized limit must be rejected, got: {result:?}"
            );
        }

        #[test]
        fn test_size_limit_exact() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let size = xml.len();
            let config = ParseConfig {
                max_bytes: size,
                ..Default::default()
            };
            assert!(
                parse_with_config(xml, &config).is_ok(),
                "Should succeed at exact limit"
            );
            let config2 = ParseConfig {
                max_bytes: size / 2,
                ..Default::default()
            };
            assert!(parse_with_config(xml, &config2).is_err());
        }

        #[test]
        fn test_depth_limit() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let config = ParseConfig {
                max_depth: 2,
                ..ParseConfig::default()
            };
            let result = parse_with_config(xml, &config);
            assert!(
                matches!(
                    result,
                    Err(hpxml_common::ParseError::DepthLimitExceeded { .. })
                ),
                "shallow limit must be rejected, got: {result:?}"
            );
        }

        #[test]
        fn test_malformed_xml() {
            let xml = b"<HPXML><unclosed>";
            let result = parse(xml);
            match result.unwrap_err() {
                hpxml_common::ParseError::Xml { position, .. } => assert!(position.is_some()),
                err => panic!("expected ParseError::Xml, got: {err:?}"),
            }
        }

        #[test]
        fn test_invalid_utf8() {
            let mut xml = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><HPXML xmlns=\"{ns}\" schemaVersion=\"{sv}\"><Building><BuildingID id=\"a\">",
                ns = $namespace,
                sv = $schema_version
            )
            .into_bytes();
            xml.extend_from_slice(b"test\xFF\xFE</BuildingID></Building></HPXML>");
            assert!(parse(&xml).is_err());
        }

        #[test]
        fn test_type_access() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let hpxml: HpxmlType = parse(xml).unwrap();
            assert!(!hpxml.building.is_empty());
            let _bytes = hpxml.to_xml().unwrap();
        }

        #[test]
        fn test_send_sync() {
            fn _assert_send_sync<T: Send + Sync>() {}
            _assert_send_sync::<HpxmlType>();
        }

        #[test]
        fn test_to_xml_into_write_error() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let hpxml = parse(xml).unwrap();

            let mut writer = $crate::test_helpers::FailingWriter;
            let result = hpxml.to_xml_into(&mut writer);
            assert!(matches!(
                result,
                Err(hpxml_common::SerializeError::Xml(_))
            ));
        }

        #[test]
        fn test_to_xml_into_flush_error() {
            let xml = include_bytes!(concat!("../tests/data/", $dir, "/audit.xml"));
            let hpxml = parse(xml).unwrap();

            let mut writer = $crate::test_helpers::FailingFlusher { buf: Vec::new() };
            let result = hpxml.to_xml_into(&mut writer);
            assert!(matches!(
                result,
                Err(hpxml_common::SerializeError::Io(_))
            ));
        }

        #[test]
        fn test_parse_rejects_dtd() {
            let xml = include_bytes!("../tests/data/negative/billion-laughs.xml");
            let result = parse(xml);
            assert!(
                matches!(result, Err(hpxml_common::ParseError::DtdNotAllowed)),
                "DTD must be rejected, got: {result:?}"
            );
        }
    };
}

#[allow(unused_imports)]
pub(crate) use conformance_tests;
