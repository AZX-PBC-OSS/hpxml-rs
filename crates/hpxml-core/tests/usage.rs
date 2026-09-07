//! Integration tests: realistic API usage patterns.
//!
//! Covers document construction, mutation, serialization, and the
//! two-phase inspect-then-parse workflow.
//!
//! Every test module below is gated on its version feature, so the whole
//! target is empty (and still compiles) with no version features enabled.

#![cfg(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5"))]

#[macro_use]
mod common;

/// Generates common usage tests for any HPXML version.
///
/// These tests work entirely from parsed fixtures, so they don't need
/// version-specific construction helpers. The struct field paths
/// (building_id.id, site, modeled_usages) are identical across all versions.
macro_rules! version_tests {
    ($mod_name:ident, feature = $feature:literal, version = $version:ident, fixture = $fixture:literal) => {
        #[cfg(feature = $feature)]
        mod $mod_name {
            use hpxml_core::HpxmlSerialize;
            use hpxml_core::Nillable;

            fn roundtrip(doc: &hpxml_core::$version::HpxmlType) -> hpxml_core::$version::HpxmlType {
                let bytes = doc.to_xml().expect("serialization failed");
                hpxml_core::$version::parse(&bytes).expect("re-parse failed")
            }

            #[test]
            fn parse_and_access_building_fields() {
                let bytes = fixture!($fixture);
                let doc = hpxml_core::$version::parse(bytes).unwrap();

                assert!(!doc.building.is_empty());
                let b = doc
                    .building
                    .first()
                    .expect("fixture has a building")
                    .as_ref()
                    .unwrap();
                assert!(!b.building_id.id.is_empty());
            }

            #[test]
            fn modify_and_roundtrip() {
                let bytes = fixture!($fixture);
                let mut doc = hpxml_core::$version::parse(bytes).unwrap();

                let b = doc
                    .building
                    .first_mut()
                    .expect("fixture has a building")
                    .as_mut()
                    .unwrap();
                let original_id = b.building_id.id.clone();
                b.building_id.id = format!("{}-modified", original_id);

                let reparsed = roundtrip(&doc);
                assert_eq!(
                    reparsed
                        .building
                        .first()
                        .expect("reparsed doc has a building")
                        .as_ref()
                        .unwrap()
                        .building_id
                        .id,
                    format!("{}-modified", original_id)
                );
            }

            #[test]
            fn nillable_nil_first_building() {
                let bytes = fixture!($fixture);
                let mut doc = hpxml_core::$version::parse(bytes).unwrap();
                let original_len = doc.building.len();
                assert!(!doc.building.is_empty(), "fixture must contain a building");

                doc.building[0] = Nillable::nil();

                let reparsed = roundtrip(&doc);
                assert_eq!(reparsed.building.len(), original_len);
                assert!(
                    reparsed
                        .building
                        .first()
                        .expect("reparsed doc has a building")
                        .is_nil()
                );
            }

            #[test]
            fn nillable_push_nil() {
                let bytes = fixture!($fixture);
                let mut doc = hpxml_core::$version::parse(bytes).unwrap();
                let original_len = doc.building.len();

                doc.building.push(Nillable::nil());

                let reparsed = roundtrip(&doc);
                assert_eq!(reparsed.building.len(), original_len + 1);
                assert!(reparsed.building.last().unwrap().is_nil());
            }

            #[test]
            fn clone_and_compare() {
                let bytes = fixture!($fixture);
                let doc = hpxml_core::$version::parse(bytes).unwrap();

                let cloned = doc.clone();
                assert_eq!(doc, cloned);
            }

            #[test]
            fn strip_optional_fields_roundtrip() {
                let bytes = fixture!($fixture);
                let mut doc = hpxml_core::$version::parse(bytes).unwrap();

                let b = doc
                    .building
                    .first_mut()
                    .expect("fixture has a building")
                    .as_mut()
                    .unwrap();
                b.site = None;
                b.modeled_usages = None;

                let reparsed = roundtrip(&doc);
                let b = reparsed
                    .building
                    .first()
                    .expect("reparsed doc has a building")
                    .as_ref()
                    .unwrap();
                assert!(b.site.is_none());
                assert!(b.modeled_usages.is_none());
            }

            #[test]
            fn filter_non_nil_buildings() {
                let bytes = fixture!($fixture);
                let mut doc = hpxml_core::$version::parse(bytes).unwrap();

                doc.building.push(Nillable::nil());

                let ids: Vec<&str> = doc
                    .building
                    .iter()
                    .filter_map(|b| b.as_ref())
                    .map(|b| b.building_id.id.as_str())
                    .collect();
                assert!(!ids.is_empty());
                assert!(ids.iter().all(|id| !id.is_empty()));
            }

            #[test]
            fn send_sync() {
                fn _assert<T: Send + Sync>() {}
                _assert::<hpxml_core::$version::HpxmlType>();
            }
        }
    };
}

