//! HPXML v3 (schema version 3.x) types and functions.

pub use hpxml_types_v3::*;

crate::impl_version_module!(v3, HpxmlType);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::HpxmlSerialize;
    use hpxml_common::ParseConfig;

    #[test]
    fn test_parse_audit_xml() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse audit.xml: {:?}", result);
        let hpxml = result.unwrap();
        assert!(!hpxml.building.is_empty());
    }

    #[test]
    fn test_parse_bpi2101_xml() {
        let xml = include_bytes!("../tests/data/v3/bpi2101.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse bpi2101.xml: {:?}", result);
    }

    #[test]
    fn test_parse_upgrade_xml() {
        let xml = include_bytes!("../tests/data/v3/upgrade.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse upgrade.xml: {:?}", result);
    }

    #[test]
    fn test_to_xml_no_declaration() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let hpxml = parse(xml).unwrap();
        let bytes = hpxml.to_xml().unwrap();
        let xml_str = String::from_utf8_lossy(&bytes);
        assert!(!xml_str.starts_with("<?xml"));
        assert!(xml_str.starts_with("<HPXML"));
    }

    #[test]
    fn test_to_xml_into_no_declaration() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
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
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let doc1 = parse(xml).unwrap();
        let bytes = doc1.to_xml().unwrap();
        let doc2 = parse(&bytes).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn test_size_limit() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let config = ParseConfig {
            max_bytes: 10,
            ..Default::default()
        };
        let result = parse_with_config(xml, &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            hpxml_common::ParseError::DocumentTooLarge { .. }
        ));
    }

    #[test]
    fn test_size_limit_exact() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
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
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let config = ParseConfig {
            max_depth: 2,
            ..ParseConfig::default()
        };
        let result = parse_with_config(xml, &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            hpxml_common::ParseError::DepthLimitExceeded { .. }
        ));
    }

    #[test]
    fn test_malformed_xml() {
        let xml = b"<HPXML><unclosed>";
        let result = parse(xml);
        assert!(result.is_err());
        match result.unwrap_err() {
            hpxml_common::ParseError::Xml { position, .. } => assert!(position.is_some()),
            _ => panic!("expected ParseError::Xml"),
        }
    }

    #[test]
    fn test_invalid_utf8() {
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?><HPXML xmlns="http://hpxmlonline.com/2019/10" schemaVersion="3.1"><Building><BuildingID id="a">test\xFF\xFE</BuildingID></Building></HPXML>"#;
        let result = parse(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_type_access() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
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
    fn test_enum_exhaustiveness() {
        use hpxml_types_v3::hpxml_data_types::SchemaVersionType;
        use hpxml_types_v3::hpxml_data_types::TransactionTypeSimple;

        let _sv: SchemaVersionType = match 0u8 {
            0 => SchemaVersionType::_30,
            1 => SchemaVersionType::_31,
            _ => panic!(),
        };

        let _tx: TransactionTypeSimple = match 0u8 {
            0 => TransactionTypeSimple::Create,
            1 => TransactionTypeSimple::Update,
            _ => panic!(),
        };
    }

    #[test]
    fn test_to_xml_into_write_error() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let hpxml = parse(xml).unwrap();

        let mut writer = crate::test_helpers::FailingWriter;
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_common::SerializeError::Xml(_))));
    }

    #[test]
    fn test_to_xml_into_flush_error() {
        let xml = include_bytes!("../tests/data/v3/audit.xml");
        let hpxml = parse(xml).unwrap();

        let mut writer = crate::test_helpers::FailingFlusher { buf: Vec::new() };
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_common::SerializeError::Io(_))));
    }
}
