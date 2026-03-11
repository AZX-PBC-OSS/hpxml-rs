// GENERATED CODE - do not edit manually.
// Regenerate with: scripts/codegen.sh
pub type Hpxml = HpxmlElementType;
#[derive(Clone, Debug, PartialEq)]
pub struct HpxmlElementType {
    pub schema_version: super::hpxml_data_types::SchemaVersionType,
    pub xml_transaction_header_information: super::base_elements::XmlTransactionHeaderInformation,
    pub software_info: super::base_elements::SoftwareInfo,
    pub contractor:
        ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Contractor>>,
    pub customer:
        ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Customer>>,
    pub building:
        ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Building>>,
    pub project: ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Project>>,
    pub utility: ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Utility>>,
    pub consumption:
        ::std::vec::Vec<::xsd_parser_types::xml::Nillable<super::base_elements::Consumption>>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for HpxmlElementType {
    type Serializer<'x> = quick_xml_serialize::HpxmlElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::HpxmlElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::HpxmlElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("HPXML"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for HpxmlElementType {
    type Deserializer = quick_xml_deserialize::HpxmlElementTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use xsd_parser_types::quick_xml::Deserializer as _;
    #[derive(Debug)]
    pub struct HpxmlElementTypeDeserializer {
        schema_version: super::super::hpxml_data_types::SchemaVersionType,
        xml_transaction_header_information:
            ::core::option::Option<super::super::base_elements::XmlTransactionHeaderInformation>,
        software_info: ::core::option::Option<super::super::base_elements::SoftwareInfo>,
        contractor: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Contractor>,
        >,
        customer: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Customer>,
        >,
        building: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Building>,
        >,
        project: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Project>,
        >,
        utility: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Utility>,
        >,
        consumption: ::std::vec::Vec<
            ::xsd_parser_types::xml::Nillable<super::super::base_elements::Consumption>,
        >,
        state__: ::std::boxed::Box<HpxmlElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HpxmlElementTypeDeserializerState {
        Init__ , XmlTransactionHeaderInformation (:: core :: option :: Option << super :: super :: base_elements :: XmlTransactionHeaderInformation as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , SoftwareInfo (:: core :: option :: Option << super :: super :: base_elements :: SoftwareInfo as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Contractor (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Contractor > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Customer (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Customer > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Building (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Building > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Project (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Project > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Utility (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Utility > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Consumption (:: core :: option :: Option << :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Consumption > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl HpxmlElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut schema_version: ::core::option::Option<
                super::super::hpxml_data_types::SchemaVersionType,
            > = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::super::NS_UNNAMED_5),
                    Some(b"schemaVersion")
                ) {
                    helper.read_attrib(&mut schema_version, b"schemaVersion", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                schema_version: schema_version.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute(
                        "schemaVersion".into(),
                    )
                })?,
                xml_transaction_header_information: None,
                software_info: None,
                contractor: ::std::vec::Vec::new(),
                customer: ::std::vec::Vec::new(),
                building: ::std::vec::Vec::new(),
                project: ::std::vec::Vec::new(),
                utility: ::std::vec::Vec::new(),
                consumption: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(HpxmlElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: HpxmlElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use HpxmlElementTypeDeserializerState as S;
            match state {
                S::XmlTransactionHeaderInformation(Some(deserializer)) => {
                    self.store_xml_transaction_header_information(deserializer.finish(helper)?)?
                }
                S::SoftwareInfo(Some(deserializer)) => {
                    self.store_software_info(deserializer.finish(helper)?)?
                }
                S::Contractor(Some(deserializer)) => {
                    self.store_contractor(deserializer.finish(helper)?)?
                }
                S::Customer(Some(deserializer)) => {
                    self.store_customer(deserializer.finish(helper)?)?
                }
                S::Building(Some(deserializer)) => {
                    self.store_building(deserializer.finish(helper)?)?
                }
                S::Project(Some(deserializer)) => {
                    self.store_project(deserializer.finish(helper)?)?
                }
                S::Utility(Some(deserializer)) => {
                    self.store_utility(deserializer.finish(helper)?)?
                }
                S::Consumption(Some(deserializer)) => {
                    self.store_consumption(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_xml_transaction_header_information(
            &mut self,
            value: super::super::base_elements::XmlTransactionHeaderInformation,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.xml_transaction_header_information.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(
                        b"XMLTransactionHeaderInformation",
                    ),
                ))?;
            }
            self.xml_transaction_header_information = Some(value);
            Ok(())
        }
        fn store_software_info(
            &mut self,
            value: super::super::base_elements::SoftwareInfo,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.software_info.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"SoftwareInfo"),
                ))?;
            }
            self.software_info = Some(value);
            Ok(())
        }
        fn store_contractor(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Contractor>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.contractor.push(value);
            Ok(())
        }
        fn store_customer(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Customer>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.customer.push(value);
            Ok(())
        }
        fn store_building(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Building>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.building.push(value);
            Ok(())
        }
        fn store_project(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Project>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.project.push(value);
            Ok(())
        }
        fn store_utility(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Utility>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.utility.push(value);
            Ok(())
        }
        fn store_consumption(
            &mut self,
            value: ::xsd_parser_types::xml::Nillable<super::super::base_elements::Consumption>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.consumption.push(value);
            Ok(())
        }
        fn handle_xml_transaction_header_information<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::super::base_elements::XmlTransactionHeaderInformation,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::XmlTransactionHeaderInformation(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                } else {
                    return Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                            event, allow_any,
                        ),
                    );
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_xml_transaction_header_information(data)?;
                    *self.state__ = S::SoftwareInfo(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::XmlTransactionHeaderInformation(Some(deserializer)));
                    *self.state__ = S::SoftwareInfo(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_software_info<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::super::base_elements::SoftwareInfo,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SoftwareInfo(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                } else {
                    return Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                            event, allow_any,
                        ),
                    );
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_software_info(data)?;
                    *self.state__ = S::Contractor(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SoftwareInfo(Some(deserializer)));
                    *self.state__ = S::Contractor(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_contractor<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Contractor>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Customer(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_contractor(data)?;
                    *self.state__ = S::Contractor(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Contractor(Some(deserializer)));
                    *self.state__ = S::Contractor(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_customer<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Customer>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Building(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_customer(data)?;
                    *self.state__ = S::Customer(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Customer(Some(deserializer)));
                    *self.state__ = S::Customer(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_building<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Building>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Project(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_building(data)?;
                    *self.state__ = S::Building(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Building(Some(deserializer)));
                    *self.state__ = S::Building(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_project<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Project>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Utility(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_project(data)?;
                    *self.state__ = S::Project(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Project(Some(deserializer)));
                    *self.state__ = S::Project(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_utility<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Utility>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Consumption(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_utility(data)?;
                    *self.state__ = S::Utility(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Utility(Some(deserializer)));
                    *self.state__ = S::Utility(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_consumption<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Nillable<super::super::base_elements::Consumption>,
            >,
            fallback: &mut ::core::option::Option<HpxmlElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use HpxmlElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_consumption(data)?;
                    *self.state__ = S::Consumption(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Consumption(Some(deserializer)));
                    *self.state__ = S::Consumption(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::HpxmlElementType>
        for HpxmlElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::HpxmlElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::HpxmlElementType>
        {
            use HpxmlElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::XmlTransactionHeaderInformation(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_xml_transaction_header_information(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::SoftwareInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_software_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Contractor(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_contractor(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Customer(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_customer(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Building(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_building(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Project(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_project(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Utility(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_utility(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Consumption(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_consumption(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::XmlTransactionHeaderInformation(None);
                        event
                    }
                    (
                        S::XmlTransactionHeaderInformation(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"XMLTransactionHeaderInformation",
                            false,
                        )?;
                        match self.handle_xml_transaction_header_information(
                            helper,
                            output,
                            &mut fallback,
                        )? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::SoftwareInfo(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"SoftwareInfo",
                            false,
                        )?;
                        match self.handle_software_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Contractor(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Contractor",
                            false,
                        )?;
                        match self.handle_contractor(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Customer(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Customer",
                            false,
                        )?;
                        match self.handle_customer(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Building(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Building",
                            false,
                        )?;
                        match self.handle_building(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Project(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Project",
                            false,
                        )?;
                        match self.handle_project(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Utility(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Utility",
                            false,
                        )?;
                        match self.handle_utility(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Consumption(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::super::NS_UNNAMED_5),
                            b"Consumption",
                            false,
                        )?;
                        match self.handle_consumption(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::HpxmlElementType, ::xsd_parser_types::quick_xml::Error>
        {
            let state = ::core::mem::replace(
                &mut *self.state__,
                HpxmlElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::HpxmlElementType {
                schema_version: self.schema_version,
                xml_transaction_header_information: helper.finish_element(
                    "XMLTransactionHeaderInformation",
                    self.xml_transaction_header_information,
                )?,
                software_info: helper.finish_element("SoftwareInfo", self.software_info)?,
                contractor: self.contractor,
                customer: self.customer,
                building: self.building,
                project: self.project,
                utility: self.utility,
                consumption: self.consumption,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::Serializer as _;
    #[derive(Debug)]
    pub struct HpxmlElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::HpxmlElementType,
        pub(super) state: ::std::boxed::Box<HpxmlElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum HpxmlElementTypeSerializerState<'ser> {
        Init__ , XmlTransactionHeaderInformation (< super :: super :: base_elements :: XmlTransactionHeaderInformation as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , SoftwareInfo (< super :: super :: base_elements :: SoftwareInfo as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Contractor (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Contractor >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Contractor > >) , Customer (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Customer >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Customer > >) , Building (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Building >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Building > >) , Project (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Project >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Project > >) , Utility (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Utility >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Utility > >) , Consumption (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [:: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Consumption >] , :: xsd_parser_types :: xml :: Nillable < super :: super :: base_elements :: Consumption > >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
    impl<'ser> HpxmlElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    HpxmlElementTypeSerializerState::Init__ => {
                        *self.state =
                            HpxmlElementTypeSerializerState::XmlTransactionHeaderInformation(
                                ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                    &self.value.xml_transaction_header_information,
                                    Some("XMLTransactionHeaderInformation"),
                                    false,
                                )?,
                            );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::super::NS_UNNAMED_5);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::super::PREFIX_XSI),
                                &super::super::NS_XSI,
                            );
                        }
                        helper.write_attrib(
                            &mut bytes,
                            "schemaVersion",
                            &self.value.schema_version,
                        )?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    HpxmlElementTypeSerializerState::XmlTransactionHeaderInformation(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::SoftwareInfo(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.software_info,
                                        Some("SoftwareInfo"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::SoftwareInfo(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Contractor(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.contractor[..],
                                        Some("Contractor"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Contractor(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Customer(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.customer[..],
                                        Some("Customer"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Customer(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Building(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.building[..],
                                        Some("Building"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Building(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Project(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.project[..],
                                        Some("Project"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Project(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Utility(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.utility[..],
                                        Some("Utility"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Utility(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = HpxmlElementTypeSerializerState::Consumption(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.consumption[..],
                                        Some("Consumption"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    HpxmlElementTypeSerializerState::Consumption(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = HpxmlElementTypeSerializerState::End__,
                        }
                    }
                    HpxmlElementTypeSerializerState::End__ => {
                        *self.state = HpxmlElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    HpxmlElementTypeSerializerState::Done__ => return Ok(None),
                    HpxmlElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for HpxmlElementTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HpxmlElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