version_tests!(
    v2_common,
    feature = "v2",
    version = v2,
    fixture = "v2/audit.xml"
);
version_tests!(
    v3_common,
    feature = "v3",
    version = v3,
    fixture = "v3/audit.xml"
);
version_tests!(
    v4_common,
    feature = "v4",
    version = v4,
    fixture = "v4/audit.xml"
);
version_tests!(
    v5_common,
    feature = "v5",
    version = v5,
    fixture = "v5/audit.xml"
);

// ---------------------------------------------------------------------------
// V4-specific tests: detailed API patterns with constructed documents.
// V4 is the primary version and gets deeper coverage.
// ---------------------------------------------------------------------------

#[cfg(feature = "v4")]
mod v4_usage {
    use hpxml_core::HpxmlSerialize;
    use hpxml_core::Nillable;
    use hpxml_core::v4::HpxmlType;
    use hpxml_core::v4::hpxml_base_elements::{
        Building, BuildingDetailsType, Customer, CustomerCustomerDetails, IndividualInfo,
        IndividualInfoName, IndividualInfoSystemInfo, ProjectStatus, ProjectStatusElementType,
        SoftwareInfo, SoftwareInfoElementType, SystemIdentifiersInfoType,
        XmlTransactionHeaderInformation, XmlTransactionHeaderInformationElementType,
    };
    use hpxml_core::v4::hpxml_data_types::{
        CreatedDateAndTime, EventType, EventTypeSimple, FuelTypeSimple, HpxmlString,
        SchemaVersionType, TransactionType, TransactionTypeSimple, XmlGeneratedBy, XmlType,
    };

    fn str_type(s: &str) -> HpxmlString {
        HpxmlString {
            data_source: None,
            content: s.to_string(),
        }
    }

    fn building_id(id: &str) -> SystemIdentifiersInfoType {
        SystemIdentifiersInfoType {
            id: id.to_string(),
            sameas: None,
            data_source: None,
            sending_system_identifier_type: None,
            sending_system_identifier_value: None,
            receiving_system_identifier_type: None,
            receiving_system_identifier_value: None,
        }
    }

    fn audit_project_status() -> ProjectStatus {
        ProjectStatusElementType {
            data_source: None,
            event_type: EventType {
                data_source: None,
                content: EventTypeSimple::Audit,
            },
            date: None,
            extension: None,
        }
    }

    fn empty_building_details() -> BuildingDetailsType {
        BuildingDetailsType {
            data_source: None,
            building_summary: None,
            climateand_risk_zones: None,
            green_building_verifications: None,
            zones: None,
            enclosure: None,
            systems: None,
            appliances: None,
            lighting: None,
            pools: None,
            spas: None,
            misc_loads: None,
            health_and_safety: None,
            extension: None,
        }
    }

    fn transaction_header(xml_type: &str, generated_by: &str) -> XmlTransactionHeaderInformation {
        XmlTransactionHeaderInformationElementType {
            data_source: None,
            xml_type: XmlType {
                data_source: None,
                content: xml_type.to_string(),
            },
            xml_generated_by: XmlGeneratedBy {
                data_source: None,
                content: generated_by.to_string(),
            },
            created_date_and_time: CreatedDateAndTime {
                data_source: None,
                content: "2024-01-01T00:00:00Z".to_string(),
            },
            transaction: TransactionType {
                data_source: None,
                content: TransactionTypeSimple::Create,
            },
            extension: None,
        }
    }

    fn empty_software_info() -> SoftwareInfo {
        SoftwareInfoElementType {
            data_source: None,
            software_program_used: None,
            software_program_version: None,
            extension: None,
        }
    }

    fn minimal_building(id: &str) -> Building {
        Building {
            data_source: None,
            building_id: building_id(id),
            external_resource: vec![],
            customer_id: None,
            site: None,
            contractor_id: None,
            project_status: audit_project_status(),
            building_details: empty_building_details(),
            modeled_usages: None,
            extension: None,
        }
    }

    fn roundtrip(doc: &HpxmlType) -> HpxmlType {
        let bytes = doc.to_xml().expect("serialization failed");
        hpxml_core::v4::parse(&bytes).expect("re-parse failed")
    }

