// GENERATED CODE - do not edit manually.
// Regenerate with: scripts/codegen.sh
use xsd_parser_types::xml::{AnyAttributes, AnyElement};
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Entities(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for Entities {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Entities {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Entities {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Entities {}
pub type Entity = ::std::string::String;
pub type Id = ::std::string::String;
pub type Idref = ::std::string::String;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Idrefs(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for Idrefs {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Idrefs {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Idrefs {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Idrefs {}
pub type NcName = ::std::string::String;
pub type Nmtoken = ::std::string::String;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Nmtokens(pub ::std::vec::Vec<::std::string::String>);
impl ::xsd_parser_types::quick_xml::SerializeBytes for Nmtokens {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = ::std::string::String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(::std::borrow::Cow::Owned(data)))
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Nmtokens {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Nmtokens {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Nmtokens {}
pub type Notation = ::std::string::String;
pub type Name = ::std::string::String;
pub type QName = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct AnySimpleType {
    pub type_: ::core::option::Option<::std::string::String>,
    pub content: Content1,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for AnySimpleType {
    type Serializer<'x> = quick_xml_serialize::AnySimpleTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::AnySimpleTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::AnySimpleTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("anySimpleType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for AnySimpleType {
    type Deserializer = quick_xml_deserialize::AnySimpleTypeDeserializer;
}
#[derive(Clone, Debug, PartialEq)]
pub struct AnyType {
    pub any_attribute: AnyAttributes,
    pub text_before: ::core::option::Option<::xsd_parser_types::xml::Text>,
    pub any: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<AnyElement>>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for AnyType {
    type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::AnyTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
            name: name.unwrap_or("anyType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for AnyType {
    type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
}
pub type AnyUri = ::std::string::String;
pub type Base64Binary = ::std::string::String;
pub type Boolean = ::core::primitive::bool;
pub type Byte = ::core::primitive::i8;
pub type Date = ::std::string::String;
pub type DateTime = ::std::string::String;
pub type Decimal = ::core::primitive::f64;
pub type Double = ::core::primitive::f64;
pub type Duration = ::std::string::String;
pub type Float = ::core::primitive::f32;
pub type GDay = ::std::string::String;
pub type GMonth = ::std::string::String;
pub type GMonthDay = ::std::string::String;
pub type GYear = ::std::string::String;
pub type GYearMonth = ::std::string::String;
pub type HexBinary = ::std::string::String;
pub type Int = ::core::primitive::i32;
pub type Integer = ::core::primitive::i32;
pub type Language = ::std::string::String;
pub type Long = ::core::primitive::i64;
pub type NegativeInteger = ::core::num::NonZeroIsize;
pub type NonNegativeInteger = ::core::primitive::usize;
pub type NonPositiveInteger = ::core::primitive::isize;
pub type NormalizedString = ::std::string::String;
pub type PositiveInteger = ::core::num::NonZeroUsize;
pub type Short = ::core::primitive::i16;
pub type String = ::std::string::String;
pub type Time = ::std::string::String;
pub type Token = ::std::string::String;
pub type UnsignedByte = ::core::primitive::u8;
pub type UnsignedInt = ::core::primitive::u32;
pub type UnsignedLong = ::core::primitive::u64;
pub type UnsignedShort = ::core::primitive::u16;
#[derive(Clone, Debug, PartialEq)]
pub struct Content1(pub ::std::string::String);
impl Content1 {
    pub fn new(
        inner: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::std::string::String {
        self.0
    }
}
impl ::core::convert::From<Content1> for ::std::string::String {
    fn from(value: Content1) -> ::std::string::String {
        value.0
    }
}
impl ::core::convert::TryFrom<::std::string::String> for Content1 {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Content1 {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Content1 {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        self.0.serialize_bytes(helper)
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Content1 {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Content1 {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::std::string::String::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Content1 {}
pub mod quick_xml_deserialize {
    use xsd_parser_types::{
        quick_xml::Deserializer as _,
        xml::{AnyAttributes, AnyElement},
    };
    #[derive(Debug)]
    pub struct AnySimpleTypeDeserializer {
        type_: ::core::option::Option<::std::string::String>,
        content: ::core::option::Option<super::Content1>,
        state__: ::std::boxed::Box<AnySimpleTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnySimpleTypeDeserializerState {
        Init__,
        Content__(
            <super::Content1 as ::xsd_parser_types::quick_xml::WithDeserializer>::Deserializer,
        ),
        Unknown__,
    }
    impl AnySimpleTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut type_: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::super::NS_XSI),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                type_: type_,
                content: None,
                state__: ::std::boxed::Box::new(AnySimpleTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: AnySimpleTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if let AnySimpleTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::Content1,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.content.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::Content1>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnySimpleType> {
            use AnySimpleTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => {
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::AnySimpleType>
        for AnySimpleTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnySimpleType> {
            let (::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x)) = &event
            else {
                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnySimpleType> {
            use AnySimpleTypeDeserializerState as S;
            match ::core::mem::replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output =
                        ::xsd_parser_types::quick_xml::ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::AnySimpleType, ::xsd_parser_types::quick_xml::Error>
        {
            let state = ::core::mem::replace(
                &mut *self.state__,
                AnySimpleTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::AnySimpleType {
                type_: self.type_,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnyTypeDeserializer {
        any_attribute: AnyAttributes,
        text_before: ::core::option::Option<::xsd_parser_types::xml::Text>,
        any: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<AnyElement>>,
        state__: ::std::boxed::Box<AnyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyTypeDeserializerState {
        Init__ , TextBefore (:: core :: option :: Option << :: xsd_parser_types :: xml :: Text as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Any (:: core :: option :: Option << :: xsd_parser_types :: xml :: Mixed < AnyElement > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl AnyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut any_attribute = AnyAttributes::default();
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                any_attribute.push(attrib)?;
            }
            Ok(Self {
                any_attribute: any_attribute,
                text_before: None,
                any: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(AnyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: AnyTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use AnyTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(
            &mut self,
            value: ::xsd_parser_types::xml::Text,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.text_before.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"text_before"),
                ))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_any(
            &mut self,
            value: ::xsd_parser_types::xml::Mixed<AnyElement>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.any.push(value);
            Ok(())
        }
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Text,
            >,
            fallback: &mut ::core::option::Option<AnyTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use AnyTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Any(None);
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
                    self.store_text_before(data)?;
                    *self.state__ = S::Any(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Mixed<AnyElement>,
            >,
            fallback: &mut ::core::option::Option<AnyTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use AnyTypeDeserializerState as S;
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
                    self.store_any(data)?;
                    *self.state__ = S::Any(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::AnyType> {
            use AnyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                    (S::Any(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = < :: xsd_parser_types :: xml :: Text as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                        match self.handle_text_before(helper, output, &mut fallback)? {
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
                        S::Any(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        if is_any_retry {
                            let output = < :: xsd_parser_types :: xml :: Mixed < AnyElement > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                            match self.handle_any(helper, output, &mut fallback)? {
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
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            *self.state__ = S::Done__;
                            break (
                                ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                                allow_any_element,
                            );
                        }
                    }
                    (
                        state,
                        ::xsd_parser_types::quick_xml::Event::Text(_)
                        | ::xsd_parser_types::quick_xml::Event::CData(_),
                    ) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            false,
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
        ) -> ::core::result::Result<super::AnyType, ::xsd_parser_types::quick_xml::Error> {
            let state =
                ::core::mem::replace(&mut *self.state__, AnyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnyType {
                any_attribute: self.any_attribute,
                text_before: self.text_before,
                any: self.any,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{quick_xml::Serializer as _, xml::AnyElement};
    #[derive(Debug)]
    pub struct AnySimpleTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnySimpleType,
        pub(super) state: ::std::boxed::Box<AnySimpleTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum AnySimpleTypeSerializerState<'ser> {
        Init__,
        Content__(
            <super::Content1 as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnySimpleTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    AnySimpleTypeSerializerState::Init__ => {
                        *self.state = AnySimpleTypeSerializerState::Content__(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.content,
                                None,
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::super::PREFIX_XSI),
                                &super::super::NS_XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "xsi:type", &self.value.type_)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    AnySimpleTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = AnySimpleTypeSerializerState::End__,
                        }
                    }
                    AnySimpleTypeSerializerState::End__ => {
                        *self.state = AnySimpleTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    AnySimpleTypeSerializerState::Done__ => return Ok(None),
                    AnySimpleTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for AnySimpleTypeSerializer<'ser> {
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
                    *self.state = AnySimpleTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnyTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnyType,
        pub(super) state: ::std::boxed::Box<AnyTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum AnyTypeSerializerState<'ser> {
        Init__,
        TextBefore(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::xsd_parser_types::xml::Text>,
                ::xsd_parser_types::xml::Text,
            >,
        ),
        Any(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::xsd_parser_types::xml::Mixed<AnyElement>],
                ::xsd_parser_types::xml::Mixed<AnyElement>,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    AnyTypeSerializerState::Init__ => {
                        *self.state = AnyTypeSerializerState::TextBefore(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.text_before.as_ref(),
                                Some(""),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::super::PREFIX_XSI),
                                &super::super::NS_XSI,
                            );
                        }
                        bytes.extend_attributes(self.value.any_attribute.attributes());
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    AnyTypeSerializerState::TextBefore(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = AnyTypeSerializerState::Any(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    &self.value.any[..],
                                    None,
                                    false,
                                ),
                            )
                        }
                    },
                    AnyTypeSerializerState::Any(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnyTypeSerializerState::End__,
                    },
                    AnyTypeSerializerState::End__ => {
                        *self.state = AnyTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    AnyTypeSerializerState::Done__ => return Ok(None),
                    AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for AnyTypeSerializer<'ser> {
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
                    *self.state = AnyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
