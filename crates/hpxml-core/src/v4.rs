//! HPXML v4 (schema version 4.x) types and functions.

pub use hpxml_types_v4::*;

crate::impl_version_module!(v4, HpxmlType);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::conformance_tests;
    use crate::traits::HpxmlSerialize;
    use hpxml_common::ParseConfig;

    conformance_tests!("v4", "http://hpxmlonline.com/2023/09", "4.2");

    #[test]
    fn test_parse_minimal_xml() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse minimal.xml: {:?}", result);
        let hpxml = result.unwrap();
        assert!(!hpxml.building.is_empty());
    }

    #[test]
    fn test_nillable_root_child_access() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let hpxml = parse(xml).unwrap();
        assert!(!hpxml.building.is_empty());

        let first = hpxml.building.first().expect("minimal.xml has a building");
        assert!(!first.is_nil());
        let building = first.as_ref().expect("expected non-nil building");
        assert!(building.building_details.building_summary.is_some());
    }

    #[test]
    fn test_nillable_construction() {
        use crate::Nillable;

        let _nillable_value: Nillable<String> = Nillable::new("test".to_string());
        let _nillable_nil: Nillable<String> = Nillable::nil();
    }

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
        assert!(
            !reparsed
                .building
                .first()
                .expect("reparsed doc has buildings")
                .is_nil()
        );
        assert!(
            reparsed
                .building
                .get(1)
                .expect("reparsed doc has two buildings")
                .is_nil()
        );
    }

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
        assert!(
            !reparsed
                .building
                .first()
                .expect("reparsed doc has a building")
                .is_nil()
        );
    }

    #[test]
    fn test_enum_exhaustiveness() {
        use hpxml_types_v4::hpxml_data_types::SchemaVersionType;
        use hpxml_types_v4::hpxml_data_types::TransactionTypeSimple;

        let _sv: SchemaVersionType = match 0u8 {
            0 => SchemaVersionType::_40,
            1 => SchemaVersionType::_41,
            2 => SchemaVersionType::_42,
            _ => panic!(),
        };

        let _tx: TransactionTypeSimple = match 0u8 {
            0 => TransactionTypeSimple::Create,
            1 => TransactionTypeSimple::Update,
            _ => panic!(),
        };
    }

    #[test]
    fn test_parse_maximal_xml() {
        let xml = include_bytes!("../tests/data/v4/maximal.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse maximal.xml: {:?}", result);
        let hpxml = result.unwrap();
        assert!(!hpxml.building.is_empty());
    }

    #[test]
    fn test_parse_upgrade_xml() {
        let xml = include_bytes!("../tests/data/v4/upgrade.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse upgrade.xml: {:?}", result);
    }

    #[test]
    fn test_parse_bpi2101_xml() {
        let xml = include_bytes!("../tests/data/v4/bpi2101.xml");
        let result = parse(xml);
        assert!(result.is_ok(), "Failed to parse bpi2101.xml: {:?}", result);
    }

    #[test]
    fn test_parse_invalid_xml() {
        let xml = include_bytes!("../tests/data/v4/invalid.xml");
        let result = parse(xml);
        assert!(result.is_err(), "Expected error for invalid.xml");
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
