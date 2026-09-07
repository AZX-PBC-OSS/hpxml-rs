//! HPXML v2 (schema version 2.x) types and functions.

pub use hpxml_types_v2::*;

crate::impl_version_module!(v2, HpxmlType);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::conformance_tests;
    use crate::traits::HpxmlSerialize;
    use hpxml_common::ParseConfig;

    conformance_tests!("v2", "http://hpxmlonline.com/2014/6", "2.3");

    #[test]
    fn test_parse_invalid_xml() {
        let xml = include_bytes!("../tests/data/v2/invalid.xml");
        let result = parse(xml);
        assert!(result.is_err(), "Expected error for invalid.xml");
    }

    #[test]
    fn test_enum_exhaustiveness() {
        use hpxml_types_v2::hpxml_data_types::SchemaVersionType;
        use hpxml_types_v2::hpxml_data_types::TransactionType;

        let _sv: SchemaVersionType = match 0u8 {
            0 => SchemaVersionType::_20,
            1 => SchemaVersionType::_21,
            2 => SchemaVersionType::_22,
            3 => SchemaVersionType::_221,
            4 => SchemaVersionType::_23,
            _ => panic!(),
        };

        let _tx: TransactionType = match 0u8 {
            0 => TransactionType::Create,
            1 => TransactionType::Update,
            _ => panic!(),
        };
    }
}
