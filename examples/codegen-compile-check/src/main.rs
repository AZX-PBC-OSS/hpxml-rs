#![cfg_attr(feature = "deny-warnings", deny(warnings))]

#[allow(dead_code)]
mod generated_v2 {
    include!(concat!(env!("OUT_DIR"), "/generated_v2/root.rs"));
}

#[allow(dead_code)]
mod generated_v3 {
    include!(concat!(env!("OUT_DIR"), "/generated_v3/root.rs"));
}

#[allow(dead_code)]
mod generated_v4 {
    include!(concat!(env!("OUT_DIR"), "/generated_v4/root.rs"));
}

#[allow(dead_code)]
mod generated_v5 {
    include!(concat!(env!("OUT_DIR"), "/generated_v5/root.rs"));
}

fn main() {
    println!("Generated + compiled modules: v2, v3, v4, v5");
}

#[cfg(test)]
mod tests {
    use xsd_parser_types::quick_xml::{DeserializeSync, SerializeSync, SliceReader};

    type HpxmlV4 = super::generated_v4::hpxml::HpxmlElementType;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn generated_types_are_send_sync() {
        assert_send_sync::<HpxmlV4>();
    }

    #[test]
    fn v4_parse_minimal() {
        let bytes = include_bytes!("../../../crates/hpxml-core/tests/data/v4/minimal.xml");
        let mut reader = SliceReader::from_bytes(bytes);
        let doc = HpxmlV4::deserialize(&mut reader).expect("minimal.xml should parse");
        assert!(!doc.building.is_empty(), "should have at least one building");
    }

    #[test]
    fn v4_round_trip_minimal() {
        let bytes = include_bytes!("../../../crates/hpxml-core/tests/data/v4/minimal.xml");
        let mut reader = SliceReader::from_bytes(bytes);
        let doc1 = HpxmlV4::deserialize(&mut reader).expect("first parse should succeed");

        let mut out = Vec::new();
        let mut writer = quick_xml::Writer::new(&mut out);
        doc1.serialize("HPXML", &mut writer)
            .expect("serialization should succeed");

        let mut reader2 = SliceReader::from_bytes(&out);
        let doc2 = HpxmlV4::deserialize(&mut reader2)
            .expect("round-trip parse should succeed");

        assert_eq!(doc1, doc2, "round-trip should produce identical structs");
    }

    #[test]
    fn v4_parse_openst_base() {
        let bytes = include_bytes!("../../../crates/hpxml-core/tests/data/v4/openst-base.xml");
        let mut reader = SliceReader::from_bytes(bytes);
        HpxmlV4::deserialize(&mut reader).expect("openst-base.xml should parse");
    }

    #[test]
    fn v4_bom_prefixed_parse() {
        let raw = include_bytes!("../../../crates/hpxml-core/tests/data/v4/minimal.xml");
        let mut with_bom = vec![0xEF, 0xBB, 0xBF];
        with_bom.extend_from_slice(raw);
        let mut reader = SliceReader::from_bytes(&with_bom);
        HpxmlV4::deserialize(&mut reader).expect("BOM-prefixed XML should parse");
    }

    #[test]
    fn v4_serialization_api() {
        let bytes = include_bytes!("../../../crates/hpxml-core/tests/data/v4/minimal.xml");
        let mut reader = SliceReader::from_bytes(bytes);
        let doc = HpxmlV4::deserialize(&mut reader).unwrap();

        let mut out = Vec::new();
        let mut writer = quick_xml::Writer::new(&mut out);
        doc.serialize("HPXML", &mut writer).unwrap();
        let s = String::from_utf8_lossy(&out);
        assert!(s.contains("<HPXML"), "output should contain HPXML root element");
    }
}
