//! HPXML v4 (schema version 4.x) types and functions.

pub use hpxml_types_v4::*;

crate::impl_version_module!(v4, HpxmlType);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::HpxmlSerialize;
    use hpxml_common::ParseConfig;

    /// Test: parse minimal.xml successfully
    #[test]
    fn test_parse_minimal_xml() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse minimal.xml: {:?}", result);
        let hpxml = result.unwrap();
        assert!(!hpxml.building.is_empty());
    }

    /// Test: parse audit.xml successfully
    #[test]
    fn test_parse_audit_xml() {
        let xml = include_bytes!("../tests/data/v4/audit.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse audit.xml: {:?}", result);
    }

    /// Test: size limit returns DocumentTooLarge
    #[test]
    fn test_size_limit() {
        let xml = b"<HPXML></HPXML>";
        let config = ParseConfig {
            max_bytes: 10,
            ..Default::default()
        };
        let result = parse_with_config(xml, &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            hpxml_common::ParseError::DocumentTooLarge { .. }
        ));
    }

    /// Test: to_xml() produces XML bytes without declaration
    #[test]
    fn test_to_xml_no_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        let bytes = hpxml.to_xml().unwrap();
        let xml_str = String::from_utf8_lossy(&bytes);
        // Should not start with <?xml
        assert!(!xml_str.starts_with("<?xml"));
        // Should start with <HPXML
        assert!(xml_str.starts_with("<HPXML"));
    }

    /// Test: to_xml_into() writes XML bytes without declaration
    #[test]
    fn test_to_xml_into_no_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        let mut out = std::io::Cursor::new(Vec::<u8>::new());

        hpxml.to_xml_into(&mut out).unwrap();
        let bytes = out.into_inner();
        let xml_str = String::from_utf8_lossy(&bytes);

        assert!(!xml_str.starts_with("<?xml"));
        assert!(xml_str.starts_with("<HPXML"));
    }

    /// Test: round-trip parse -> to_xml -> parse
    #[test]
    fn test_roundtrip() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let doc1 = parse(xml).unwrap();
        let bytes = doc1.to_xml().unwrap();
        let doc2 = parse(&bytes).unwrap();

        assert_eq!(doc1, doc2);
    }

    /// Test: depth limit is enforced during parse
    #[test]
    fn test_depth_limit() {
        // Test with minimal.xml and a very low depth limit
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let config = ParseConfig {
            max_depth: 2,
            ..ParseConfig::default()
        };
        let result = parse_with_config(xml, &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            hpxml_common::ParseError::DepthLimitExceeded { .. }
        ));
    }

    /// Test: malformed XML returns ParseError::Xml
    #[test]
    fn test_malformed_xml() {
        let xml = b"<HPXML><unclosed>";
        let result = parse(xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            hpxml_common::ParseError::Xml { position, .. } => assert!(position.is_some()),
            _ => panic!("expected ParseError::Xml"),
        }
    }

    /// Test: parse_with_config respects size limit
    #[test]
    fn test_size_limit_exact() {
        // Use a valid minimal HPXML and test size limits
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let size = xml.len();

        // Test with limit equal to file size - should succeed
        let config = ParseConfig {
            max_bytes: size,
            ..Default::default()
        };
        let result = parse_with_config(xml, &config);
        assert!(
            result.is_ok(),
            "Should succeed at exact limit: {:?}",
            result
        );

        // Test with limit less than file size - should fail
        let config2 = ParseConfig {
            max_bytes: size / 2,
            ..Default::default()
        };
        let result2 = parse_with_config(xml, &config2);
        assert!(result2.is_err());
    }

    /// Test: verify types are accessible and can be used
    #[test]
    fn test_type_access() {
        // Verify parse returns the right type
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml: HpxmlType = parse(xml).unwrap();

        // Verify can access building field
        assert!(!hpxml.building.is_empty());

        // Verify can serialize back to XML
        let _bytes = hpxml.to_xml().unwrap();
    }

    /// Test: root-child nillable access works for parsed data
    #[test]
    fn test_nillable_root_child_access() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        assert!(!hpxml.building.is_empty());

        let first = &hpxml.building[0];
        assert!(!first.is_nil());
        let building = first.as_ref().expect("expected non-nil building");
        assert!(building.building_details.building_summary.is_some());
    }

    /// Test: verify Nillable type can be constructed
    #[test]
    fn test_nillable_construction() {
        use crate::Nillable;

        // Create Nillable with Value
        let _nillable_value: Nillable<String> = Nillable::new("test".to_string());

        // Create Nillable with Nil
        let _nillable_nil: Nillable<String> = Nillable::nil();
    }

    /// Test: programmatic construction with nested building and nillable values round-trips
    #[test]
    fn test_programmatic_roundtrip_with_nillable() {
        use crate::Nillable;

        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let mut doc = parse(xml).unwrap();

        let building_value = doc.building.remove(0).unwrap();
        let doc_programmatic = HpxmlType {
            schema_version: doc.schema_version,
            data_source: doc.data_source,
            xml_transaction_header_information: doc.xml_transaction_header_information,
            software_info: doc.software_info,
            contractor: doc.contractor,
            customer: doc.customer,
            building: vec![Nillable::new(building_value), Nillable::nil()],
            project: doc.project,
            utility: doc.utility,
            consumption: doc.consumption,
        };

        let bytes = doc_programmatic.to_xml().unwrap();
        let reparsed = parse(&bytes).unwrap();
        assert_eq!(doc_programmatic, reparsed);
        assert_eq!(reparsed.building.len(), 2);
        assert!(!reparsed.building[0].is_nil());
        assert!(reparsed.building[1].is_nil());
    }

    /// Test: construct nested Building via struct literal and roundtrip
    #[test]
    fn test_building_type_struct_literal_roundtrip() {
        use crate::Nillable;

        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let mut doc = parse(xml).unwrap();
        let building = doc.building.remove(0).unwrap();

        let hpxml_types_v4::hpxml_base_elements::Building {
            data_source,
            building_id,
            external_resource,
            customer_id,
            site,
            contractor_id,
            project_status,
            building_details,
            modeled_usages,
            extension,
        } = building;

        let rebuilt = hpxml_types_v4::hpxml_base_elements::Building {
            data_source,
            building_id,
            external_resource,
            customer_id,
            site,
            contractor_id,
            project_status,
            building_details,
            modeled_usages,
            extension,
        };

        doc.building = vec![Nillable::new(rebuilt)];
        let bytes = doc.to_xml().unwrap();
        let reparsed = parse(&bytes).unwrap();
        assert_eq!(reparsed.building.len(), 1);
        assert!(!reparsed.building[0].is_nil());
    }

    /// Test: Send + Sync are implemented (required for thread safety)
    #[test]
    fn test_send_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<HpxmlType>();
    }

    /// Test: enum exhaustiveness check
    #[test]
    fn test_enum_exhaustiveness() {
        use hpxml_types_v4::hpxml_data_types::SchemaVersionType;
        use hpxml_types_v4::hpxml_data_types::TransactionTypeSimple;

        // SchemaVersionType has variants
        let _sv: SchemaVersionType = match 0u8 {
            0 => SchemaVersionType::_40,
            1 => SchemaVersionType::_41,
            2 => SchemaVersionType::_42,
            _ => panic!(),
        };

        // TransactionTypeSimple has variants
        let _tx: TransactionTypeSimple = match 0u8 {
            0 => TransactionTypeSimple::Create,
            1 => TransactionTypeSimple::Update,
            _ => panic!(),
        };
    }

    /// Test: parse maximal.xml (large file with all elements)
    #[test]
    fn test_parse_maximal_xml() {
        let xml = include_bytes!("../tests/data/v4/maximal.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse maximal.xml: {:?}", result);
        let hpxml = result.unwrap();
        assert!(!hpxml.building.is_empty());
    }

    /// Test: parse upgrade.xml
    #[test]
    fn test_parse_upgrade_xml() {
        let xml = include_bytes!("../tests/data/v4/upgrade.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse upgrade.xml: {:?}", result);
    }

    /// Test: parse bpi2101.xml
    #[test]
    fn test_parse_bpi2101_xml() {
        let xml = include_bytes!("../tests/data/v4/bpi2101.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse bpi2101.xml: {:?}", result);
    }

    /// Test: invalid.xml produces an error (schema validation failure)
    #[test]
    fn test_parse_invalid_xml() {
        let xml = include_bytes!("../tests/data/v4/invalid.xml");
        let result = parse(xml);
        // Should fail due to schema validation issues
        assert!(result.is_err(), "Expected error for invalid.xml");
    }

    /// Test: invalid UTF-8 bytes returns ParseError (either Utf8 or Xml)
    #[test]
    fn test_invalid_utf8() {
        // XML with invalid UTF-8 sequence in a text node
        // Using bytes that are invalid UTF-8 but valid in some ISO-8859-1 context
        let xml = br#"<?xml version="1.0" encoding="UTF-8"?><HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0"><Building><BuildingID id="a">test\xFF\xFE</BuildingID></Building></HPXML>"#;
        let result = parse(xml);
        // Either UTF-8 error or XML parsing error is acceptable
        assert!(result.is_err());
    }

    /// Test: to_xml_into with failing writer returns SerializeError::Xml
    /// (write failures during serialization are wrapped by quick_xml as Xml errors)
    #[test]
    fn test_to_xml_into_write_error() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();

        let mut writer = crate::test_helpers::FailingWriter;
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_common::SerializeError::Xml(_))));
    }

    /// Test: to_xml_into with failing flush returns SerializeError::Io
    #[test]
    fn test_to_xml_into_flush_error() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();

        let mut writer = crate::test_helpers::FailingFlusher { buf: Vec::new() };
        let result = hpxml.to_xml_into(&mut writer);
        assert!(matches!(result, Err(hpxml_common::SerializeError::Io(_))));
    }

    #[test]
    fn test_to_xml_with_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        let opts = hpxml_common::SerializeOptions {
            xml_declaration: true,
        };
        let bytes = hpxml.to_xml_with_options(&opts).unwrap();
        let xml_str = String::from_utf8(bytes).unwrap();
        assert!(
            xml_str.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"),
            "Expected XML declaration, got: {}",
            &xml_str[..80.min(xml_str.len())]
        );
        assert!(xml_str.contains("<HPXML"));
    }

    #[test]
    fn test_to_xml_into_with_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        let opts = hpxml_common::SerializeOptions {
            xml_declaration: true,
        };
        let mut out = std::io::Cursor::new(Vec::<u8>::new());
        hpxml.to_xml_into_with_options(&mut out, &opts).unwrap();
        let xml_str = String::from_utf8(out.into_inner()).unwrap();
        assert!(xml_str.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml_str.contains("<HPXML"));
    }

    #[test]
    fn test_roundtrip_with_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let doc1 = parse(xml).unwrap();
        let opts = hpxml_common::SerializeOptions {
            xml_declaration: true,
        };
        let bytes = doc1.to_xml_with_options(&opts).unwrap();
        let doc2 = parse(&bytes).unwrap();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn test_default_options_no_declaration() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        let opts = hpxml_common::SerializeOptions::default();
        let bytes = hpxml.to_xml_with_options(&opts).unwrap();
        let xml_str = String::from_utf8(bytes).unwrap();
        assert!(!xml_str.starts_with("<?xml"));
        assert!(xml_str.starts_with("<HPXML"));
    }

    #[test]
    fn test_parse_rejects_billion_laughs() {
        let xml = include_bytes!("../tests/data/negative/billion-laughs.xml");
        let result = parse(xml);
        assert!(matches!(
            result,
            Err(hpxml_common::ParseError::DtdNotAllowed)
        ));
    }

    #[test]
    fn test_parse_rejects_xxe() {
        let xml = include_bytes!("../tests/data/negative/xxe.xml");
        let result = parse(xml);
        assert!(matches!(
            result,
            Err(hpxml_common::ParseError::DtdNotAllowed)
        ));
    }

    #[test]
    fn test_parse_rejects_dtd_internal() {
        let xml = include_bytes!("../tests/data/negative/dtd-internal.xml");
        let result = parse(xml);
        assert!(matches!(
            result,
            Err(hpxml_common::ParseError::DtdNotAllowed)
        ));
    }
}