    #[test]
    fn construct_minimal_document_from_scratch() {
        let doc = HpxmlType {
            schema_version: SchemaVersionType::_42,
            data_source: None,
            xml_transaction_header_information: transaction_header("HPXML", "hpxml-rs test"),
            software_info: empty_software_info(),
            contractor: vec![],
            customer: vec![],
            building: vec![Nillable::new(minimal_building("Building1"))],
            project: vec![],
            utility: vec![],
            consumption: vec![],
        };

        let bytes = doc.to_xml().unwrap();
        let xml_str = String::from_utf8_lossy(&bytes);
        assert!(xml_str.contains("<HPXML"), "missing root element");
        assert!(xml_str.contains("Building1"), "missing building id");

        let reparsed = hpxml_core::v4::parse(&bytes).unwrap();
        assert_eq!(doc, reparsed);
    }

    #[test]
    fn parse_modify_serialize() {
        let bytes = fixture!("v4/minimal.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();

        assert_eq!(doc.schema_version, SchemaVersionType::_42);
        doc.schema_version = SchemaVersionType::_40;

        let second = {
            let mut b = doc
                .building
                .first()
                .expect("fixture has a building")
                .as_ref()
                .unwrap()
                .clone();
            b.building_id = building_id("Building2");
            b
        };
        doc.building.push(Nillable::new(second));

        let reparsed = roundtrip(&doc);
        assert_eq!(reparsed.schema_version, SchemaVersionType::_40);
        assert_eq!(reparsed.building.len(), 2);
    }

    #[test]
    fn add_customer_to_document() {
        let bytes = fixture!("v4/minimal.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();

        assert!(doc.customer.is_empty());

        let customer = Customer {
            data_source: None,
            customer_details: CustomerCustomerDetails {
                data_source: None,
                person: IndividualInfo {
                    data_source: None,
                    system_info: IndividualInfoSystemInfo {
                        system_identifier: building_id("customer1"),
                        external_resource: vec![],
                    },
                    name: IndividualInfoName {
                        data_source: None,
                        prefix_name: None,
                        first_name: str_type("Alice"),
                        middle_name: None,
                        last_name: str_type("Smith"),
                        suffix_name: None,
                        extension: None,
                    },
                    individual_type: None,
                    telephone: vec![],
                    email: vec![],
                    extension: None,
                },
                mailing_address: None,
                extension: None,
            },
            comments: None,
            other_contact: vec![],
            extension: None,
        };
        doc.customer.push(Nillable::new(customer));

        let reparsed = roundtrip(&doc);
        assert_eq!(reparsed.customer.len(), 1);
        let c = reparsed.customer[0].as_ref().unwrap();
        assert_eq!(c.customer_details.person.name.first_name.content, "Alice");
        assert_eq!(c.customer_details.person.name.last_name.content, "Smith");
    }

    #[test]
    fn nillable_mixed_nil_and_value_roundtrip() {
        let bytes = fixture!("v4/minimal.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();

        let b1 = minimal_building("First");
        let b2 = minimal_building("Third");
        doc.building = vec![
            Nillable::new(b1),
            Nillable::nil(),
            Nillable::new(b2),
            Nillable::nil(),
        ];

        let reparsed = roundtrip(&doc);
        assert_eq!(reparsed.building.len(), 4);
        for (i, expect_nil) in [false, true, false, true].iter().enumerate() {
            let entry = reparsed
                .building
                .get(i)
                .expect("reparsed doc has four buildings");
            assert_eq!(entry.is_nil(), *expect_nil, "building {i} nil state");
        }
        assert_eq!(
            reparsed
                .building
                .first()
                .expect("reparsed doc has a building")
                .as_ref()
                .unwrap()
                .building_id
                .id,
            "First"
        );
        assert_eq!(
            reparsed
                .building
                .get(2)
                .expect("reparsed doc has three buildings")
                .as_ref()
                .unwrap()
                .building_id
                .id,
            "Third"
        );
    }

    #[test]
    fn nillable_empty_vec_roundtrip() {
        let bytes = fixture!("v4/minimal.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();
        doc.building = vec![];

        let reparsed = roundtrip(&doc);
        assert!(reparsed.building.is_empty());
    }

    #[test]
    fn inspect_then_parse_workflow() {
        let bytes = fixture!("v4/audit.xml");

        let info = hpxml_core::inspect::inspect(bytes).unwrap();
        assert_eq!(info.version, hpxml_core::HpxmlVersion::V4);
        assert!(info.schema_version.starts_with("4."));

        let doc = match info.version {
            hpxml_core::HpxmlVersion::V4 => hpxml_core::v4::parse(bytes).unwrap(),
            _ => panic!("expected v4"),
        };
        assert!(!doc.building.is_empty());
    }

    #[test]
    fn serialize_to_writer() {
        let bytes = fixture!("v4/minimal.xml");
        let doc = hpxml_core::v4::parse(bytes).unwrap();

        let mut cursor = std::io::Cursor::new(Vec::<u8>::new());
        doc.to_xml_into(&mut cursor).unwrap();
        let written = cursor.into_inner();

        assert!(!written.is_empty());
        let reparsed = hpxml_core::v4::parse(&written).unwrap();
        assert_eq!(doc, reparsed);
    }

    #[test]
    fn clone_and_modify_building_details() {
        let bytes = fixture!("v4/audit.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();

        let mut cloned = doc
            .building
            .first()
            .expect("fixture has a building")
            .as_ref()
            .unwrap()
            .clone();
        cloned.building_id = building_id("ClonedBuilding");
        cloned.building_details.systems = None;
        cloned.building_details.appliances = None;
        cloned.site = None;

        doc.building.push(Nillable::new(cloned));

        let reparsed = roundtrip(&doc);
        let found = reparsed
            .building
            .iter()
            .find_map(|b| b.as_ref().filter(|b| b.building_id.id == "ClonedBuilding"))
            .expect("cloned building not found");

        assert!(found.building_details.systems.is_none());
        assert!(found.building_details.appliances.is_none());
        assert!(found.site.is_none());
    }

    #[test]
    fn enum_in_wrapper_struct_roundtrip() {
        let event = EventType {
            data_source: None,
            content: EventTypeSimple::Audit,
        };

        let doc = HpxmlType {
            schema_version: SchemaVersionType::_42,
            data_source: None,
            xml_transaction_header_information: transaction_header("HPXML", "test"),
            software_info: empty_software_info(),
            contractor: vec![],
            customer: vec![],
            building: vec![Nillable::new(Building {
                data_source: None,
                building_id: building_id("B1"),
                external_resource: vec![],
                customer_id: None,
                site: None,
                contractor_id: None,
                project_status: ProjectStatusElementType {
                    data_source: None,
                    event_type: event,
                    date: None,
                    extension: None,
                },
                building_details: empty_building_details(),
                modeled_usages: None,
                extension: None,
            })],
            project: vec![],
            utility: vec![],
            consumption: vec![],
        };

        let reparsed = roundtrip(&doc);
        let b = reparsed
            .building
            .first()
            .expect("reparsed doc has a building")
            .as_ref()
            .unwrap();
        assert_eq!(b.project_status.event_type.content, EventTypeSimple::Audit);
    }

    #[test]
    fn enum_all_schema_versions_roundtrip() {
        for version in [
            SchemaVersionType::_40,
            SchemaVersionType::_41,
            SchemaVersionType::_42,
        ] {
            let doc = HpxmlType {
                schema_version: version.clone(),
                data_source: None,
                xml_transaction_header_information: transaction_header("HPXML", "test"),
                software_info: empty_software_info(),
                contractor: vec![],
                customer: vec![],
                building: vec![Nillable::new(minimal_building("B"))],
                project: vec![],
                utility: vec![],
                consumption: vec![],
            };
            let reparsed = roundtrip(&doc);
            assert_eq!(reparsed.schema_version, version);
        }
    }

    #[test]
    fn modify_enum_field_and_roundtrip() {
        let bytes = fixture!("v4/audit.xml");
        let mut doc = hpxml_core::v4::parse(bytes).unwrap();

        let b = doc
            .building
            .first_mut()
            .expect("fixture has a building")
            .as_mut()
            .unwrap();
        b.project_status.event_type.content = EventTypeSimple::ProposedWorkscope;

        let reparsed = roundtrip(&doc);
        let b = reparsed
            .building
            .first()
            .expect("reparsed doc has a building")
            .as_ref()
            .unwrap();
        assert_eq!(
            b.project_status.event_type.content,
            EventTypeSimple::ProposedWorkscope
        );
    }

    #[test]
    fn enum_clone_debug_eq() {
        let variant = TransactionTypeSimple::Create;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
        assert!(format!("{:?}", variant).contains("Create"));

        assert_ne!(FuelTypeSimple::NaturalGas, FuelTypeSimple::Electricity);
        assert_ne!(FuelTypeSimple::NaturalGas, FuelTypeSimple::Propane);
    }

    #[test]
    fn pattern_match_on_enum() {
        let bytes = fixture!("v4/audit.xml");
        let doc = hpxml_core::v4::parse(bytes).unwrap();

        let b = doc
            .building
            .first()
            .expect("fixture has a building")
            .as_ref()
            .unwrap();
        let label = match &b.project_status.event_type.content {
            EventTypeSimple::Audit => "audit",
            EventTypeSimple::ProposedWorkscope => "proposed",
            EventTypeSimple::ApprovedWorkscope => "approved",
            _ => "other",
        };
        assert_eq!(label, "audit");
    }
}
