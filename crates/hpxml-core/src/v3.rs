//! HPXML v3 (schema version 3.x) types and functions.

pub use hpxml_types_v3::*;

crate::impl_version_module!(v3, HpxmlType);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::conformance_tests;
    use crate::traits::HpxmlSerialize;
    use hpxml_common::ParseConfig;

    conformance_tests!("v3", "http://hpxmlonline.com/2019/10", "3.1");

    #[test]
    fn test_parse_invalid_xml() {
        let xml = include_bytes!("../tests/data/v3/invalid.xml");
        let result = parse(xml);
        assert!(result.is_err(), "Expected error for invalid.xml");
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
}
