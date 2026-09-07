// GENERATED CODE - do not edit manually.
// Regenerate with: scripts/codegen.sh
#[derive(Clone, Debug, PartialEq)]
pub struct Afue(pub ::core::primitive::f64);
impl Afue {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("1"));
        }
        if *value > 100f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "100",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Afue> for ::core::primitive::f64 {
    fn from(value: Afue) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Afue {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Afue {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Afue {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Afue {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Afue {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Afue {}
#[derive(Clone, Debug, PartialEq)]
pub enum AddressTypeCode {
    Street,
    Mailing,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AddressTypeCode {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Street => Ok(Some(::std::borrow::Cow::Borrowed("street"))),
            Self::Mailing => Ok(Some(::std::borrow::Cow::Borrowed("mailing"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AddressTypeCode {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AddressTypeCode {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"street" => Ok(Self::Street),
            b"mailing" => Ok(Self::Mailing),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AddressTypeCode {}
#[derive(Clone, Debug, PartialEq)]
pub enum AdjacentTo {
    Ambient,
    Garage,
    Attic,
    Crawlspace,
    Ground,
    LivingSpace,
    UnconditionedBasement,
    OtherHousingUnit,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AdjacentTo {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Ambient => Ok(Some(::std::borrow::Cow::Borrowed("ambient"))),
            Self::Garage => Ok(Some(::std::borrow::Cow::Borrowed("garage"))),
            Self::Attic => Ok(Some(::std::borrow::Cow::Borrowed("attic"))),
            Self::Crawlspace => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace"))),
            Self::Ground => Ok(Some(::std::borrow::Cow::Borrowed("ground"))),
            Self::LivingSpace => Ok(Some(::std::borrow::Cow::Borrowed("living space"))),
            Self::UnconditionedBasement => {
                Ok(Some(::std::borrow::Cow::Borrowed("unconditioned basement")))
            }
            Self::OtherHousingUnit => Ok(Some(::std::borrow::Cow::Borrowed("other housing unit"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AdjacentTo {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AdjacentTo {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"ambient" => Ok(Self::Ambient),
            b"garage" => Ok(Self::Garage),
            b"attic" => Ok(Self::Attic),
            b"crawlspace" => Ok(Self::Crawlspace),
            b"ground" => Ok(Self::Ground),
            b"living space" => Ok(Self::LivingSpace),
            b"unconditioned basement" => Ok(Self::UnconditionedBasement),
            b"other housing unit" => Ok(Self::OtherHousingUnit),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AdjacentTo {}
#[derive(Clone, Debug, PartialEq)]
pub enum AirDistributionType {
    RegularVelocity,
    HighVelocity,
    Gravity,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AirDistributionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::RegularVelocity => Ok(Some(::std::borrow::Cow::Borrowed("regular velocity"))),
            Self::HighVelocity => Ok(Some(::std::borrow::Cow::Borrowed("high velocity"))),
            Self::Gravity => Ok(Some(::std::borrow::Cow::Borrowed("gravity"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AirDistributionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AirDistributionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"regular velocity" => Ok(Self::RegularVelocity),
            b"high velocity" => Ok(Self::HighVelocity),
            b"gravity" => Ok(Self::Gravity),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AirDistributionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum AirHandlerMotorType {
    PscSingleSpeed,
    PscMultiSpeed,
    Ecm,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AirHandlerMotorType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::PscSingleSpeed => Ok(Some(::std::borrow::Cow::Borrowed("PSC single-speed"))),
            Self::PscMultiSpeed => Ok(Some(::std::borrow::Cow::Borrowed("PSC multi-speed"))),
            Self::Ecm => Ok(Some(::std::borrow::Cow::Borrowed("ECM"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AirHandlerMotorType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AirHandlerMotorType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"PSC single-speed" => Ok(Self::PscSingleSpeed),
            b"PSC multi-speed" => Ok(Self::PscMultiSpeed),
            b"ECM" => Ok(Self::Ecm),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AirHandlerMotorType {}
#[derive(Clone, Debug, PartialEq)]
pub enum AirHandlerStaticPressureMeasurementLocation {
    InDucts,
    AtEquipment,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AirHandlerStaticPressureMeasurementLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::InDucts => Ok(Some(::std::borrow::Cow::Borrowed("in ducts"))),
            Self::AtEquipment => Ok(Some(::std::borrow::Cow::Borrowed("at equipment"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes
    for AirHandlerStaticPressureMeasurementLocation
{
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes
    for AirHandlerStaticPressureMeasurementLocation
{
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"in ducts" => Ok(Self::InDucts),
            b"at equipment" => Ok(Self::AtEquipment),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for AirHandlerStaticPressureMeasurementLocation
{
}
pub type AnnualAmount = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum ApplianceThirdPartyCertifications {
    EnergyStar,
    EnergyStarMostEfficient,
    CeeTier1,
    CeeTier2,
    CeeTier3,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ApplianceThirdPartyCertifications {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::EnergyStarMostEfficient => Ok(Some(::std::borrow::Cow::Borrowed(
                "Energy Star Most Efficient",
            ))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 3"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ApplianceThirdPartyCertifications {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ApplianceThirdPartyCertifications {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"Energy Star Most Efficient" => Ok(Self::EnergyStarMostEfficient),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"CEE Tier 3" => Ok(Self::CeeTier3),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ApplianceThirdPartyCertifications {}
#[derive(Clone, Debug, PartialEq)]
pub enum AtticComponentsAirSealed {
    AtticFloor,
    TopPlates,
    KneewallTransitions,
    PlumbingWetWalls,
    ChimneyFlueChases,
    RecessedLights,
    AtticAccess,
    DroppedSoffit,
    AtticLevelTransitions,
    MechanicalChases,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AtticComponentsAirSealed {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AtticFloor => Ok(Some(::std::borrow::Cow::Borrowed("attic floor"))),
            Self::TopPlates => Ok(Some(::std::borrow::Cow::Borrowed("top plates"))),
            Self::KneewallTransitions => {
                Ok(Some(::std::borrow::Cow::Borrowed("kneewall transitions")))
            }
            Self::PlumbingWetWalls => Ok(Some(::std::borrow::Cow::Borrowed("plumbing wet walls"))),
            Self::ChimneyFlueChases => {
                Ok(Some(::std::borrow::Cow::Borrowed("chimney/flue chases")))
            }
            Self::RecessedLights => Ok(Some(::std::borrow::Cow::Borrowed("recessed lights"))),
            Self::AtticAccess => Ok(Some(::std::borrow::Cow::Borrowed("attic access"))),
            Self::DroppedSoffit => Ok(Some(::std::borrow::Cow::Borrowed("dropped soffit"))),
            Self::AtticLevelTransitions => Ok(Some(::std::borrow::Cow::Borrowed(
                "attic level transitions",
            ))),
            Self::MechanicalChases => Ok(Some(::std::borrow::Cow::Borrowed("mechanical chases"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AtticComponentsAirSealed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AtticComponentsAirSealed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"attic floor" => Ok(Self::AtticFloor),
            b"top plates" => Ok(Self::TopPlates),
            b"kneewall transitions" => Ok(Self::KneewallTransitions),
            b"plumbing wet walls" => Ok(Self::PlumbingWetWalls),
            b"chimney/flue chases" => Ok(Self::ChimneyFlueChases),
            b"recessed lights" => Ok(Self::RecessedLights),
            b"attic access" => Ok(Self::AtticAccess),
            b"dropped soffit" => Ok(Self::DroppedSoffit),
            b"attic level transitions" => Ok(Self::AtticLevelTransitions),
            b"mechanical chases" => Ok(Self::MechanicalChases),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AtticComponentsAirSealed {}
#[derive(Clone, Debug, PartialEq)]
pub enum AtticType {
    CapeCod,
    CathedralCeiling,
    FlatRoof,
    UnventedAttic,
    VentedAttic,
    VentingUnknownAttic,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AtticType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::CapeCod => Ok(Some(::std::borrow::Cow::Borrowed("cape cod"))),
            Self::CathedralCeiling => Ok(Some(::std::borrow::Cow::Borrowed("cathedral ceiling"))),
            Self::FlatRoof => Ok(Some(::std::borrow::Cow::Borrowed("flat roof"))),
            Self::UnventedAttic => Ok(Some(::std::borrow::Cow::Borrowed("unvented attic"))),
            Self::VentedAttic => Ok(Some(::std::borrow::Cow::Borrowed("vented attic"))),
            Self::VentingUnknownAttic => {
                Ok(Some(::std::borrow::Cow::Borrowed("venting unknown attic")))
            }
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AtticType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AtticType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"cape cod" => Ok(Self::CapeCod),
            b"cathedral ceiling" => Ok(Self::CathedralCeiling),
            b"flat roof" => Ok(Self::FlatRoof),
            b"unvented attic" => Ok(Self::UnventedAttic),
            b"vented attic" => Ok(Self::VentedAttic),
            b"venting unknown attic" => Ok(Self::VentingUnknownAttic),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AtticType {}
#[derive(Clone, Debug, PartialEq)]
pub enum AuditorQualification {
    Pe,
    Cem,
    BpiBa,
    ResnetHomePartner,
    Ra,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AuditorQualification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Pe => Ok(Some(::std::borrow::Cow::Borrowed("PE"))),
            Self::Cem => Ok(Some(::std::borrow::Cow::Borrowed("CEM"))),
            Self::BpiBa => Ok(Some(::std::borrow::Cow::Borrowed("BPI-BA"))),
            Self::ResnetHomePartner => {
                Ok(Some(::std::borrow::Cow::Borrowed("RESNET-Home Partner")))
            }
            Self::Ra => Ok(Some(::std::borrow::Cow::Borrowed("RA"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AuditorQualification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AuditorQualification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"PE" => Ok(Self::Pe),
            b"CEM" => Ok(Self::Cem),
            b"BPI-BA" => Ok(Self::BpiBa),
            b"RESNET-Home Partner" => Ok(Self::ResnetHomePartner),
            b"RA" => Ok(Self::Ra),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AuditorQualification {}
pub type AuditorRelationship = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct AzimuthType(pub ::core::primitive::i32);
impl AzimuthType {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value >= 360i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterEqualThan("360"));
        }
        Ok(())
    }
}
impl ::core::convert::From<AzimuthType> for ::core::primitive::i32 {
    fn from(value: AzimuthType) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for AzimuthType {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for AzimuthType {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for AzimuthType {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for AzimuthType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for AzimuthType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for AzimuthType {}
#[derive(Clone, Debug, PartialEq)]
pub enum Bpi2400CalibrationQualification {
    None,
    Detailed,
    Simple,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Bpi2400CalibrationQualification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
            Self::Detailed => Ok(Some(::std::borrow::Cow::Borrowed("detailed"))),
            Self::Simple => Ok(Some(::std::borrow::Cow::Borrowed("simple"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Bpi2400CalibrationQualification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Bpi2400CalibrationQualification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"none" => Ok(Self::None),
            b"detailed" => Ok(Self::Detailed),
            b"simple" => Ok(Self::Simple),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Bpi2400CalibrationQualification {}
#[derive(Clone, Debug, PartialEq)]
pub enum BasementCrawlspaceComponentsAirSealed {
    PlumbingPenetrations,
    Access,
    WiringPenetrations,
    ChimneyFlueChase,
    MechanicalChases,
    RimJoists,
    WindowsAndDoor,
    FoundationServicePenetrations,
    Cantilevers,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BasementCrawlspaceComponentsAirSealed {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::PlumbingPenetrations => {
                Ok(Some(::std::borrow::Cow::Borrowed("plumbing penetrations")))
            }
            Self::Access => Ok(Some(::std::borrow::Cow::Borrowed("access"))),
            Self::WiringPenetrations => {
                Ok(Some(::std::borrow::Cow::Borrowed("wiring penetrations")))
            }
            Self::ChimneyFlueChase => Ok(Some(::std::borrow::Cow::Borrowed("chimney/flue chase"))),
            Self::MechanicalChases => Ok(Some(::std::borrow::Cow::Borrowed("mechanical chases"))),
            Self::RimJoists => Ok(Some(::std::borrow::Cow::Borrowed("rim joists"))),
            Self::WindowsAndDoor => Ok(Some(::std::borrow::Cow::Borrowed("windows and door"))),
            Self::FoundationServicePenetrations => Ok(Some(::std::borrow::Cow::Borrowed(
                "foundation service penetrations",
            ))),
            Self::Cantilevers => Ok(Some(::std::borrow::Cow::Borrowed("cantilevers"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BasementCrawlspaceComponentsAirSealed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BasementCrawlspaceComponentsAirSealed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"plumbing penetrations" => Ok(Self::PlumbingPenetrations),
            b"access" => Ok(Self::Access),
            b"wiring penetrations" => Ok(Self::WiringPenetrations),
            b"chimney/flue chase" => Ok(Self::ChimneyFlueChase),
            b"mechanical chases" => Ok(Self::MechanicalChases),
            b"rim joists" => Ok(Self::RimJoists),
            b"windows and door" => Ok(Self::WindowsAndDoor),
            b"foundation service penetrations" => Ok(Self::FoundationServicePenetrations),
            b"cantilevers" => Ok(Self::Cantilevers),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for BasementCrawlspaceComponentsAirSealed
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum BoilerType {
    HotWater,
    Steam,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BoilerType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::HotWater => Ok(Some(::std::borrow::Cow::Borrowed("hot water"))),
            Self::Steam => Ok(Some(::std::borrow::Cow::Borrowed("steam"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BoilerType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BoilerType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"hot water" => Ok(Self::HotWater),
            b"steam" => Ok(Self::Steam),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BoilerType {}
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanWithNa {
    True,
    False,
    Na,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BooleanWithNa {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::True => Ok(Some(::std::borrow::Cow::Borrowed("true"))),
            Self::False => Ok(Some(::std::borrow::Cow::Borrowed("false"))),
            Self::Na => Ok(Some(::std::borrow::Cow::Borrowed("na"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BooleanWithNa {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BooleanWithNa {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"true" => Ok(Self::True),
            b"false" => Ok(Self::False),
            b"na" => Ok(Self::Na),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BooleanWithNa {}
#[derive(Clone, Debug, PartialEq)]
pub struct BuildingAirLeakage(pub ::core::primitive::f64);
impl BuildingAirLeakage {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<BuildingAirLeakage> for ::core::primitive::f64 {
    fn from(value: BuildingAirLeakage) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for BuildingAirLeakage {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for BuildingAirLeakage {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BuildingAirLeakage {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BuildingAirLeakage {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BuildingAirLeakage {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BuildingAirLeakage {}
#[derive(Clone, Debug, PartialEq)]
pub enum BuildingAirLeakageUnit {
    Cfm,
    CfMnatural,
    Ach,
    AcHnatural,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BuildingAirLeakageUnit {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cfm => Ok(Some(::std::borrow::Cow::Borrowed("CFM"))),
            Self::CfMnatural => Ok(Some(::std::borrow::Cow::Borrowed("CFMnatural"))),
            Self::Ach => Ok(Some(::std::borrow::Cow::Borrowed("ACH"))),
            Self::AcHnatural => Ok(Some(::std::borrow::Cow::Borrowed("ACHnatural"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BuildingAirLeakageUnit {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BuildingAirLeakageUnit {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"CFM" => Ok(Self::Cfm),
            b"CFMnatural" => Ok(Self::CfMnatural),
            b"ACH" => Ok(Self::Ach),
            b"ACHnatural" => Ok(Self::AcHnatural),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BuildingAirLeakageUnit {}
#[derive(Clone, Debug, PartialEq)]
pub enum BuildingLeakiness {
    VeryTight,
    Tight,
    Average,
    Leaky,
    VeryLeaky,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BuildingLeakiness {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::VeryTight => Ok(Some(::std::borrow::Cow::Borrowed("very tight"))),
            Self::Tight => Ok(Some(::std::borrow::Cow::Borrowed("tight"))),
            Self::Average => Ok(Some(::std::borrow::Cow::Borrowed("average"))),
            Self::Leaky => Ok(Some(::std::borrow::Cow::Borrowed("leaky"))),
            Self::VeryLeaky => Ok(Some(::std::borrow::Cow::Borrowed("very leaky"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BuildingLeakiness {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BuildingLeakiness {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"very tight" => Ok(Self::VeryTight),
            b"tight" => Ok(Self::Tight),
            b"average" => Ok(Self::Average),
            b"leaky" => Ok(Self::Leaky),
            b"very leaky" => Ok(Self::VeryLeaky),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BuildingLeakiness {}
#[derive(Clone, Debug, PartialEq)]
pub enum BusinessCertification {
    Bpi,
    Resnet,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BusinessCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Bpi => Ok(Some(::std::borrow::Cow::Borrowed("BPI"))),
            Self::Resnet => Ok(Some(::std::borrow::Cow::Borrowed("RESNET"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BusinessCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BusinessCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"BPI" => Ok(Self::Bpi),
            b"RESNET" => Ok(Self::Resnet),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BusinessCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum BusinessSpecialization {
    EnergyAudit,
    Hvac,
    Insulation,
    Carpentry,
    Plumbing,
    Electrical,
    Painting,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BusinessSpecialization {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyAudit => Ok(Some(::std::borrow::Cow::Borrowed("energy audit"))),
            Self::Hvac => Ok(Some(::std::borrow::Cow::Borrowed("hvac"))),
            Self::Insulation => Ok(Some(::std::borrow::Cow::Borrowed("insulation"))),
            Self::Carpentry => Ok(Some(::std::borrow::Cow::Borrowed("carpentry"))),
            Self::Plumbing => Ok(Some(::std::borrow::Cow::Borrowed("plumbing"))),
            Self::Electrical => Ok(Some(::std::borrow::Cow::Borrowed("electrical"))),
            Self::Painting => Ok(Some(::std::borrow::Cow::Borrowed("painting"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BusinessSpecialization {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BusinessSpecialization {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"energy audit" => Ok(Self::EnergyAudit),
            b"hvac" => Ok(Self::Hvac),
            b"insulation" => Ok(Self::Insulation),
            b"carpentry" => Ok(Self::Carpentry),
            b"plumbing" => Ok(Self::Plumbing),
            b"electrical" => Ok(Self::Electrical),
            b"painting" => Ok(Self::Painting),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BusinessSpecialization {}
#[derive(Clone, Debug, PartialEq)]
pub enum BusinessType {
    Contractor,
    Auditor,
    Subcontractor,
    PropertyManager,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for BusinessType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Contractor => Ok(Some(::std::borrow::Cow::Borrowed("contractor"))),
            Self::Auditor => Ok(Some(::std::borrow::Cow::Borrowed("auditor"))),
            Self::Subcontractor => Ok(Some(::std::borrow::Cow::Borrowed("subcontractor"))),
            Self::PropertyManager => Ok(Some(::std::borrow::Cow::Borrowed("property manager"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for BusinessType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for BusinessType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"contractor" => Ok(Self::Contractor),
            b"auditor" => Ok(Self::Auditor),
            b"subcontractor" => Ok(Self::Subcontractor),
            b"property manager" => Ok(Self::PropertyManager),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for BusinessType {}
pub type CazDepressurizationLimit = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub struct Cop(pub ::core::primitive::f64);
impl Cop {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Cop> for ::core::primitive::f64 {
    fn from(value: Cop) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Cop {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Cop {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Cop {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Cop {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Cop {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Cop {}
pub type CoReading = ::core::primitive::f64;
pub type Capacity = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum CertifyingOrganization {
    Usgbc,
    Nahb,
    EnergyStarHome,
    LocalProgram,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for CertifyingOrganization {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Usgbc => Ok(Some(::std::borrow::Cow::Borrowed("USGBC"))),
            Self::Nahb => Ok(Some(::std::borrow::Cow::Borrowed("NAHB"))),
            Self::EnergyStarHome => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star Home"))),
            Self::LocalProgram => Ok(Some(::std::borrow::Cow::Borrowed("local program"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for CertifyingOrganization {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for CertifyingOrganization {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"USGBC" => Ok(Self::Usgbc),
            b"NAHB" => Ok(Self::Nahb),
            b"Energy Star Home" => Ok(Self::EnergyStarHome),
            b"local program" => Ok(Self::LocalProgram),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for CertifyingOrganization {}
#[derive(Clone, Debug, PartialEq)]
pub enum ClimateZoneDoe {
    Subarctic,
    Marine,
    HotDry,
    MixedDry,
    HotHumid,
    MixedHumid,
    Cold,
    VeryCold,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ClimateZoneDoe {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Subarctic => Ok(Some(::std::borrow::Cow::Borrowed("subarctic"))),
            Self::Marine => Ok(Some(::std::borrow::Cow::Borrowed("marine"))),
            Self::HotDry => Ok(Some(::std::borrow::Cow::Borrowed("hot-dry"))),
            Self::MixedDry => Ok(Some(::std::borrow::Cow::Borrowed("mixed-dry"))),
            Self::HotHumid => Ok(Some(::std::borrow::Cow::Borrowed("hot-humid"))),
            Self::MixedHumid => Ok(Some(::std::borrow::Cow::Borrowed("mixed-humid"))),
            Self::Cold => Ok(Some(::std::borrow::Cow::Borrowed("cold"))),
            Self::VeryCold => Ok(Some(::std::borrow::Cow::Borrowed("very cold"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ClimateZoneDoe {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ClimateZoneDoe {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"subarctic" => Ok(Self::Subarctic),
            b"marine" => Ok(Self::Marine),
            b"hot-dry" => Ok(Self::HotDry),
            b"mixed-dry" => Ok(Self::MixedDry),
            b"hot-humid" => Ok(Self::HotHumid),
            b"mixed-humid" => Ok(Self::MixedHumid),
            b"cold" => Ok(Self::Cold),
            b"very cold" => Ok(Self::VeryCold),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ClimateZoneDoe {}
#[derive(Clone, Debug, PartialEq)]
pub enum ClimateZoneIecc {
    _1A,
    _1B,
    _1C,
    _2A,
    _2B,
    _2C,
    _3A,
    _3B,
    _3C,
    _4A,
    _4B,
    _4C,
    _5A,
    _5B,
    _5C,
    _6A,
    _6B,
    _6C,
    _7,
    _8,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ClimateZoneIecc {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_1A => Ok(Some(::std::borrow::Cow::Borrowed("1A"))),
            Self::_1B => Ok(Some(::std::borrow::Cow::Borrowed("1B"))),
            Self::_1C => Ok(Some(::std::borrow::Cow::Borrowed("1C"))),
            Self::_2A => Ok(Some(::std::borrow::Cow::Borrowed("2A"))),
            Self::_2B => Ok(Some(::std::borrow::Cow::Borrowed("2B"))),
            Self::_2C => Ok(Some(::std::borrow::Cow::Borrowed("2C"))),
            Self::_3A => Ok(Some(::std::borrow::Cow::Borrowed("3A"))),
            Self::_3B => Ok(Some(::std::borrow::Cow::Borrowed("3B"))),
            Self::_3C => Ok(Some(::std::borrow::Cow::Borrowed("3C"))),
            Self::_4A => Ok(Some(::std::borrow::Cow::Borrowed("4A"))),
            Self::_4B => Ok(Some(::std::borrow::Cow::Borrowed("4B"))),
            Self::_4C => Ok(Some(::std::borrow::Cow::Borrowed("4C"))),
            Self::_5A => Ok(Some(::std::borrow::Cow::Borrowed("5A"))),
            Self::_5B => Ok(Some(::std::borrow::Cow::Borrowed("5B"))),
            Self::_5C => Ok(Some(::std::borrow::Cow::Borrowed("5C"))),
            Self::_6A => Ok(Some(::std::borrow::Cow::Borrowed("6A"))),
            Self::_6B => Ok(Some(::std::borrow::Cow::Borrowed("6B"))),
            Self::_6C => Ok(Some(::std::borrow::Cow::Borrowed("6C"))),
            Self::_7 => Ok(Some(::std::borrow::Cow::Borrowed("7"))),
            Self::_8 => Ok(Some(::std::borrow::Cow::Borrowed("8"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ClimateZoneIecc {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ClimateZoneIecc {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"1A" => Ok(Self::_1A),
            b"1B" => Ok(Self::_1B),
            b"1C" => Ok(Self::_1C),
            b"2A" => Ok(Self::_2A),
            b"2B" => Ok(Self::_2B),
            b"2C" => Ok(Self::_2C),
            b"3A" => Ok(Self::_3A),
            b"3B" => Ok(Self::_3B),
            b"3C" => Ok(Self::_3C),
            b"4A" => Ok(Self::_4A),
            b"4B" => Ok(Self::_4B),
            b"4C" => Ok(Self::_4C),
            b"5A" => Ok(Self::_5A),
            b"5B" => Ok(Self::_5B),
            b"5C" => Ok(Self::_5C),
            b"6A" => Ok(Self::_6A),
            b"6B" => Ok(Self::_6B),
            b"6C" => Ok(Self::_6C),
            b"7" => Ok(Self::_7),
            b"8" => Ok(Self::_8),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ClimateZoneIecc {}
#[derive(Clone, Debug, PartialEq)]
pub enum ClothesDryerType {
    Dryer,
    AllInOneCombinationWasherDryer,
    UnitizedStackedWasherDryerPair,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ClothesDryerType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Dryer => Ok(Some(::std::borrow::Cow::Borrowed("dryer"))),
            Self::AllInOneCombinationWasherDryer => Ok(Some(::std::borrow::Cow::Borrowed(
                "all-in-one combination washer/dryer",
            ))),
            Self::UnitizedStackedWasherDryerPair => Ok(Some(::std::borrow::Cow::Borrowed(
                "unitized/stacked washer-dryer pair",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ClothesDryerType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ClothesDryerType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"dryer" => Ok(Self::Dryer),
            b"all-in-one combination washer/dryer" => Ok(Self::AllInOneCombinationWasherDryer),
            b"unitized/stacked washer-dryer pair" => Ok(Self::UnitizedStackedWasherDryerPair),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ClothesDryerType {}
#[derive(Clone, Debug, PartialEq)]
pub enum ClothesWasherType {
    TopLoader,
    FrontLoader,
    AllInOneCombinationWasherDryer,
    UnitizedStackedWasherDryerPair,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ClothesWasherType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::TopLoader => Ok(Some(::std::borrow::Cow::Borrowed("top loader"))),
            Self::FrontLoader => Ok(Some(::std::borrow::Cow::Borrowed("front loader"))),
            Self::AllInOneCombinationWasherDryer => Ok(Some(::std::borrow::Cow::Borrowed(
                "all-in-one combination washer/dryer",
            ))),
            Self::UnitizedStackedWasherDryerPair => Ok(Some(::std::borrow::Cow::Borrowed(
                "unitized/stacked washer-dryer pair",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ClothesWasherType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ClothesWasherType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"top loader" => Ok(Self::TopLoader),
            b"front loader" => Ok(Self::FrontLoader),
            b"all-in-one combination washer/dryer" => Ok(Self::AllInOneCombinationWasherDryer),
            b"unitized/stacked washer-dryer pair" => Ok(Self::UnitizedStackedWasherDryerPair),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ClothesWasherType {}
pub type CompleteDateActual = ::std::string::String;
pub type CompleteDateEstimated = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum ConsumptionType {
    Electricity,
    RenewableElectricity,
    NaturalGas,
    RenewableNaturalGas,
    FuelOil,
    FuelOil1,
    FuelOil2,
    FuelOil4,
    FuelOil56,
    DistrictSteam,
    DistrictHotWater,
    DistrictChilledWater,
    SolarHotWater,
    Propane,
    Kerosene,
    Diesel,
    AnthraciteCoal,
    BituminousCoal,
    Coke,
    Wood,
    WoodPellets,
    Combination,
    Water,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ConsumptionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Electricity => Ok(Some(::std::borrow::Cow::Borrowed("electricity"))),
            Self::RenewableElectricity => {
                Ok(Some(::std::borrow::Cow::Borrowed("renewable electricity")))
            }
            Self::NaturalGas => Ok(Some(::std::borrow::Cow::Borrowed("natural gas"))),
            Self::RenewableNaturalGas => {
                Ok(Some(::std::borrow::Cow::Borrowed("renewable natural gas")))
            }
            Self::FuelOil => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil"))),
            Self::FuelOil1 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 1"))),
            Self::FuelOil2 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 2"))),
            Self::FuelOil4 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 4"))),
            Self::FuelOil56 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 5/6"))),
            Self::DistrictSteam => Ok(Some(::std::borrow::Cow::Borrowed("district steam"))),
            Self::DistrictHotWater => Ok(Some(::std::borrow::Cow::Borrowed("district hot water"))),
            Self::DistrictChilledWater => {
                Ok(Some(::std::borrow::Cow::Borrowed("district chilled water")))
            }
            Self::SolarHotWater => Ok(Some(::std::borrow::Cow::Borrowed("solar hot water"))),
            Self::Propane => Ok(Some(::std::borrow::Cow::Borrowed("propane"))),
            Self::Kerosene => Ok(Some(::std::borrow::Cow::Borrowed("kerosene"))),
            Self::Diesel => Ok(Some(::std::borrow::Cow::Borrowed("diesel"))),
            Self::AnthraciteCoal => Ok(Some(::std::borrow::Cow::Borrowed("anthracite coal"))),
            Self::BituminousCoal => Ok(Some(::std::borrow::Cow::Borrowed("bituminous coal"))),
            Self::Coke => Ok(Some(::std::borrow::Cow::Borrowed("coke"))),
            Self::Wood => Ok(Some(::std::borrow::Cow::Borrowed("wood"))),
            Self::WoodPellets => Ok(Some(::std::borrow::Cow::Borrowed("wood pellets"))),
            Self::Combination => Ok(Some(::std::borrow::Cow::Borrowed("combination"))),
            Self::Water => Ok(Some(::std::borrow::Cow::Borrowed("water"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ConsumptionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ConsumptionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"electricity" => Ok(Self::Electricity),
            b"renewable electricity" => Ok(Self::RenewableElectricity),
            b"natural gas" => Ok(Self::NaturalGas),
            b"renewable natural gas" => Ok(Self::RenewableNaturalGas),
            b"fuel oil" => Ok(Self::FuelOil),
            b"fuel oil 1" => Ok(Self::FuelOil1),
            b"fuel oil 2" => Ok(Self::FuelOil2),
            b"fuel oil 4" => Ok(Self::FuelOil4),
            b"fuel oil 5/6" => Ok(Self::FuelOil56),
            b"district steam" => Ok(Self::DistrictSteam),
            b"district hot water" => Ok(Self::DistrictHotWater),
            b"district chilled water" => Ok(Self::DistrictChilledWater),
            b"solar hot water" => Ok(Self::SolarHotWater),
            b"propane" => Ok(Self::Propane),
            b"kerosene" => Ok(Self::Kerosene),
            b"diesel" => Ok(Self::Diesel),
            b"anthracite coal" => Ok(Self::AnthraciteCoal),
            b"bituminous coal" => Ok(Self::BituminousCoal),
            b"coke" => Ok(Self::Coke),
            b"wood" => Ok(Self::Wood),
            b"wood pellets" => Ok(Self::WoodPellets),
            b"combination" => Ok(Self::Combination),
            b"water" => Ok(Self::Water),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ConsumptionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum CoolingEfficiencyUnits {
    Seer,
    Eer,
    Cop,
    KWTon,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for CoolingEfficiencyUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Seer => Ok(Some(::std::borrow::Cow::Borrowed("SEER"))),
            Self::Eer => Ok(Some(::std::borrow::Cow::Borrowed("EER"))),
            Self::Cop => Ok(Some(::std::borrow::Cow::Borrowed("COP"))),
            Self::KWTon => Ok(Some(::std::borrow::Cow::Borrowed("kW/ton"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for CoolingEfficiencyUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for CoolingEfficiencyUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"SEER" => Ok(Self::Seer),
            b"EER" => Ok(Self::Eer),
            b"COP" => Ok(Self::Cop),
            b"kW/ton" => Ok(Self::KWTon),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for CoolingEfficiencyUnits {}
#[derive(Clone, Debug, PartialEq)]
pub enum CoolingSystemType {
    CentralAirConditioning,
    MiniSplit,
    RoomAirConditioner,
    EvaporativeCooler,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for CoolingSystemType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::CentralAirConditioning => Ok(Some(::std::borrow::Cow::Borrowed(
                "central air conditioning",
            ))),
            Self::MiniSplit => Ok(Some(::std::borrow::Cow::Borrowed("mini-split"))),
            Self::RoomAirConditioner => {
                Ok(Some(::std::borrow::Cow::Borrowed("room air conditioner")))
            }
            Self::EvaporativeCooler => Ok(Some(::std::borrow::Cow::Borrowed("evaporative cooler"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for CoolingSystemType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for CoolingSystemType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"central air conditioning" => Ok(Self::CentralAirConditioning),
            b"mini-split" => Ok(Self::MiniSplit),
            b"room air conditioner" => Ok(Self::RoomAirConditioner),
            b"evaporative cooler" => Ok(Self::EvaporativeCooler),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for CoolingSystemType {}
pub type Cost = ::core::primitive::f64;
pub type CreatedDateAndTime = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum DhwThirdPartyCertification {
    EnergyStar,
    CeeTier1,
    CeeTier2,
    CeeTier3,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DhwThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 3"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DhwThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DhwThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"CEE Tier 3" => Ok(Self::CeeTier3),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DhwThirdPartyCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum DeckType {
    Concrete,
    Metal,
    Wood,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DeckType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Concrete => Ok(Some(::std::borrow::Cow::Borrowed("concrete"))),
            Self::Metal => Ok(Some(::std::borrow::Cow::Borrowed("metal"))),
            Self::Wood => Ok(Some(::std::borrow::Cow::Borrowed("wood"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DeckType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DeckType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"concrete" => Ok(Self::Concrete),
            b"metal" => Ok(Self::Metal),
            b"wood" => Ok(Self::Wood),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DeckType {}
#[derive(Clone, Debug, PartialEq)]
pub enum DehumidifierLocation {
    LivingSpace,
    Basement,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DehumidifierLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::LivingSpace => Ok(Some(::std::borrow::Cow::Borrowed("living space"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DehumidifierLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DehumidifierLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"living space" => Ok(Self::LivingSpace),
            b"basement" => Ok(Self::Basement),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DehumidifierLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum DepressurizationFindingPoorCase {
    Pass,
    Fail,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DepressurizationFindingPoorCase {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Pass => Ok(Some(::std::borrow::Cow::Borrowed("pass"))),
            Self::Fail => Ok(Some(::std::borrow::Cow::Borrowed("fail"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DepressurizationFindingPoorCase {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DepressurizationFindingPoorCase {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"pass" => Ok(Self::Pass),
            b"fail" => Ok(Self::Fail),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DepressurizationFindingPoorCase {}
#[derive(Clone, Debug, PartialEq)]
pub enum DishwasherType {
    Uncategorized,
    BuiltInUnderCounter,
    Portable,
    CounterTop,
    SingleTank,
    Conveyor,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DishwasherType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Uncategorized => Ok(Some(::std::borrow::Cow::Borrowed("uncategorized"))),
            Self::BuiltInUnderCounter => {
                Ok(Some(::std::borrow::Cow::Borrowed("built-in under counter")))
            }
            Self::Portable => Ok(Some(::std::borrow::Cow::Borrowed("portable"))),
            Self::CounterTop => Ok(Some(::std::borrow::Cow::Borrowed("counter-top"))),
            Self::SingleTank => Ok(Some(::std::borrow::Cow::Borrowed("single tank"))),
            Self::Conveyor => Ok(Some(::std::borrow::Cow::Borrowed("conveyor"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DishwasherType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DishwasherType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"uncategorized" => Ok(Self::Uncategorized),
            b"built-in under counter" => Ok(Self::BuiltInUnderCounter),
            b"portable" => Ok(Self::Portable),
            b"counter-top" => Ok(Self::CounterTop),
            b"single tank" => Ok(Self::SingleTank),
            b"conveyor" => Ok(Self::Conveyor),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DishwasherType {}
pub type DispositionofExistingSystem = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum DistrictSteamType {
    _1Pipe,
    _2Pipe,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DistrictSteamType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_1Pipe => Ok(Some(::std::borrow::Cow::Borrowed("1-pipe"))),
            Self::_2Pipe => Ok(Some(::std::borrow::Cow::Borrowed("2-pipe"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DistrictSteamType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DistrictSteamType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"1-pipe" => Ok(Self::_1Pipe),
            b"2-pipe" => Ok(Self::_2Pipe),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DistrictSteamType {}
#[derive(Clone, Debug, PartialEq)]
pub enum DoorMaterial {
    SolidWood,
    HollowWood,
    UninsulatedMetal,
    InsulatedMetal,
    Glass,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DoorMaterial {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SolidWood => Ok(Some(::std::borrow::Cow::Borrowed("solid wood"))),
            Self::HollowWood => Ok(Some(::std::borrow::Cow::Borrowed("hollow wood"))),
            Self::UninsulatedMetal => Ok(Some(::std::borrow::Cow::Borrowed("uninsulated metal"))),
            Self::InsulatedMetal => Ok(Some(::std::borrow::Cow::Borrowed("insulated metal"))),
            Self::Glass => Ok(Some(::std::borrow::Cow::Borrowed("glass"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DoorMaterial {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DoorMaterial {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"solid wood" => Ok(Self::SolidWood),
            b"hollow wood" => Ok(Self::HollowWood),
            b"uninsulated metal" => Ok(Self::UninsulatedMetal),
            b"insulated metal" => Ok(Self::InsulatedMetal),
            b"glass" => Ok(Self::Glass),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DoorMaterial {}
#[derive(Clone, Debug, PartialEq)]
pub enum DoorThirdPartyCertifications {
    EnergyStar,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DoorThirdPartyCertifications {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DoorThirdPartyCertifications {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DoorThirdPartyCertifications {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DoorThirdPartyCertifications {}
#[derive(Clone, Debug, PartialEq)]
pub enum DoorType {
    Interior,
    Exterior,
    Storm,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DoorType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Interior => Ok(Some(::std::borrow::Cow::Borrowed("interior"))),
            Self::Exterior => Ok(Some(::std::borrow::Cow::Borrowed("exterior"))),
            Self::Storm => Ok(Some(::std::borrow::Cow::Borrowed("storm"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DoorType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DoorType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"interior" => Ok(Self::Interior),
            b"exterior" => Ok(Self::Exterior),
            b"storm" => Ok(Self::Storm),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DoorType {}
#[derive(Clone, Debug, PartialEq)]
pub enum DrainWaterHeatRecoveryFacilitiesConnected {
    One,
    All,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DrainWaterHeatRecoveryFacilitiesConnected {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::One => Ok(Some(::std::borrow::Cow::Borrowed("one"))),
            Self::All => Ok(Some(::std::borrow::Cow::Borrowed("all"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes
    for DrainWaterHeatRecoveryFacilitiesConnected
{
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DrainWaterHeatRecoveryFacilitiesConnected {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"one" => Ok(Self::One),
            b"all" => Ok(Self::All),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for DrainWaterHeatRecoveryFacilitiesConnected
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctLeakageTestMethod {
    DuctLeakageTester,
    BlowerDoorSubtract,
    PressurePan,
    VisualInspection,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctLeakageTestMethod {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::DuctLeakageTester => {
                Ok(Some(::std::borrow::Cow::Borrowed("duct leakage tester")))
            }
            Self::BlowerDoorSubtract => {
                Ok(Some(::std::borrow::Cow::Borrowed("blower door subtract")))
            }
            Self::PressurePan => Ok(Some(::std::borrow::Cow::Borrowed("pressure pan"))),
            Self::VisualInspection => Ok(Some(::std::borrow::Cow::Borrowed("visual inspection"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctLeakageTestMethod {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctLeakageTestMethod {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"duct leakage tester" => Ok(Self::DuctLeakageTester),
            b"blower door subtract" => Ok(Self::BlowerDoorSubtract),
            b"pressure pan" => Ok(Self::PressurePan),
            b"visual inspection" => Ok(Self::VisualInspection),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctLeakageTestMethod {}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctLeakageTestUnitofMeasure {
    Cfm50,
    Cfm25,
    CfmPerStd152,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctLeakageTestUnitofMeasure {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cfm50 => Ok(Some(::std::borrow::Cow::Borrowed("CFM50"))),
            Self::Cfm25 => Ok(Some(::std::borrow::Cow::Borrowed("CFM25"))),
            Self::CfmPerStd152 => Ok(Some(::std::borrow::Cow::Borrowed("CFM per Std 152"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctLeakageTestUnitofMeasure {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctLeakageTestUnitofMeasure {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"CFM50" => Ok(Self::Cfm50),
            b"CFM25" => Ok(Self::Cfm25),
            b"CFM per Std 152" => Ok(Self::CfmPerStd152),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctLeakageTestUnitofMeasure {}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctLeakageTotalOrToOutside {
    ToOutside,
    Total,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctLeakageTotalOrToOutside {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ToOutside => Ok(Some(::std::borrow::Cow::Borrowed("to outside"))),
            Self::Total => Ok(Some(::std::borrow::Cow::Borrowed("total"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctLeakageTotalOrToOutside {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctLeakageTotalOrToOutside {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"to outside" => Ok(Self::ToOutside),
            b"total" => Ok(Self::Total),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctLeakageTotalOrToOutside {}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctLocation {
    ConditionedSpace,
    UnconditionedSpace,
    UnconditionedBasement,
    UnventedCrawlspace,
    VentedCrawlspace,
    Crawlspace,
    UnconditionedAttic,
    InterstitialSpace,
    Garage,
    Outside,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ConditionedSpace => Ok(Some(::std::borrow::Cow::Borrowed("conditioned space"))),
            Self::UnconditionedSpace => {
                Ok(Some(::std::borrow::Cow::Borrowed("unconditioned space")))
            }
            Self::UnconditionedBasement => {
                Ok(Some(::std::borrow::Cow::Borrowed("unconditioned basement")))
            }
            Self::UnventedCrawlspace => {
                Ok(Some(::std::borrow::Cow::Borrowed("unvented crawlspace")))
            }
            Self::VentedCrawlspace => Ok(Some(::std::borrow::Cow::Borrowed("vented crawlspace"))),
            Self::Crawlspace => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace"))),
            Self::UnconditionedAttic => {
                Ok(Some(::std::borrow::Cow::Borrowed("unconditioned attic")))
            }
            Self::InterstitialSpace => Ok(Some(::std::borrow::Cow::Borrowed("interstitial space"))),
            Self::Garage => Ok(Some(::std::borrow::Cow::Borrowed("garage"))),
            Self::Outside => Ok(Some(::std::borrow::Cow::Borrowed("outside"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"conditioned space" => Ok(Self::ConditionedSpace),
            b"unconditioned space" => Ok(Self::UnconditionedSpace),
            b"unconditioned basement" => Ok(Self::UnconditionedBasement),
            b"unvented crawlspace" => Ok(Self::UnventedCrawlspace),
            b"vented crawlspace" => Ok(Self::VentedCrawlspace),
            b"crawlspace" => Ok(Self::Crawlspace),
            b"unconditioned attic" => Ok(Self::UnconditionedAttic),
            b"interstitial space" => Ok(Self::InterstitialSpace),
            b"garage" => Ok(Self::Garage),
            b"outside" => Ok(Self::Outside),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctMaterial {
    DuctBoard,
    SheetMetal,
    Galvanized,
    Flexible,
    Fiberboard,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctMaterial {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::DuctBoard => Ok(Some(::std::borrow::Cow::Borrowed("duct board"))),
            Self::SheetMetal => Ok(Some(::std::borrow::Cow::Borrowed("sheet metal"))),
            Self::Galvanized => Ok(Some(::std::borrow::Cow::Borrowed("galvanized"))),
            Self::Flexible => Ok(Some(::std::borrow::Cow::Borrowed("flexible"))),
            Self::Fiberboard => Ok(Some(::std::borrow::Cow::Borrowed("fiberboard"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctMaterial {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctMaterial {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"duct board" => Ok(Self::DuctBoard),
            b"sheet metal" => Ok(Self::SheetMetal),
            b"galvanized" => Ok(Self::Galvanized),
            b"flexible" => Ok(Self::Flexible),
            b"fiberboard" => Ok(Self::Fiberboard),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctMaterial {}
#[derive(Clone, Debug, PartialEq)]
pub enum DuctType {
    Supply,
    Return,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for DuctType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Supply => Ok(Some(::std::borrow::Cow::Borrowed("supply"))),
            Self::Return => Ok(Some(::std::borrow::Cow::Borrowed("return"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for DuctType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for DuctType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"supply" => Ok(Self::Supply),
            b"return" => Ok(Self::Return),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for DuctType {}
#[derive(Clone, Debug, PartialEq)]
pub struct Eer(pub ::core::primitive::f64);
impl Eer {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Eer> for ::core::primitive::f64 {
    fn from(value: Eer) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Eer {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Eer {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Eer {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Eer {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Eer {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Eer {}
pub type EarthquakeZone = ::core::primitive::bool;
#[derive(Clone, Debug, PartialEq)]
pub enum EducationLevels {
    NoHighSchool,
    SomeHighSchool,
    HighSchoolGraduate,
    SomeCollege,
    VocationalTechnicalAssociatesDegree,
    BachelorSDegree,
    SomePostGraduate,
    MasterSDegree,
    ProfessionalDegree,
    DoctoralDegree,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EducationLevels {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::NoHighSchool => Ok(Some(::std::borrow::Cow::Borrowed("no high school"))),
            Self::SomeHighSchool => Ok(Some(::std::borrow::Cow::Borrowed("some high school"))),
            Self::HighSchoolGraduate => {
                Ok(Some(::std::borrow::Cow::Borrowed("high school graduate")))
            }
            Self::SomeCollege => Ok(Some(::std::borrow::Cow::Borrowed("some college"))),
            Self::VocationalTechnicalAssociatesDegree => Ok(Some(::std::borrow::Cow::Borrowed(
                "vocational/technical/associates degree",
            ))),
            Self::BachelorSDegree => Ok(Some(::std::borrow::Cow::Borrowed("bachelor's degree"))),
            Self::SomePostGraduate => Ok(Some(::std::borrow::Cow::Borrowed("some post graduate"))),
            Self::MasterSDegree => Ok(Some(::std::borrow::Cow::Borrowed("master's degree"))),
            Self::ProfessionalDegree => {
                Ok(Some(::std::borrow::Cow::Borrowed("professional degree")))
            }
            Self::DoctoralDegree => Ok(Some(::std::borrow::Cow::Borrowed("doctoral degree"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EducationLevels {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EducationLevels {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"no high school" => Ok(Self::NoHighSchool),
            b"some high school" => Ok(Self::SomeHighSchool),
            b"high school graduate" => Ok(Self::HighSchoolGraduate),
            b"some college" => Ok(Self::SomeCollege),
            b"vocational/technical/associates degree" => {
                Ok(Self::VocationalTechnicalAssociatesDegree)
            }
            b"bachelor's degree" => Ok(Self::BachelorSDegree),
            b"some post graduate" => Ok(Self::SomePostGraduate),
            b"master's degree" => Ok(Self::MasterSDegree),
            b"professional degree" => Ok(Self::ProfessionalDegree),
            b"doctoral degree" => Ok(Self::DoctoralDegree),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EducationLevels {}
#[derive(Clone, Debug, PartialEq)]
pub struct Efficiency(pub ::core::primitive::f64);
impl Efficiency {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Efficiency> for ::core::primitive::f64 {
    fn from(value: Efficiency) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Efficiency {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Efficiency {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Efficiency {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Efficiency {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Efficiency {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Efficiency {}
#[derive(Clone, Debug, PartialEq)]
pub enum ElectricDistributionType {
    Baseboard,
    RadiantFloor,
    RadiantCeiling,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ElectricDistributionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Baseboard => Ok(Some(::std::borrow::Cow::Borrowed("baseboard"))),
            Self::RadiantFloor => Ok(Some(::std::borrow::Cow::Borrowed("radiant floor"))),
            Self::RadiantCeiling => Ok(Some(::std::borrow::Cow::Borrowed("radiant ceiling"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ElectricDistributionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ElectricDistributionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"baseboard" => Ok(Self::Baseboard),
            b"radiant floor" => Ok(Self::RadiantFloor),
            b"radiant ceiling" => Ok(Self::RadiantCeiling),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ElectricDistributionType {}
pub type EmailAddress = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum EmailTypeCode {
    Personal,
    Work,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EmailTypeCode {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Personal => Ok(Some(::std::borrow::Cow::Borrowed("personal"))),
            Self::Work => Ok(Some(::std::borrow::Cow::Borrowed("work"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EmailTypeCode {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EmailTypeCode {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"personal" => Ok(Self::Personal),
            b"work" => Ok(Self::Work),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EmailTypeCode {}
#[derive(Clone, Debug, PartialEq)]
pub enum EmissionType {
    Co2,
    Methane,
    N2O,
    Co2Equivalent,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EmissionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Co2 => Ok(Some(::std::borrow::Cow::Borrowed("CO2"))),
            Self::Methane => Ok(Some(::std::borrow::Cow::Borrowed("methane"))),
            Self::N2O => Ok(Some(::std::borrow::Cow::Borrowed("N2O"))),
            Self::Co2Equivalent => Ok(Some(::std::borrow::Cow::Borrowed("CO2 equivalent"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EmissionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EmissionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"CO2" => Ok(Self::Co2),
            b"methane" => Ok(Self::Methane),
            b"N2O" => Ok(Self::N2O),
            b"CO2 equivalent" => Ok(Self::Co2Equivalent),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EmissionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum EmissionUnits {
    Kg,
    Ton,
    MetricTon,
    Pound,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EmissionUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Kg => Ok(Some(::std::borrow::Cow::Borrowed("kg"))),
            Self::Ton => Ok(Some(::std::borrow::Cow::Borrowed("ton"))),
            Self::MetricTon => Ok(Some(::std::borrow::Cow::Borrowed("metric ton"))),
            Self::Pound => Ok(Some(::std::borrow::Cow::Borrowed("pound"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EmissionUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EmissionUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kg" => Ok(Self::Kg),
            b"ton" => Ok(Self::Ton),
            b"metric ton" => Ok(Self::MetricTon),
            b"pound" => Ok(Self::Pound),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EmissionUnits {}
#[derive(Clone, Debug, PartialEq)]
pub struct Emittance(pub ::core::primitive::f64);
impl Emittance {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "1",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Emittance> for ::core::primitive::f64 {
    fn from(value: Emittance) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Emittance {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Emittance {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Emittance {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Emittance {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Emittance {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Emittance {}
#[derive(Clone, Debug, PartialEq)]
pub struct EnergyFactor(pub ::core::primitive::f64);
impl EnergyFactor {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        if *value > 5f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "5",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<EnergyFactor> for ::core::primitive::f64 {
    fn from(value: EnergyFactor) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for EnergyFactor {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for EnergyFactor {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EnergyFactor {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EnergyFactor {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EnergyFactor {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EnergyFactor {}
pub type EstimatedLife = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
    Audit,
    ProposedWorkscope,
    ApprovedWorkscope,
    ConstructionPeriodTestingDailyTestOut,
    JobCompletionTestingFinalInspection,
    QualityAssuranceMonitoring,
    Preconstruction,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EventType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Audit => Ok(Some(::std::borrow::Cow::Borrowed("audit"))),
            Self::ProposedWorkscope => Ok(Some(::std::borrow::Cow::Borrowed("proposed workscope"))),
            Self::ApprovedWorkscope => Ok(Some(::std::borrow::Cow::Borrowed("approved workscope"))),
            Self::ConstructionPeriodTestingDailyTestOut => Ok(Some(::std::borrow::Cow::Borrowed(
                "construction-period testing/daily test out",
            ))),
            Self::JobCompletionTestingFinalInspection => Ok(Some(::std::borrow::Cow::Borrowed(
                "job completion testing/final inspection",
            ))),
            Self::QualityAssuranceMonitoring => Ok(Some(::std::borrow::Cow::Borrowed(
                "quality assurance/monitoring",
            ))),
            Self::Preconstruction => Ok(Some(::std::borrow::Cow::Borrowed("preconstruction"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EventType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EventType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"audit" => Ok(Self::Audit),
            b"proposed workscope" => Ok(Self::ProposedWorkscope),
            b"approved workscope" => Ok(Self::ApprovedWorkscope),
            b"construction-period testing/daily test out" => {
                Ok(Self::ConstructionPeriodTestingDailyTestOut)
            }
            b"job completion testing/final inspection" => {
                Ok(Self::JobCompletionTestingFinalInspection)
            }
            b"quality assurance/monitoring" => Ok(Self::QualityAssuranceMonitoring),
            b"preconstruction" => Ok(Self::Preconstruction),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EventType {}
#[derive(Clone, Debug, PartialEq)]
pub enum ExteriorLocationsWaterIntrusionorDamage {
    Roof,
    InteriorCeiling,
    Foundation,
    Basement,
    Crawlspace,
    Walls,
    AroundWindows,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ExteriorLocationsWaterIntrusionorDamage {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Roof => Ok(Some(::std::borrow::Cow::Borrowed("roof"))),
            Self::InteriorCeiling => Ok(Some(::std::borrow::Cow::Borrowed("interior ceiling"))),
            Self::Foundation => Ok(Some(::std::borrow::Cow::Borrowed("foundation"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Crawlspace => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace"))),
            Self::Walls => Ok(Some(::std::borrow::Cow::Borrowed("walls"))),
            Self::AroundWindows => Ok(Some(::std::borrow::Cow::Borrowed("around windows"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ExteriorLocationsWaterIntrusionorDamage {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ExteriorLocationsWaterIntrusionorDamage {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"roof" => Ok(Self::Roof),
            b"interior ceiling" => Ok(Self::InteriorCeiling),
            b"foundation" => Ok(Self::Foundation),
            b"basement" => Ok(Self::Basement),
            b"crawlspace" => Ok(Self::Crawlspace),
            b"walls" => Ok(Self::Walls),
            b"around windows" => Ok(Self::AroundWindows),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for ExteriorLocationsWaterIntrusionorDamage
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum ExteriorShading {
    ExternalOverhangs,
    Awnings,
    SolarScreens,
    SolarFilm,
    DeciduousTree,
    EvergreenTree,
    Building,
    Other,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ExteriorShading {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ExternalOverhangs => Ok(Some(::std::borrow::Cow::Borrowed("external overhangs"))),
            Self::Awnings => Ok(Some(::std::borrow::Cow::Borrowed("awnings"))),
            Self::SolarScreens => Ok(Some(::std::borrow::Cow::Borrowed("solar screens"))),
            Self::SolarFilm => Ok(Some(::std::borrow::Cow::Borrowed("solar film"))),
            Self::DeciduousTree => Ok(Some(::std::borrow::Cow::Borrowed("deciduous tree"))),
            Self::EvergreenTree => Ok(Some(::std::borrow::Cow::Borrowed("evergreen tree"))),
            Self::Building => Ok(Some(::std::borrow::Cow::Borrowed("building"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ExteriorShading {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ExteriorShading {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"external overhangs" => Ok(Self::ExternalOverhangs),
            b"awnings" => Ok(Self::Awnings),
            b"solar screens" => Ok(Self::SolarScreens),
            b"solar film" => Ok(Self::SolarFilm),
            b"deciduous tree" => Ok(Self::DeciduousTree),
            b"evergreen tree" => Ok(Self::EvergreenTree),
            b"building" => Ok(Self::Building),
            b"other" => Ok(Self::Other),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ExteriorShading {}
pub type FanPressure = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum FanRingUsed {
    Open,
    A,
    B,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FanRingUsed {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Open => Ok(Some(::std::borrow::Cow::Borrowed("open"))),
            Self::A => Ok(Some(::std::borrow::Cow::Borrowed("A"))),
            Self::B => Ok(Some(::std::borrow::Cow::Borrowed("B"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FanRingUsed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FanRingUsed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"open" => Ok(Self::Open),
            b"A" => Ok(Self::A),
            b"B" => Ok(Self::B),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FanRingUsed {}
#[derive(Clone, Debug, PartialEq)]
pub enum FloorCovering {
    Carpet,
    Tile,
    Hardwood,
    Vinyl,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FloorCovering {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Carpet => Ok(Some(::std::borrow::Cow::Borrowed("carpet"))),
            Self::Tile => Ok(Some(::std::borrow::Cow::Borrowed("tile"))),
            Self::Hardwood => Ok(Some(::std::borrow::Cow::Borrowed("hardwood"))),
            Self::Vinyl => Ok(Some(::std::borrow::Cow::Borrowed("vinyl"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FloorCovering {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FloorCovering {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"carpet" => Ok(Self::Carpet),
            b"tile" => Ok(Self::Tile),
            b"hardwood" => Ok(Self::Hardwood),
            b"vinyl" => Ok(Self::Vinyl),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FloorCovering {}
#[derive(Clone, Debug, PartialEq)]
pub struct FlowRate(pub ::core::primitive::f64);
impl FlowRate {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<FlowRate> for ::core::primitive::f64 {
    fn from(value: FlowRate) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for FlowRate {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for FlowRate {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FlowRate {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FlowRate {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FlowRate {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FlowRate {}
#[derive(Clone, Debug, PartialEq)]
pub enum FlueCondition {
    Pass,
    Fail,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FlueCondition {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Pass => Ok(Some(::std::borrow::Cow::Borrowed("pass"))),
            Self::Fail => Ok(Some(::std::borrow::Cow::Borrowed("fail"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FlueCondition {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FlueCondition {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"pass" => Ok(Self::Pass),
            b"fail" => Ok(Self::Fail),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FlueCondition {}
#[derive(Clone, Debug, PartialEq)]
pub enum FluorescentBallastType {
    Electronic,
    Magnetic,
    InstantStart,
    RapidStart,
    ProgrammedStart,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FluorescentBallastType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Electronic => Ok(Some(::std::borrow::Cow::Borrowed("electronic"))),
            Self::Magnetic => Ok(Some(::std::borrow::Cow::Borrowed("magnetic"))),
            Self::InstantStart => Ok(Some(::std::borrow::Cow::Borrowed("instant start"))),
            Self::RapidStart => Ok(Some(::std::borrow::Cow::Borrowed("rapid start"))),
            Self::ProgrammedStart => Ok(Some(::std::borrow::Cow::Borrowed("programmed start"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FluorescentBallastType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FluorescentBallastType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"electronic" => Ok(Self::Electronic),
            b"magnetic" => Ok(Self::Magnetic),
            b"instant start" => Ok(Self::InstantStart),
            b"rapid start" => Ok(Self::RapidStart),
            b"programmed start" => Ok(Self::ProgrammedStart),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FluorescentBallastType {}
#[derive(Clone, Debug, PartialEq)]
pub enum FluorescentTubeType {
    T2,
    T4,
    T5,
    T8,
    SuperT8,
    T9,
    T10,
    T12,
    T17,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FluorescentTubeType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::T2 => Ok(Some(::std::borrow::Cow::Borrowed("T2"))),
            Self::T4 => Ok(Some(::std::borrow::Cow::Borrowed("T4"))),
            Self::T5 => Ok(Some(::std::borrow::Cow::Borrowed("T5"))),
            Self::T8 => Ok(Some(::std::borrow::Cow::Borrowed("T8"))),
            Self::SuperT8 => Ok(Some(::std::borrow::Cow::Borrowed("super T8"))),
            Self::T9 => Ok(Some(::std::borrow::Cow::Borrowed("T9"))),
            Self::T10 => Ok(Some(::std::borrow::Cow::Borrowed("T10"))),
            Self::T12 => Ok(Some(::std::borrow::Cow::Borrowed("T12"))),
            Self::T17 => Ok(Some(::std::borrow::Cow::Borrowed("T17"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FluorescentTubeType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FluorescentTubeType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"T2" => Ok(Self::T2),
            b"T4" => Ok(Self::T4),
            b"T5" => Ok(Self::T5),
            b"T8" => Ok(Self::T8),
            b"super T8" => Ok(Self::SuperT8),
            b"T9" => Ok(Self::T9),
            b"T10" => Ok(Self::T10),
            b"T12" => Ok(Self::T12),
            b"T17" => Ok(Self::T17),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FluorescentTubeType {}
#[derive(Clone, Debug, PartialEq)]
pub enum FootprintShape {
    Rectangular,
    Square,
    Circular,
    LShaped,
    UShaped,
    IShaped,
    VShaped,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FootprintShape {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Rectangular => Ok(Some(::std::borrow::Cow::Borrowed("rectangular"))),
            Self::Square => Ok(Some(::std::borrow::Cow::Borrowed("square"))),
            Self::Circular => Ok(Some(::std::borrow::Cow::Borrowed("circular"))),
            Self::LShaped => Ok(Some(::std::borrow::Cow::Borrowed("L-shaped"))),
            Self::UShaped => Ok(Some(::std::borrow::Cow::Borrowed("U-shaped"))),
            Self::IShaped => Ok(Some(::std::borrow::Cow::Borrowed("I-shaped"))),
            Self::VShaped => Ok(Some(::std::borrow::Cow::Borrowed("V-shaped"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FootprintShape {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FootprintShape {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"rectangular" => Ok(Self::Rectangular),
            b"square" => Ok(Self::Square),
            b"circular" => Ok(Self::Circular),
            b"L-shaped" => Ok(Self::LShaped),
            b"U-shaped" => Ok(Self::UShaped),
            b"I-shaped" => Ok(Self::IShaped),
            b"V-shaped" => Ok(Self::VShaped),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FootprintShape {}
#[derive(Clone, Debug, PartialEq)]
pub enum FoundationThermalBoundary {
    FrameFloor,
    FoundationWall,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FoundationThermalBoundary {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::FrameFloor => Ok(Some(::std::borrow::Cow::Borrowed("frame floor"))),
            Self::FoundationWall => Ok(Some(::std::borrow::Cow::Borrowed("foundation wall"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FoundationThermalBoundary {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FoundationThermalBoundary {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"frame floor" => Ok(Self::FrameFloor),
            b"foundation wall" => Ok(Self::FoundationWall),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FoundationThermalBoundary {}
#[derive(Clone, Debug, PartialEq)]
pub enum FoundationWallType {
    SolidConcrete,
    ConcreteBlock,
    ConcreteBlockFoamCore,
    ConcreteBlockVermiculiteCore,
    DoubleBrick,
    Wood,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FoundationWallType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SolidConcrete => Ok(Some(::std::borrow::Cow::Borrowed("solid concrete"))),
            Self::ConcreteBlock => Ok(Some(::std::borrow::Cow::Borrowed("concrete block"))),
            Self::ConcreteBlockFoamCore => Ok(Some(::std::borrow::Cow::Borrowed(
                "concrete block foam core",
            ))),
            Self::ConcreteBlockVermiculiteCore => Ok(Some(::std::borrow::Cow::Borrowed(
                "concrete block vermiculite core",
            ))),
            Self::DoubleBrick => Ok(Some(::std::borrow::Cow::Borrowed("double brick"))),
            Self::Wood => Ok(Some(::std::borrow::Cow::Borrowed("wood"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FoundationWallType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FoundationWallType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"solid concrete" => Ok(Self::SolidConcrete),
            b"concrete block" => Ok(Self::ConcreteBlock),
            b"concrete block foam core" => Ok(Self::ConcreteBlockFoamCore),
            b"concrete block vermiculite core" => Ok(Self::ConcreteBlockVermiculiteCore),
            b"double brick" => Ok(Self::DoubleBrick),
            b"wood" => Ok(Self::Wood),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FoundationWallType {}
#[derive(Clone, Debug, PartialEq)]
pub struct Fraction(pub ::core::primitive::f64);
impl Fraction {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "1",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Fraction> for ::core::primitive::f64 {
    fn from(value: Fraction) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Fraction {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Fraction {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Fraction {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Fraction {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Fraction {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Fraction {}
#[derive(Clone, Debug, PartialEq)]
pub struct FractionGreaterThanOne(pub ::core::primitive::f64);
impl FractionGreaterThanOne {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<FractionGreaterThanOne> for ::core::primitive::f64 {
    fn from(value: FractionGreaterThanOne) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for FractionGreaterThanOne {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for FractionGreaterThanOne {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FractionGreaterThanOne {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FractionGreaterThanOne {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FractionGreaterThanOne {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FractionGreaterThanOne {}
#[derive(Clone, Debug, PartialEq)]
pub enum FreezerStyle {
    Uncategorized,
    ManualDefrost,
    FrostFree,
    WalkIn,
    Case,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FreezerStyle {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Uncategorized => Ok(Some(::std::borrow::Cow::Borrowed("uncategorized"))),
            Self::ManualDefrost => Ok(Some(::std::borrow::Cow::Borrowed("manual defrost"))),
            Self::FrostFree => Ok(Some(::std::borrow::Cow::Borrowed("frost free"))),
            Self::WalkIn => Ok(Some(::std::borrow::Cow::Borrowed("walk-in"))),
            Self::Case => Ok(Some(::std::borrow::Cow::Borrowed("case"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FreezerStyle {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FreezerStyle {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"uncategorized" => Ok(Self::Uncategorized),
            b"manual defrost" => Ok(Self::ManualDefrost),
            b"frost free" => Ok(Self::FrostFree),
            b"walk-in" => Ok(Self::WalkIn),
            b"case" => Ok(Self::Case),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FreezerStyle {}
#[derive(Clone, Debug, PartialEq)]
pub enum FuelInterruptibility {
    Interruptible,
    Firm,
    Na,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FuelInterruptibility {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Interruptible => Ok(Some(::std::borrow::Cow::Borrowed("interruptible"))),
            Self::Firm => Ok(Some(::std::borrow::Cow::Borrowed("firm"))),
            Self::Na => Ok(Some(::std::borrow::Cow::Borrowed("na"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FuelInterruptibility {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FuelInterruptibility {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"interruptible" => Ok(Self::Interruptible),
            b"firm" => Ok(Self::Firm),
            b"na" => Ok(Self::Na),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FuelInterruptibility {}
#[derive(Clone, Debug, PartialEq)]
pub enum FuelType {
    Electricity,
    RenewableElectricity,
    NaturalGas,
    RenewableNaturalGas,
    FuelOil,
    FuelOil1,
    FuelOil2,
    FuelOil4,
    FuelOil56,
    DistrictSteam,
    DistrictHotWater,
    DistrictChilledWater,
    SolarHotWater,
    Propane,
    Kerosene,
    Diesel,
    AnthraciteCoal,
    BituminousCoal,
    Coke,
    Wood,
    WoodPellets,
    Combination,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for FuelType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Electricity => Ok(Some(::std::borrow::Cow::Borrowed("electricity"))),
            Self::RenewableElectricity => {
                Ok(Some(::std::borrow::Cow::Borrowed("renewable electricity")))
            }
            Self::NaturalGas => Ok(Some(::std::borrow::Cow::Borrowed("natural gas"))),
            Self::RenewableNaturalGas => {
                Ok(Some(::std::borrow::Cow::Borrowed("renewable natural gas")))
            }
            Self::FuelOil => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil"))),
            Self::FuelOil1 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 1"))),
            Self::FuelOil2 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 2"))),
            Self::FuelOil4 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 4"))),
            Self::FuelOil56 => Ok(Some(::std::borrow::Cow::Borrowed("fuel oil 5/6"))),
            Self::DistrictSteam => Ok(Some(::std::borrow::Cow::Borrowed("district steam"))),
            Self::DistrictHotWater => Ok(Some(::std::borrow::Cow::Borrowed("district hot water"))),
            Self::DistrictChilledWater => {
                Ok(Some(::std::borrow::Cow::Borrowed("district chilled water")))
            }
            Self::SolarHotWater => Ok(Some(::std::borrow::Cow::Borrowed("solar hot water"))),
            Self::Propane => Ok(Some(::std::borrow::Cow::Borrowed("propane"))),
            Self::Kerosene => Ok(Some(::std::borrow::Cow::Borrowed("kerosene"))),
            Self::Diesel => Ok(Some(::std::borrow::Cow::Borrowed("diesel"))),
            Self::AnthraciteCoal => Ok(Some(::std::borrow::Cow::Borrowed("anthracite coal"))),
            Self::BituminousCoal => Ok(Some(::std::borrow::Cow::Borrowed("bituminous coal"))),
            Self::Coke => Ok(Some(::std::borrow::Cow::Borrowed("coke"))),
            Self::Wood => Ok(Some(::std::borrow::Cow::Borrowed("wood"))),
            Self::WoodPellets => Ok(Some(::std::borrow::Cow::Borrowed("wood pellets"))),
            Self::Combination => Ok(Some(::std::borrow::Cow::Borrowed("combination"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for FuelType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for FuelType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"electricity" => Ok(Self::Electricity),
            b"renewable electricity" => Ok(Self::RenewableElectricity),
            b"natural gas" => Ok(Self::NaturalGas),
            b"renewable natural gas" => Ok(Self::RenewableNaturalGas),
            b"fuel oil" => Ok(Self::FuelOil),
            b"fuel oil 1" => Ok(Self::FuelOil1),
            b"fuel oil 2" => Ok(Self::FuelOil2),
            b"fuel oil 4" => Ok(Self::FuelOil4),
            b"fuel oil 5/6" => Ok(Self::FuelOil56),
            b"district steam" => Ok(Self::DistrictSteam),
            b"district hot water" => Ok(Self::DistrictHotWater),
            b"district chilled water" => Ok(Self::DistrictChilledWater),
            b"solar hot water" => Ok(Self::SolarHotWater),
            b"propane" => Ok(Self::Propane),
            b"kerosene" => Ok(Self::Kerosene),
            b"diesel" => Ok(Self::Diesel),
            b"anthracite coal" => Ok(Self::AnthraciteCoal),
            b"bituminous coal" => Ok(Self::BituminousCoal),
            b"coke" => Ok(Self::Coke),
            b"wood" => Ok(Self::Wood),
            b"wood pellets" => Ok(Self::WoodPellets),
            b"combination" => Ok(Self::Combination),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for FuelType {}
pub type FundingSourceCode = ::std::string::String;
pub type FundingSourceName = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum GarageLocation {
    Basement,
    FirstFloor,
    Detached,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GarageLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::FirstFloor => Ok(Some(::std::borrow::Cow::Borrowed("first floor"))),
            Self::Detached => Ok(Some(::std::borrow::Cow::Borrowed("detached"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GarageLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GarageLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"basement" => Ok(Self::Basement),
            b"first floor" => Ok(Self::FirstFloor),
            b"detached" => Ok(Self::Detached),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GarageLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum GasFill {
    Air,
    Argon,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GasFill {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Air => Ok(Some(::std::borrow::Cow::Borrowed("air"))),
            Self::Argon => Ok(Some(::std::borrow::Cow::Borrowed("argon"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GasFill {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GasFill {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"air" => Ok(Self::Air),
            b"argon" => Ok(Self::Argon),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GasFill {}
#[derive(Clone, Debug, PartialEq)]
pub enum GeothermalLoop {
    Open,
    Closed,
    DirectExpansion,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GeothermalLoop {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Open => Ok(Some(::std::borrow::Cow::Borrowed("open"))),
            Self::Closed => Ok(Some(::std::borrow::Cow::Borrowed("closed"))),
            Self::DirectExpansion => Ok(Some(::std::borrow::Cow::Borrowed("direct expansion"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GeothermalLoop {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GeothermalLoop {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"open" => Ok(Self::Open),
            b"closed" => Ok(Self::Closed),
            b"direct expansion" => Ok(Self::DirectExpansion),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GeothermalLoop {}
#[derive(Clone, Debug, PartialEq)]
pub enum GlassLayers {
    SinglePane,
    DoublePane,
    TriplePane,
    MultiLayered,
    SinglePanedWithStorms,
    SinglePanedWithLowEStorms,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GlassLayers {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SinglePane => Ok(Some(::std::borrow::Cow::Borrowed("single-pane"))),
            Self::DoublePane => Ok(Some(::std::borrow::Cow::Borrowed("double-pane"))),
            Self::TriplePane => Ok(Some(::std::borrow::Cow::Borrowed("triple-pane"))),
            Self::MultiLayered => Ok(Some(::std::borrow::Cow::Borrowed("multi-layered"))),
            Self::SinglePanedWithStorms => Ok(Some(::std::borrow::Cow::Borrowed(
                "single-paned with storms",
            ))),
            Self::SinglePanedWithLowEStorms => Ok(Some(::std::borrow::Cow::Borrowed(
                "single-paned with low-e storms",
            ))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GlassLayers {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GlassLayers {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"single-pane" => Ok(Self::SinglePane),
            b"double-pane" => Ok(Self::DoublePane),
            b"triple-pane" => Ok(Self::TriplePane),
            b"multi-layered" => Ok(Self::MultiLayered),
            b"single-paned with storms" => Ok(Self::SinglePanedWithStorms),
            b"single-paned with low-e storms" => Ok(Self::SinglePanedWithLowEStorms),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GlassLayers {}
#[derive(Clone, Debug, PartialEq)]
pub enum GlassType {
    LowE,
    Tinted,
    Reflective,
    TintedReflective,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GlassType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::LowE => Ok(Some(::std::borrow::Cow::Borrowed("low-e"))),
            Self::Tinted => Ok(Some(::std::borrow::Cow::Borrowed("tinted"))),
            Self::Reflective => Ok(Some(::std::borrow::Cow::Borrowed("reflective"))),
            Self::TintedReflective => Ok(Some(::std::borrow::Cow::Borrowed("tinted/reflective"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GlassType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GlassType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"low-e" => Ok(Self::LowE),
            b"tinted" => Ok(Self::Tinted),
            b"reflective" => Ok(Self::Reflective),
            b"tinted/reflective" => Ok(Self::TintedReflective),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GlassType {}
#[derive(Clone, Debug, PartialEq)]
pub enum GrossOrNet {
    Gross,
    Net,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for GrossOrNet {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Gross => Ok(Some(::std::borrow::Cow::Borrowed("gross"))),
            Self::Net => Ok(Some(::std::borrow::Cow::Borrowed("net"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for GrossOrNet {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for GrossOrNet {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"gross" => Ok(Self::Gross),
            b"net" => Ok(Self::Net),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for GrossOrNet {}
#[derive(Clone, Debug, PartialEq)]
pub struct Hspf(pub ::core::primitive::f64);
impl Hspf {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Hspf> for ::core::primitive::f64 {
    fn from(value: Hspf) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Hspf {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Hspf {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Hspf {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Hspf {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Hspf {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Hspf {}
#[derive(Clone, Debug, PartialEq)]
pub enum HvacInstallationStandard {
    Acca5QiHvac,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HvacInstallationStandard {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Acca5QiHvac => Ok(Some(::std::borrow::Cow::Borrowed("ACCA 5 QI HVAC"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HvacInstallationStandard {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HvacInstallationStandard {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"ACCA 5 QI HVAC" => Ok(Self::Acca5QiHvac),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HvacInstallationStandard {}
#[derive(Clone, Debug, PartialEq)]
pub enum HvacMaintenanceSchedule {
    None,
    YesUnspecified,
    AsNeeded,
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    SemiQuarterly,
    Quarterly,
    SemiAnnually,
    Annually,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HvacMaintenanceSchedule {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
            Self::YesUnspecified => Ok(Some(::std::borrow::Cow::Borrowed("yes - unspecified"))),
            Self::AsNeeded => Ok(Some(::std::borrow::Cow::Borrowed("as needed"))),
            Self::Daily => Ok(Some(::std::borrow::Cow::Borrowed("daily"))),
            Self::Weekly => Ok(Some(::std::borrow::Cow::Borrowed("weekly"))),
            Self::BiWeekly => Ok(Some(::std::borrow::Cow::Borrowed("bi-weekly"))),
            Self::Monthly => Ok(Some(::std::borrow::Cow::Borrowed("monthly"))),
            Self::SemiQuarterly => Ok(Some(::std::borrow::Cow::Borrowed("semi-quarterly"))),
            Self::Quarterly => Ok(Some(::std::borrow::Cow::Borrowed("quarterly"))),
            Self::SemiAnnually => Ok(Some(::std::borrow::Cow::Borrowed("semi-annually"))),
            Self::Annually => Ok(Some(::std::borrow::Cow::Borrowed("annually"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HvacMaintenanceSchedule {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HvacMaintenanceSchedule {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"none" => Ok(Self::None),
            b"yes - unspecified" => Ok(Self::YesUnspecified),
            b"as needed" => Ok(Self::AsNeeded),
            b"daily" => Ok(Self::Daily),
            b"weekly" => Ok(Self::Weekly),
            b"bi-weekly" => Ok(Self::BiWeekly),
            b"monthly" => Ok(Self::Monthly),
            b"semi-quarterly" => Ok(Self::SemiQuarterly),
            b"quarterly" => Ok(Self::Quarterly),
            b"semi-annually" => Ok(Self::SemiAnnually),
            b"annually" => Ok(Self::Annually),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HvacMaintenanceSchedule {}
#[derive(Clone, Debug, PartialEq)]
pub enum HvacSizingCalcs {
    ManualJ,
    ManualJAndManualD,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HvacSizingCalcs {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ManualJ => Ok(Some(::std::borrow::Cow::Borrowed("manual j"))),
            Self::ManualJAndManualD => {
                Ok(Some(::std::borrow::Cow::Borrowed("manual j and manual d")))
            }
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HvacSizingCalcs {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HvacSizingCalcs {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"manual j" => Ok(Self::ManualJ),
            b"manual j and manual d" => Ok(Self::ManualJAndManualD),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HvacSizingCalcs {}
#[derive(Clone, Debug, PartialEq)]
pub enum HvacThirdPartyCertification {
    EnergyStar,
    EnergyStarMostEfficient,
    CeeTier1,
    CeeTier2,
    CeeTier3,
    NeepColdClimateAirSourceHeatPumpSpecification,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HvacThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::EnergyStarMostEfficient => Ok(Some(::std::borrow::Cow::Borrowed(
                "Energy Star Most Efficient",
            ))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 3"))),
            Self::NeepColdClimateAirSourceHeatPumpSpecification => {
                Ok(Some(::std::borrow::Cow::Borrowed(
                    "NEEP Cold-Climate Air-Source Heat Pump Specification",
                )))
            }
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HvacThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HvacThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"Energy Star Most Efficient" => Ok(Self::EnergyStarMostEfficient),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"CEE Tier 3" => Ok(Self::CeeTier3),
            b"NEEP Cold-Climate Air-Source Heat Pump Specification" => {
                Ok(Self::NeepColdClimateAirSourceHeatPumpSpecification)
            }
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HvacThirdPartyCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum HeatPumpType {
    WaterToAir,
    WaterToWater,
    AirToAir,
    MiniSplit,
    GroundToAir,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HeatPumpType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::WaterToAir => Ok(Some(::std::borrow::Cow::Borrowed("water-to-air"))),
            Self::WaterToWater => Ok(Some(::std::borrow::Cow::Borrowed("water-to-water"))),
            Self::AirToAir => Ok(Some(::std::borrow::Cow::Borrowed("air-to-air"))),
            Self::MiniSplit => Ok(Some(::std::borrow::Cow::Borrowed("mini-split"))),
            Self::GroundToAir => Ok(Some(::std::borrow::Cow::Borrowed("ground-to-air"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HeatPumpType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HeatPumpType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"water-to-air" => Ok(Self::WaterToAir),
            b"water-to-water" => Ok(Self::WaterToWater),
            b"air-to-air" => Ok(Self::AirToAir),
            b"mini-split" => Ok(Self::MiniSplit),
            b"ground-to-air" => Ok(Self::GroundToAir),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HeatPumpType {}
#[derive(Clone, Debug, PartialEq)]
pub enum HeatingEfficiencyUnits {
    Hspf,
    Cop,
    Afue,
    Percent,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HeatingEfficiencyUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Hspf => Ok(Some(::std::borrow::Cow::Borrowed("HSPF"))),
            Self::Cop => Ok(Some(::std::borrow::Cow::Borrowed("COP"))),
            Self::Afue => Ok(Some(::std::borrow::Cow::Borrowed("AFUE"))),
            Self::Percent => Ok(Some(::std::borrow::Cow::Borrowed("Percent"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HeatingEfficiencyUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HeatingEfficiencyUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"HSPF" => Ok(Self::Hspf),
            b"COP" => Ok(Self::Cop),
            b"AFUE" => Ok(Self::Afue),
            b"Percent" => Ok(Self::Percent),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HeatingEfficiencyUnits {}
#[derive(Clone, Debug, PartialEq)]
pub struct HomeownerQuestionaireScore(pub ::core::primitive::i32);
impl HomeownerQuestionaireScore {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 10i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "10",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<HomeownerQuestionaireScore> for ::core::primitive::i32 {
    fn from(value: HomeownerQuestionaireScore) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for HomeownerQuestionaireScore {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for HomeownerQuestionaireScore {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HomeownerQuestionaireScore {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HomeownerQuestionaireScore {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HomeownerQuestionaireScore {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HomeownerQuestionaireScore {}
#[derive(Clone, Debug, PartialEq)]
pub enum HotWaterResetControl {
    Seasonal,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HotWaterResetControl {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Seasonal => Ok(Some(::std::borrow::Cow::Borrowed("seasonal"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HotWaterResetControl {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HotWaterResetControl {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"seasonal" => Ok(Self::Seasonal),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HotWaterResetControl {}
pub type Hours = ::core::primitive::i32;
#[derive(Clone, Debug, PartialEq)]
pub struct HoursPerDay(pub ::core::primitive::f64);
impl HoursPerDay {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 24f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "24",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<HoursPerDay> for ::core::primitive::f64 {
    fn from(value: HoursPerDay) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for HoursPerDay {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for HoursPerDay {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HoursPerDay {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HoursPerDay {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HoursPerDay {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HoursPerDay {}
#[derive(Clone, Debug, PartialEq)]
pub struct HousePressure(pub ::core::primitive::f64);
impl HousePressure {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<HousePressure> for ::core::primitive::f64 {
    fn from(value: HousePressure) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for HousePressure {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for HousePressure {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HousePressure {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HousePressure {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HousePressure {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HousePressure {}
#[derive(Clone, Debug, PartialEq)]
pub enum HouseholdType {
    FamilyHousehold,
    MarriedCoupleNoChildren,
    MaleHouseholdNoSpouse,
    FemaleHouseholdNoSpouse,
    NonfamilyHousehold,
    SingleMale,
    SingleFemale,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HouseholdType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::FamilyHousehold => Ok(Some(::std::borrow::Cow::Borrowed("family household"))),
            Self::MarriedCoupleNoChildren => Ok(Some(::std::borrow::Cow::Borrowed(
                "married couple, no children",
            ))),
            Self::MaleHouseholdNoSpouse => Ok(Some(::std::borrow::Cow::Borrowed(
                "male household, no spouse",
            ))),
            Self::FemaleHouseholdNoSpouse => Ok(Some(::std::borrow::Cow::Borrowed(
                "female household, no spouse",
            ))),
            Self::NonfamilyHousehold => {
                Ok(Some(::std::borrow::Cow::Borrowed("nonfamily household")))
            }
            Self::SingleMale => Ok(Some(::std::borrow::Cow::Borrowed("single male"))),
            Self::SingleFemale => Ok(Some(::std::borrow::Cow::Borrowed("single female"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HouseholdType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HouseholdType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"family household" => Ok(Self::FamilyHousehold),
            b"married couple, no children" => Ok(Self::MarriedCoupleNoChildren),
            b"male household, no spouse" => Ok(Self::MaleHouseholdNoSpouse),
            b"female household, no spouse" => Ok(Self::FemaleHouseholdNoSpouse),
            b"nonfamily household" => Ok(Self::NonfamilyHousehold),
            b"single male" => Ok(Self::SingleMale),
            b"single female" => Ok(Self::SingleFemale),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HouseholdType {}
#[derive(Clone, Debug, PartialEq)]
pub enum HydronicDistributionType {
    Radiator,
    Baseboard,
    RadiantFloor,
    RadiantCeiling,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for HydronicDistributionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Radiator => Ok(Some(::std::borrow::Cow::Borrowed("radiator"))),
            Self::Baseboard => Ok(Some(::std::borrow::Cow::Borrowed("baseboard"))),
            Self::RadiantFloor => Ok(Some(::std::borrow::Cow::Borrowed("radiant floor"))),
            Self::RadiantCeiling => Ok(Some(::std::borrow::Cow::Borrowed("radiant ceiling"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for HydronicDistributionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for HydronicDistributionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"radiator" => Ok(Self::Radiator),
            b"baseboard" => Ok(Self::Baseboard),
            b"radiant floor" => Ok(Self::RadiantFloor),
            b"radiant ceiling" => Ok(Self::RadiantCeiling),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for HydronicDistributionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum IeccYear {
    _2012,
    _2009,
    _2006,
    _2003,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for IeccYear {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_2012 => Ok(Some(::std::borrow::Cow::Borrowed("2012"))),
            Self::_2009 => Ok(Some(::std::borrow::Cow::Borrowed("2009"))),
            Self::_2006 => Ok(Some(::std::borrow::Cow::Borrowed("2006"))),
            Self::_2003 => Ok(Some(::std::borrow::Cow::Borrowed("2003"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for IeccYear {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IeccYear {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"2012" => Ok(Self::_2012),
            b"2009" => Ok(Self::_2009),
            b"2006" => Ok(Self::_2006),
            b"2003" => Ok(Self::_2003),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for IeccYear {}
#[derive(Clone, Debug, PartialEq)]
pub enum ImplementerQualification {
    Pe,
    Cem,
    BpiBa,
    BpiMfba,
    ResnetHomePartner,
    Ra,
    RefrigeratingSystemOperatingEngineer,
    HighPressureBoilerOperatingEngineer,
    HepEa,
    HepQci,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ImplementerQualification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Pe => Ok(Some(::std::borrow::Cow::Borrowed("PE"))),
            Self::Cem => Ok(Some(::std::borrow::Cow::Borrowed("CEM"))),
            Self::BpiBa => Ok(Some(::std::borrow::Cow::Borrowed("BPI-BA"))),
            Self::BpiMfba => Ok(Some(::std::borrow::Cow::Borrowed("BPI-MFBA"))),
            Self::ResnetHomePartner => {
                Ok(Some(::std::borrow::Cow::Borrowed("RESNET-Home Partner")))
            }
            Self::Ra => Ok(Some(::std::borrow::Cow::Borrowed("RA"))),
            Self::RefrigeratingSystemOperatingEngineer => Ok(Some(::std::borrow::Cow::Borrowed(
                "Refrigerating System Operating Engineer",
            ))),
            Self::HighPressureBoilerOperatingEngineer => Ok(Some(::std::borrow::Cow::Borrowed(
                "High Pressure Boiler Operating Engineer",
            ))),
            Self::HepEa => Ok(Some(::std::borrow::Cow::Borrowed("HEP - EA"))),
            Self::HepQci => Ok(Some(::std::borrow::Cow::Borrowed("HEP - QCI"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ImplementerQualification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ImplementerQualification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"PE" => Ok(Self::Pe),
            b"CEM" => Ok(Self::Cem),
            b"BPI-BA" => Ok(Self::BpiBa),
            b"BPI-MFBA" => Ok(Self::BpiMfba),
            b"RESNET-Home Partner" => Ok(Self::ResnetHomePartner),
            b"RA" => Ok(Self::Ra),
            b"Refrigerating System Operating Engineer" => {
                Ok(Self::RefrigeratingSystemOperatingEngineer)
            }
            b"High Pressure Boiler Operating Engineer" => {
                Ok(Self::HighPressureBoilerOperatingEngineer)
            }
            b"HEP - EA" => Ok(Self::HepEa),
            b"HEP - QCI" => Ok(Self::HepQci),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ImplementerQualification {}
#[derive(Clone, Debug, PartialEq)]
pub enum ImprovementStatusType {
    Installed,
    NotInstalled,
    Recommended,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ImprovementStatusType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Installed => Ok(Some(::std::borrow::Cow::Borrowed("Installed"))),
            Self::NotInstalled => Ok(Some(::std::borrow::Cow::Borrowed("NotInstalled"))),
            Self::Recommended => Ok(Some(::std::borrow::Cow::Borrowed("Recommended"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ImprovementStatusType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ImprovementStatusType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Installed" => Ok(Self::Installed),
            b"NotInstalled" => Ok(Self::NotInstalled),
            b"Recommended" => Ok(Self::Recommended),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ImprovementStatusType {}
pub type IncentiveAmount = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum IndividualType {
    OwnerOccupant,
    OwnerNonOccupant,
    PropertyManager,
    RealEstateAgent,
    Tenant,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for IndividualType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::OwnerOccupant => Ok(Some(::std::borrow::Cow::Borrowed("owner-occupant"))),
            Self::OwnerNonOccupant => Ok(Some(::std::borrow::Cow::Borrowed("owner-non-occupant"))),
            Self::PropertyManager => Ok(Some(::std::borrow::Cow::Borrowed("property manager"))),
            Self::RealEstateAgent => Ok(Some(::std::borrow::Cow::Borrowed("real estate agent"))),
            Self::Tenant => Ok(Some(::std::borrow::Cow::Borrowed("tenant"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for IndividualType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IndividualType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"owner-occupant" => Ok(Self::OwnerOccupant),
            b"owner-non-occupant" => Ok(Self::OwnerNonOccupant),
            b"property manager" => Ok(Self::PropertyManager),
            b"real estate agent" => Ok(Self::RealEstateAgent),
            b"tenant" => Ok(Self::Tenant),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for IndividualType {}
pub type InstallationDate = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum InstallationType {
    Cavity,
    Continuous,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InstallationType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cavity => Ok(Some(::std::borrow::Cow::Borrowed("cavity"))),
            Self::Continuous => Ok(Some(::std::borrow::Cow::Borrowed("continuous"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InstallationType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InstallationType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"cavity" => Ok(Self::Cavity),
            b"continuous" => Ok(Self::Continuous),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InstallationType {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationBattType {
    Fiberglass,
    Rockwool,
    RecycledCotton,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationBattType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Fiberglass => Ok(Some(::std::borrow::Cow::Borrowed("fiberglass"))),
            Self::Rockwool => Ok(Some(::std::borrow::Cow::Borrowed("rockwool"))),
            Self::RecycledCotton => Ok(Some(::std::borrow::Cow::Borrowed("recycled cotton"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationBattType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationBattType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"fiberglass" => Ok(Self::Fiberglass),
            b"rockwool" => Ok(Self::Rockwool),
            b"recycled cotton" => Ok(Self::RecycledCotton),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationBattType {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationCondition {
    Good,
    Fair,
    Poor,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationCondition {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Good => Ok(Some(::std::borrow::Cow::Borrowed("good"))),
            Self::Fair => Ok(Some(::std::borrow::Cow::Borrowed("fair"))),
            Self::Poor => Ok(Some(::std::borrow::Cow::Borrowed("poor"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationCondition {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationCondition {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"good" => Ok(Self::Good),
            b"fair" => Ok(Self::Fair),
            b"poor" => Ok(Self::Poor),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationCondition {}
#[derive(Clone, Debug, PartialEq)]
pub struct InsulationGrade(pub ::core::primitive::i32);
impl InsulationGrade {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 1i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("1"));
        }
        if *value > 3i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "3",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<InsulationGrade> for ::core::primitive::i32 {
    fn from(value: InsulationGrade) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for InsulationGrade {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for InsulationGrade {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationGrade {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationGrade {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationGrade {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationGrade {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationLocation {
    Interior,
    Exterior,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Interior => Ok(Some(::std::borrow::Cow::Borrowed("interior"))),
            Self::Exterior => Ok(Some(::std::borrow::Cow::Borrowed("exterior"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"interior" => Ok(Self::Interior),
            b"exterior" => Ok(Self::Exterior),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationLooseFillType {
    Cellulose,
    Fiberglass,
    Rockwool,
    Vermiculite,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationLooseFillType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cellulose => Ok(Some(::std::borrow::Cow::Borrowed("cellulose"))),
            Self::Fiberglass => Ok(Some(::std::borrow::Cow::Borrowed("fiberglass"))),
            Self::Rockwool => Ok(Some(::std::borrow::Cow::Borrowed("rockwool"))),
            Self::Vermiculite => Ok(Some(::std::borrow::Cow::Borrowed("vermiculite"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationLooseFillType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationLooseFillType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"cellulose" => Ok(Self::Cellulose),
            b"fiberglass" => Ok(Self::Fiberglass),
            b"rockwool" => Ok(Self::Rockwool),
            b"vermiculite" => Ok(Self::Vermiculite),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationLooseFillType {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationRigidType {
    Polyisocyanurate,
    Xps,
    Eps,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationRigidType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Polyisocyanurate => Ok(Some(::std::borrow::Cow::Borrowed("polyisocyanurate"))),
            Self::Xps => Ok(Some(::std::borrow::Cow::Borrowed("xps"))),
            Self::Eps => Ok(Some(::std::borrow::Cow::Borrowed("eps"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationRigidType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationRigidType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"polyisocyanurate" => Ok(Self::Polyisocyanurate),
            b"xps" => Ok(Self::Xps),
            b"eps" => Ok(Self::Eps),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationRigidType {}
#[derive(Clone, Debug, PartialEq)]
pub enum InsulationSprayFoamType {
    OpenCell,
    ClosedCell,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InsulationSprayFoamType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::OpenCell => Ok(Some(::std::borrow::Cow::Borrowed("open cell"))),
            Self::ClosedCell => Ok(Some(::std::borrow::Cow::Borrowed("closed cell"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InsulationSprayFoamType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InsulationSprayFoamType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"open cell" => Ok(Self::OpenCell),
            b"closed cell" => Ok(Self::ClosedCell),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InsulationSprayFoamType {}
#[derive(Clone, Debug, PartialEq)]
pub struct IntegerGreaterThanOrEqualToZero(pub ::core::primitive::i32);
impl IntegerGreaterThanOrEqualToZero {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<IntegerGreaterThanOrEqualToZero> for ::core::primitive::i32 {
    fn from(value: IntegerGreaterThanOrEqualToZero) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for IntegerGreaterThanOrEqualToZero {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for IntegerGreaterThanOrEqualToZero {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for IntegerGreaterThanOrEqualToZero {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for IntegerGreaterThanOrEqualToZero {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IntegerGreaterThanOrEqualToZero {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for IntegerGreaterThanOrEqualToZero {}
#[derive(Clone, Debug, PartialEq)]
pub struct IntegerGreaterThanZero(pub ::core::primitive::i32);
impl IntegerGreaterThanZero {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<IntegerGreaterThanZero> for ::core::primitive::i32 {
    fn from(value: IntegerGreaterThanZero) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for IntegerGreaterThanZero {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for IntegerGreaterThanZero {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for IntegerGreaterThanZero {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for IntegerGreaterThanZero {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IntegerGreaterThanZero {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for IntegerGreaterThanZero {}
#[derive(Clone, Debug, PartialEq)]
pub enum InteriorLocationsofWaterLeaksorDamage {
    Kitchen,
    Bathroom,
    Basement,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InteriorLocationsofWaterLeaksorDamage {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Kitchen => Ok(Some(::std::borrow::Cow::Borrowed("kitchen"))),
            Self::Bathroom => Ok(Some(::std::borrow::Cow::Borrowed("bathroom"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InteriorLocationsofWaterLeaksorDamage {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InteriorLocationsofWaterLeaksorDamage {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kitchen" => Ok(Self::Kitchen),
            b"bathroom" => Ok(Self::Bathroom),
            b"basement" => Ok(Self::Basement),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for InteriorLocationsofWaterLeaksorDamage
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum InteriorShading {
    LightBlinds,
    DarkBlinds,
    LightShades,
    DarkShades,
    LightCurtains,
    DarkCurtains,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for InteriorShading {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::LightBlinds => Ok(Some(::std::borrow::Cow::Borrowed("light blinds"))),
            Self::DarkBlinds => Ok(Some(::std::borrow::Cow::Borrowed("dark blinds"))),
            Self::LightShades => Ok(Some(::std::borrow::Cow::Borrowed("light shades"))),
            Self::DarkShades => Ok(Some(::std::borrow::Cow::Borrowed("dark shades"))),
            Self::LightCurtains => Ok(Some(::std::borrow::Cow::Borrowed("light curtains"))),
            Self::DarkCurtains => Ok(Some(::std::borrow::Cow::Borrowed("dark curtains"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for InteriorShading {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for InteriorShading {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"light blinds" => Ok(Self::LightBlinds),
            b"dark blinds" => Ok(Self::DarkBlinds),
            b"light shades" => Ok(Self::LightShades),
            b"dark shades" => Ok(Self::DarkShades),
            b"light curtains" => Ok(Self::LightCurtains),
            b"dark curtains" => Ok(Self::DarkCurtains),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for InteriorShading {}
#[derive(Clone, Debug, PartialEq)]
pub enum IntervalType {
    _15Minute,
    Hourly,
    Daily,
    Monthly,
    Annual,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for IntervalType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_15Minute => Ok(Some(::std::borrow::Cow::Borrowed("15-minute"))),
            Self::Hourly => Ok(Some(::std::borrow::Cow::Borrowed("hourly"))),
            Self::Daily => Ok(Some(::std::borrow::Cow::Borrowed("daily"))),
            Self::Monthly => Ok(Some(::std::borrow::Cow::Borrowed("monthly"))),
            Self::Annual => Ok(Some(::std::borrow::Cow::Borrowed("annual"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for IntervalType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for IntervalType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"15-minute" => Ok(Self::_15Minute),
            b"hourly" => Ok(Self::Hourly),
            b"daily" => Ok(Self::Daily),
            b"monthly" => Ok(Self::Monthly),
            b"annual" => Ok(Self::Annual),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for IntervalType {}
pub type JobRole = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum KnownOrEstimated {
    Known,
    Estimated,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for KnownOrEstimated {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Known => Ok(Some(::std::borrow::Cow::Borrowed("known"))),
            Self::Estimated => Ok(Some(::std::borrow::Cow::Borrowed("estimated"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for KnownOrEstimated {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for KnownOrEstimated {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"known" => Ok(Self::Known),
            b"estimated" => Ok(Self::Estimated),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for KnownOrEstimated {}
#[derive(Clone, Debug, PartialEq)]
pub enum LaundryMachineLocation {
    LaundryRoom,
    LivingSpace,
    Basement,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LaundryMachineLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::LaundryRoom => Ok(Some(::std::borrow::Cow::Borrowed("laundry room"))),
            Self::LivingSpace => Ok(Some(::std::borrow::Cow::Borrowed("living space"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LaundryMachineLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LaundryMachineLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"laundry room" => Ok(Self::LaundryRoom),
            b"living space" => Ok(Self::LivingSpace),
            b"basement" => Ok(Self::Basement),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LaundryMachineLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum LeakinessObservedVisualInspection {
    ConnectionsSealedWMastic,
    NoObservableLeaks,
    SomeObservableLeaks,
    SignificantLeaks,
    CatastrophicLeaks,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LeakinessObservedVisualInspection {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ConnectionsSealedWMastic => Ok(Some(::std::borrow::Cow::Borrowed(
                "connections sealed w mastic",
            ))),
            Self::NoObservableLeaks => {
                Ok(Some(::std::borrow::Cow::Borrowed("no observable leaks")))
            }
            Self::SomeObservableLeaks => {
                Ok(Some(::std::borrow::Cow::Borrowed("some observable leaks")))
            }
            Self::SignificantLeaks => Ok(Some(::std::borrow::Cow::Borrowed("significant leaks"))),
            Self::CatastrophicLeaks => Ok(Some(::std::borrow::Cow::Borrowed("catastrophic leaks"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LeakinessObservedVisualInspection {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LeakinessObservedVisualInspection {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"connections sealed w mastic" => Ok(Self::ConnectionsSealedWMastic),
            b"no observable leaks" => Ok(Self::NoObservableLeaks),
            b"some observable leaks" => Ok(Self::SomeObservableLeaks),
            b"significant leaks" => Ok(Self::SignificantLeaks),
            b"catastrophic leaks" => Ok(Self::CatastrophicLeaks),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LeakinessObservedVisualInspection {}
#[derive(Clone, Debug, PartialEq)]
pub struct LengthMeasurement(pub ::core::primitive::f64);
impl LengthMeasurement {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<LengthMeasurement> for ::core::primitive::f64 {
    fn from(value: LengthMeasurement) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for LengthMeasurement {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for LengthMeasurement {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LengthMeasurement {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LengthMeasurement {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LengthMeasurement {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LengthMeasurement {}
#[derive(Clone, Debug, PartialEq)]
pub enum LightingControls {
    DaylightDimming,
    OccupancySensors,
    VacancySensors,
    ManualDimming,
    BiLevelControl,
    Timers,
    Manual,
    AdvancedControls,
    PartOfEmcs,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LightingControls {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::DaylightDimming => Ok(Some(::std::borrow::Cow::Borrowed("daylight dimming"))),
            Self::OccupancySensors => Ok(Some(::std::borrow::Cow::Borrowed("occupancy sensors"))),
            Self::VacancySensors => Ok(Some(::std::borrow::Cow::Borrowed("vacancy sensors"))),
            Self::ManualDimming => Ok(Some(::std::borrow::Cow::Borrowed("manual dimming"))),
            Self::BiLevelControl => Ok(Some(::std::borrow::Cow::Borrowed("bi-level control"))),
            Self::Timers => Ok(Some(::std::borrow::Cow::Borrowed("timers"))),
            Self::Manual => Ok(Some(::std::borrow::Cow::Borrowed("manual"))),
            Self::AdvancedControls => Ok(Some(::std::borrow::Cow::Borrowed("advanced controls"))),
            Self::PartOfEmcs => Ok(Some(::std::borrow::Cow::Borrowed("part of emcs"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LightingControls {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LightingControls {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"daylight dimming" => Ok(Self::DaylightDimming),
            b"occupancy sensors" => Ok(Self::OccupancySensors),
            b"vacancy sensors" => Ok(Self::VacancySensors),
            b"manual dimming" => Ok(Self::ManualDimming),
            b"bi-level control" => Ok(Self::BiLevelControl),
            b"timers" => Ok(Self::Timers),
            b"manual" => Ok(Self::Manual),
            b"advanced controls" => Ok(Self::AdvancedControls),
            b"part of emcs" => Ok(Self::PartOfEmcs),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LightingControls {}
#[derive(Clone, Debug, PartialEq)]
pub enum LightingDailyHours {
    _1To4HoursPerDay,
    _4To12HoursPerDay,
    MoreThan12HoursPerDay,
    AllDay,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LightingDailyHours {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_1To4HoursPerDay => {
                Ok(Some(::std::borrow::Cow::Borrowed("1 to 4 hours per day")))
            }
            Self::_4To12HoursPerDay => {
                Ok(Some(::std::borrow::Cow::Borrowed("4 to 12 hours per day")))
            }
            Self::MoreThan12HoursPerDay => Ok(Some(::std::borrow::Cow::Borrowed(
                "more than 12 hours per day",
            ))),
            Self::AllDay => Ok(Some(::std::borrow::Cow::Borrowed("all day"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LightingDailyHours {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LightingDailyHours {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"1 to 4 hours per day" => Ok(Self::_1To4HoursPerDay),
            b"4 to 12 hours per day" => Ok(Self::_4To12HoursPerDay),
            b"more than 12 hours per day" => Ok(Self::MoreThan12HoursPerDay),
            b"all day" => Ok(Self::AllDay),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LightingDailyHours {}
#[derive(Clone, Debug, PartialEq)]
pub enum LightingFixtureThirdPartyCertification {
    EnergyStar,
    EnergyStarMostEfficient,
    CeeTier1,
    CeeTier2,
    CeeTier3,
    Other,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LightingFixtureThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::EnergyStarMostEfficient => Ok(Some(::std::borrow::Cow::Borrowed(
                "Energy Star Most Efficient",
            ))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 3"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LightingFixtureThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LightingFixtureThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"Energy Star Most Efficient" => Ok(Self::EnergyStarMostEfficient),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"CEE Tier 3" => Ok(Self::CeeTier3),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for LightingFixtureThirdPartyCertification
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum LightingLocation {
    Interior,
    Exterior,
    CommonArea,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LightingLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Interior => Ok(Some(::std::borrow::Cow::Borrowed("interior"))),
            Self::Exterior => Ok(Some(::std::borrow::Cow::Borrowed("exterior"))),
            Self::CommonArea => Ok(Some(::std::borrow::Cow::Borrowed("common area"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LightingLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LightingLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"interior" => Ok(Self::Interior),
            b"exterior" => Ok(Self::Exterior),
            b"common area" => Ok(Self::CommonArea),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LightingLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum LightingThirdPartyCertification {
    EnergyStar,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LightingThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LightingThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LightingThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LightingThirdPartyCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum LivingSpaceComponentsAirSealed {
    HomeGarageConnection,
    RimJoists,
    Baseboards,
    WindowsAndDoor,
    PlumbingPenetrations,
    HvacRegisters,
    InteriorSheatingVoids,
    Cantilevers,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for LivingSpaceComponentsAirSealed {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::HomeGarageConnection => {
                Ok(Some(::std::borrow::Cow::Borrowed("home-garage connection")))
            }
            Self::RimJoists => Ok(Some(::std::borrow::Cow::Borrowed("rim joists"))),
            Self::Baseboards => Ok(Some(::std::borrow::Cow::Borrowed("baseboards"))),
            Self::WindowsAndDoor => Ok(Some(::std::borrow::Cow::Borrowed("windows and door"))),
            Self::PlumbingPenetrations => {
                Ok(Some(::std::borrow::Cow::Borrowed("plumbing penetrations")))
            }
            Self::HvacRegisters => Ok(Some(::std::borrow::Cow::Borrowed("hvac registers"))),
            Self::InteriorSheatingVoids => Ok(Some(::std::borrow::Cow::Borrowed(
                "interior sheating voids",
            ))),
            Self::Cantilevers => Ok(Some(::std::borrow::Cow::Borrowed("cantilevers"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for LivingSpaceComponentsAirSealed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for LivingSpaceComponentsAirSealed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"home-garage connection" => Ok(Self::HomeGarageConnection),
            b"rim joists" => Ok(Self::RimJoists),
            b"baseboards" => Ok(Self::Baseboards),
            b"windows and door" => Ok(Self::WindowsAndDoor),
            b"plumbing penetrations" => Ok(Self::PlumbingPenetrations),
            b"hvac registers" => Ok(Self::HvacRegisters),
            b"interior sheating voids" => Ok(Self::InteriorSheatingVoids),
            b"cantilevers" => Ok(Self::Cantilevers),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for LivingSpaceComponentsAirSealed {}
pub type LoadProfile = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct Merv(pub ::core::primitive::i32);
impl Merv {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 1i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("1"));
        }
        if *value > 20i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "20",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Merv> for ::core::primitive::i32 {
    fn from(value: Merv) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for Merv {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Merv {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Merv {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Merv {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Merv {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Merv {}
pub type Manufacturer = ::std::string::String;
pub type MeasureCode = ::std::string::String;
pub type MeasureDescription = ::std::string::String;
pub type MeasuredDuctLeakage = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum MeasuredOrEstimated {
    Estimated,
    Measured,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for MeasuredOrEstimated {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Estimated => Ok(Some(::std::borrow::Cow::Borrowed("estimated"))),
            Self::Measured => Ok(Some(::std::borrow::Cow::Borrowed("measured"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for MeasuredOrEstimated {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for MeasuredOrEstimated {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"estimated" => Ok(Self::Estimated),
            b"measured" => Ok(Self::Measured),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for MeasuredOrEstimated {}
#[derive(Clone, Debug, PartialEq)]
pub enum MeterReadingType {
    Point,
    Median,
    Average,
    Total,
    Estimate,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for MeterReadingType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Point => Ok(Some(::std::borrow::Cow::Borrowed("point"))),
            Self::Median => Ok(Some(::std::borrow::Cow::Borrowed("median"))),
            Self::Average => Ok(Some(::std::borrow::Cow::Borrowed("average"))),
            Self::Total => Ok(Some(::std::borrow::Cow::Borrowed("total"))),
            Self::Estimate => Ok(Some(::std::borrow::Cow::Borrowed("estimate"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for MeterReadingType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for MeterReadingType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"point" => Ok(Self::Point),
            b"median" => Ok(Self::Median),
            b"average" => Ok(Self::Average),
            b"total" => Ok(Self::Total),
            b"estimate" => Ok(Self::Estimate),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for MeterReadingType {}
#[derive(Clone, Debug, PartialEq)]
pub enum MeteringConfiguration {
    DirectMetering,
    MasterMeterWithoutSubMetering,
    MasterMeterWithSubMetering,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for MeteringConfiguration {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::DirectMetering => Ok(Some(::std::borrow::Cow::Borrowed("direct metering"))),
            Self::MasterMeterWithoutSubMetering => Ok(Some(::std::borrow::Cow::Borrowed(
                "master meter without sub-metering",
            ))),
            Self::MasterMeterWithSubMetering => Ok(Some(::std::borrow::Cow::Borrowed(
                "master meter with sub-metering",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for MeteringConfiguration {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for MeteringConfiguration {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"direct metering" => Ok(Self::DirectMetering),
            b"master meter without sub-metering" => Ok(Self::MasterMeterWithoutSubMetering),
            b"master meter with sub-metering" => Ok(Self::MasterMeterWithSubMetering),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for MeteringConfiguration {}
pub type Model = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct MonthsPerYear(pub ::core::primitive::i32);
impl MonthsPerYear {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 12i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "12",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<MonthsPerYear> for ::core::primitive::i32 {
    fn from(value: MonthsPerYear) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for MonthsPerYear {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for MonthsPerYear {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for MonthsPerYear {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for MonthsPerYear {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for MonthsPerYear {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for MonthsPerYear {}
pub type NetPressureChange = ::core::primitive::f64;
pub type Notes = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct NumberOfFloorsType(pub ::core::primitive::f64);
impl NumberOfFloorsType {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<NumberOfFloorsType> for ::core::primitive::f64 {
    fn from(value: NumberOfFloorsType) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for NumberOfFloorsType {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for NumberOfFloorsType {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for NumberOfFloorsType {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for NumberOfFloorsType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for NumberOfFloorsType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for NumberOfFloorsType {}
#[derive(Clone, Debug, PartialEq)]
pub enum Occupancy {
    OwnerOccupied,
    RenterOccupied,
    OwnerAndRenterOccupied,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Occupancy {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::OwnerOccupied => Ok(Some(::std::borrow::Cow::Borrowed("owner-occupied"))),
            Self::RenterOccupied => Ok(Some(::std::borrow::Cow::Borrowed("renter-occupied"))),
            Self::OwnerAndRenterOccupied => Ok(Some(::std::borrow::Cow::Borrowed(
                "owner-and-renter-occupied",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Occupancy {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Occupancy {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"owner-occupied" => Ok(Self::OwnerOccupied),
            b"renter-occupied" => Ok(Self::RenterOccupied),
            b"owner-and-renter-occupied" => Ok(Self::OwnerAndRenterOccupied),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Occupancy {}
#[derive(Clone, Debug, PartialEq)]
pub enum OccupantIncomeRangeUnits {
    AreaMedianIncome,
    FederalPovertyLevel,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for OccupantIncomeRangeUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AreaMedianIncome => {
                Ok(Some(::std::borrow::Cow::Borrowed("% area median income")))
            }
            Self::FederalPovertyLevel => Ok(Some(::std::borrow::Cow::Borrowed(
                "% federal poverty level",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for OccupantIncomeRangeUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for OccupantIncomeRangeUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"% area median income" => Ok(Self::AreaMedianIncome),
            b"% federal poverty level" => Ok(Self::FederalPovertyLevel),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for OccupantIncomeRangeUnits {}
#[derive(Clone, Debug, PartialEq)]
pub enum OpenClosed {
    Open,
    Closed,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for OpenClosed {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Open => Ok(Some(::std::borrow::Cow::Borrowed("open"))),
            Self::Closed => Ok(Some(::std::borrow::Cow::Borrowed("closed"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for OpenClosed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for OpenClosed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"open" => Ok(Self::Open),
            b"closed" => Ok(Self::Closed),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for OpenClosed {}
#[derive(Clone, Debug, PartialEq)]
pub enum OrientationType {
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
    East,
    Northeast,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for OrientationType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::North => Ok(Some(::std::borrow::Cow::Borrowed("north"))),
            Self::Northwest => Ok(Some(::std::borrow::Cow::Borrowed("northwest"))),
            Self::West => Ok(Some(::std::borrow::Cow::Borrowed("west"))),
            Self::Southwest => Ok(Some(::std::borrow::Cow::Borrowed("southwest"))),
            Self::South => Ok(Some(::std::borrow::Cow::Borrowed("south"))),
            Self::Southeast => Ok(Some(::std::borrow::Cow::Borrowed("southeast"))),
            Self::East => Ok(Some(::std::borrow::Cow::Borrowed("east"))),
            Self::Northeast => Ok(Some(::std::borrow::Cow::Borrowed("northeast"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for OrientationType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for OrientationType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"north" => Ok(Self::North),
            b"northwest" => Ok(Self::Northwest),
            b"west" => Ok(Self::West),
            b"southwest" => Ok(Self::Southwest),
            b"south" => Ok(Self::South),
            b"southeast" => Ok(Self::Southeast),
            b"east" => Ok(Self::East),
            b"northeast" => Ok(Self::Northeast),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for OrientationType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PvSystemLocation {
    Roof,
    Ground,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PvSystemLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Roof => Ok(Some(::std::borrow::Cow::Borrowed("roof"))),
            Self::Ground => Ok(Some(::std::borrow::Cow::Borrowed("ground"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PvSystemLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PvSystemLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"roof" => Ok(Self::Roof),
            b"ground" => Ok(Self::Ground),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PvSystemLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum PvSystemOwnership {
    Leased,
    Owned,
    PowerPurchaseAgreement,
    UtilityOwned,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PvSystemOwnership {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Leased => Ok(Some(::std::borrow::Cow::Borrowed("leased"))),
            Self::Owned => Ok(Some(::std::borrow::Cow::Borrowed("owned"))),
            Self::PowerPurchaseAgreement => Ok(Some(::std::borrow::Cow::Borrowed(
                "power purchase agreement",
            ))),
            Self::UtilityOwned => Ok(Some(::std::borrow::Cow::Borrowed("utility owned"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PvSystemOwnership {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PvSystemOwnership {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"leased" => Ok(Self::Leased),
            b"owned" => Ok(Self::Owned),
            b"power purchase agreement" => Ok(Self::PowerPurchaseAgreement),
            b"utility owned" => Ok(Self::UtilityOwned),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PvSystemOwnership {}
#[derive(Clone, Debug, PartialEq)]
pub enum PeakSeason {
    Summer,
    Winter,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PeakSeason {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Summer => Ok(Some(::std::borrow::Cow::Borrowed("summer"))),
            Self::Winter => Ok(Some(::std::borrow::Cow::Borrowed("winter"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PeakSeason {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PeakSeason {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"summer" => Ok(Self::Summer),
            b"winter" => Ok(Self::Winter),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PeakSeason {}
#[derive(Clone, Debug, PartialEq)]
pub struct PeopleCount(pub ::core::primitive::f64);
impl PeopleCount {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<PeopleCount> for ::core::primitive::f64 {
    fn from(value: PeopleCount) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for PeopleCount {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for PeopleCount {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PeopleCount {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PeopleCount {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PeopleCount {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PeopleCount {}
pub type PipeInsulated = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct Pitch(pub ::core::primitive::f64);
impl Pitch {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Pitch> for ::core::primitive::f64 {
    fn from(value: Pitch) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Pitch {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Pitch {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Pitch {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Pitch {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Pitch {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Pitch {}
#[derive(Clone, Debug, PartialEq)]
pub enum PlugLoadControlType {
    AdvancedPowerStripForAv,
    AdvancedPowerStripForIt,
    WholeHouseEnergyManagementSystem,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PlugLoadControlType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AdvancedPowerStripForAv => Ok(Some(::std::borrow::Cow::Borrowed(
                "advanced power strip for AV",
            ))),
            Self::AdvancedPowerStripForIt => Ok(Some(::std::borrow::Cow::Borrowed(
                "advanced power strip for IT",
            ))),
            Self::WholeHouseEnergyManagementSystem => Ok(Some(::std::borrow::Cow::Borrowed(
                "whole-house energy management system",
            ))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PlugLoadControlType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PlugLoadControlType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"advanced power strip for AV" => Ok(Self::AdvancedPowerStripForAv),
            b"advanced power strip for IT" => Ok(Self::AdvancedPowerStripForIt),
            b"whole-house energy management system" => Ok(Self::WholeHouseEnergyManagementSystem),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PlugLoadControlType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PlugLoadLocation {
    Interior,
    Exterior,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PlugLoadLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Interior => Ok(Some(::std::borrow::Cow::Borrowed("interior"))),
            Self::Exterior => Ok(Some(::std::borrow::Cow::Borrowed("exterior"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PlugLoadLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PlugLoadLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"interior" => Ok(Self::Interior),
            b"exterior" => Ok(Self::Exterior),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PlugLoadLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum PlugLoadType {
    TvPlasma,
    TvCrt,
    TvOther,
    Computer,
    SpaceHeater,
    WaterBed,
    Aquarium,
    ElectricVehicleCharging,
    WellPump,
    Sauna,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PlugLoadType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::TvPlasma => Ok(Some(::std::borrow::Cow::Borrowed("TV plasma"))),
            Self::TvCrt => Ok(Some(::std::borrow::Cow::Borrowed("TV CRT"))),
            Self::TvOther => Ok(Some(::std::borrow::Cow::Borrowed("TV other"))),
            Self::Computer => Ok(Some(::std::borrow::Cow::Borrowed("computer"))),
            Self::SpaceHeater => Ok(Some(::std::borrow::Cow::Borrowed("space heater"))),
            Self::WaterBed => Ok(Some(::std::borrow::Cow::Borrowed("water bed"))),
            Self::Aquarium => Ok(Some(::std::borrow::Cow::Borrowed("aquarium"))),
            Self::ElectricVehicleCharging => Ok(Some(::std::borrow::Cow::Borrowed(
                "electric vehicle charging",
            ))),
            Self::WellPump => Ok(Some(::std::borrow::Cow::Borrowed("well pump"))),
            Self::Sauna => Ok(Some(::std::borrow::Cow::Borrowed("sauna"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PlugLoadType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PlugLoadType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"TV plasma" => Ok(Self::TvPlasma),
            b"TV CRT" => Ok(Self::TvCrt),
            b"TV other" => Ok(Self::TvOther),
            b"computer" => Ok(Self::Computer),
            b"space heater" => Ok(Self::SpaceHeater),
            b"water bed" => Ok(Self::WaterBed),
            b"aquarium" => Ok(Self::Aquarium),
            b"electric vehicle charging" => Ok(Self::ElectricVehicleCharging),
            b"well pump" => Ok(Self::WellPump),
            b"sauna" => Ok(Self::Sauna),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PlugLoadType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PlugLoadUnits {
    KWhYear,
    W,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PlugLoadUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::KWhYear => Ok(Some(::std::borrow::Cow::Borrowed("kWh/year"))),
            Self::W => Ok(Some(::std::borrow::Cow::Borrowed("W"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PlugLoadUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PlugLoadUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kWh/year" => Ok(Self::KWhYear),
            b"W" => Ok(Self::W),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PlugLoadUnits {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolCleanerType {
    Robotic,
    SuctionSide,
    PressureSide,
    BoosterPump,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolCleanerType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Robotic => Ok(Some(::std::borrow::Cow::Borrowed("robotic"))),
            Self::SuctionSide => Ok(Some(::std::borrow::Cow::Borrowed("suction side"))),
            Self::PressureSide => Ok(Some(::std::borrow::Cow::Borrowed("pressure side"))),
            Self::BoosterPump => Ok(Some(::std::borrow::Cow::Borrowed("booster pump"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolCleanerType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolCleanerType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"robotic" => Ok(Self::Robotic),
            b"suction side" => Ok(Self::SuctionSide),
            b"pressure side" => Ok(Self::PressureSide),
            b"booster pump" => Ok(Self::BoosterPump),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolCleanerType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolFilterType {
    Sand,
    DiatomaceousEarth,
    Cartridge,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolFilterType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Sand => Ok(Some(::std::borrow::Cow::Borrowed("sand"))),
            Self::DiatomaceousEarth => Ok(Some(::std::borrow::Cow::Borrowed("diatomaceous earth"))),
            Self::Cartridge => Ok(Some(::std::borrow::Cow::Borrowed("cartridge"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolFilterType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolFilterType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"sand" => Ok(Self::Sand),
            b"diatomaceous earth" => Ok(Self::DiatomaceousEarth),
            b"cartridge" => Ok(Self::Cartridge),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolFilterType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolHeaterType {
    GasFired,
    ElectricResistance,
    HeatPump,
    Solar,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolHeaterType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::GasFired => Ok(Some(::std::borrow::Cow::Borrowed("gas fired"))),
            Self::ElectricResistance => {
                Ok(Some(::std::borrow::Cow::Borrowed("electric resistance")))
            }
            Self::HeatPump => Ok(Some(::std::borrow::Cow::Borrowed("heat pump"))),
            Self::Solar => Ok(Some(::std::borrow::Cow::Borrowed("solar"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolHeaterType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolHeaterType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"gas fired" => Ok(Self::GasFired),
            b"electric resistance" => Ok(Self::ElectricResistance),
            b"heat pump" => Ok(Self::HeatPump),
            b"solar" => Ok(Self::Solar),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolHeaterType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolPump3RdPartyCertification {
    EnergyStar,
    EnergyStarMostEfficient,
    CeeTier1,
    CeeTier2,
    CeeTier3,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolPump3RdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("ENERGY STAR"))),
            Self::EnergyStarMostEfficient => Ok(Some(::std::borrow::Cow::Borrowed(
                "ENERGY STAR Most Efficient",
            ))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("Cee Tier 3"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolPump3RdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolPump3RdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"ENERGY STAR" => Ok(Self::EnergyStar),
            b"ENERGY STAR Most Efficient" => Ok(Self::EnergyStarMostEfficient),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"Cee Tier 3" => Ok(Self::CeeTier3),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolPump3RdPartyCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolPumpSpeedSetting {
    Low,
    High,
    MostEfficient,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolPumpSpeedSetting {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Low => Ok(Some(::std::borrow::Cow::Borrowed("low"))),
            Self::High => Ok(Some(::std::borrow::Cow::Borrowed("high"))),
            Self::MostEfficient => Ok(Some(::std::borrow::Cow::Borrowed("most efficient"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolPumpSpeedSetting {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolPumpSpeedSetting {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"low" => Ok(Self::Low),
            b"high" => Ok(Self::High),
            b"most efficient" => Ok(Self::MostEfficient),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolPumpSpeedSetting {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolPumpType {
    SingleSpeed,
    MultiSpeed,
    VariableSpeed,
    VariableFlow,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolPumpType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SingleSpeed => Ok(Some(::std::borrow::Cow::Borrowed("single speed"))),
            Self::MultiSpeed => Ok(Some(::std::borrow::Cow::Borrowed("multi speed"))),
            Self::VariableSpeed => Ok(Some(::std::borrow::Cow::Borrowed("variable speed"))),
            Self::VariableFlow => Ok(Some(::std::borrow::Cow::Borrowed("variable flow"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolPumpType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolPumpType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"single speed" => Ok(Self::SingleSpeed),
            b"multi speed" => Ok(Self::MultiSpeed),
            b"variable speed" => Ok(Self::VariableSpeed),
            b"variable flow" => Ok(Self::VariableFlow),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolPumpType {}
#[derive(Clone, Debug, PartialEq)]
pub enum PoolType {
    InGround,
    OnGround,
    AboveGround,
    Other,
    Unknown,
    None,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for PoolType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::InGround => Ok(Some(::std::borrow::Cow::Borrowed("in ground"))),
            Self::OnGround => Ok(Some(::std::borrow::Cow::Borrowed("on ground"))),
            Self::AboveGround => Ok(Some(::std::borrow::Cow::Borrowed("above ground"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
            Self::None => Ok(Some(::std::borrow::Cow::Borrowed("none"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for PoolType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for PoolType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"in ground" => Ok(Self::InGround),
            b"on ground" => Ok(Self::OnGround),
            b"above ground" => Ok(Self::AboveGround),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            b"none" => Ok(Self::None),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for PoolType {}
#[derive(Clone, Debug, PartialEq)]
pub struct Power(pub ::core::primitive::f64);
impl Power {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Power> for ::core::primitive::f64 {
    fn from(value: Power) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Power {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Power {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Power {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Power {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Power {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Power {}
pub type PrefixName = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum ProgramCertificate {
    HomePerformanceWithEnergyStar,
    LeedCertified,
    LeedSilver,
    LeedGold,
    LeedPlatinum,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ProgramCertificate {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::HomePerformanceWithEnergyStar => Ok(Some(::std::borrow::Cow::Borrowed(
                "Home Performance with Energy Star",
            ))),
            Self::LeedCertified => Ok(Some(::std::borrow::Cow::Borrowed("LEED Certified"))),
            Self::LeedSilver => Ok(Some(::std::borrow::Cow::Borrowed("LEED Silver"))),
            Self::LeedGold => Ok(Some(::std::borrow::Cow::Borrowed("LEED Gold"))),
            Self::LeedPlatinum => Ok(Some(::std::borrow::Cow::Borrowed("LEED Platinum"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ProgramCertificate {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ProgramCertificate {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Home Performance with Energy Star" => Ok(Self::HomePerformanceWithEnergyStar),
            b"LEED Certified" => Ok(Self::LeedCertified),
            b"LEED Silver" => Ok(Self::LeedSilver),
            b"LEED Gold" => Ok(Self::LeedGold),
            b"LEED Platinum" => Ok(Self::LeedPlatinum),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ProgramCertificate {}
pub type ProgramName = ::std::string::String;
pub type ProgramSponsor = ::std::string::String;
pub type ProjectType = ::std::string::String;
pub type Quantity = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub struct RValue(pub ::core::primitive::f64);
impl RValue {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<RValue> for ::core::primitive::f64 {
    fn from(value: RValue) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for RValue {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for RValue {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RValue {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RValue {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RValue {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RValue {}
#[derive(Clone, Debug, PartialEq)]
pub enum RadiantBarrierLocation {
    TopSideOfTrussUnderSheathing,
    BelowBottomChordOfTruss,
    AtticFloor,
    UndersideOfRafters,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RadiantBarrierLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::TopSideOfTrussUnderSheathing => Ok(Some(::std::borrow::Cow::Borrowed(
                "top side of truss under sheathing",
            ))),
            Self::BelowBottomChordOfTruss => Ok(Some(::std::borrow::Cow::Borrowed(
                "below bottom chord of truss",
            ))),
            Self::AtticFloor => Ok(Some(::std::borrow::Cow::Borrowed("attic floor"))),
            Self::UndersideOfRafters => {
                Ok(Some(::std::borrow::Cow::Borrowed("underside of rafters")))
            }
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RadiantBarrierLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RadiantBarrierLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"top side of truss under sheathing" => Ok(Self::TopSideOfTrussUnderSheathing),
            b"below bottom chord of truss" => Ok(Self::BelowBottomChordOfTruss),
            b"attic floor" => Ok(Self::AtticFloor),
            b"underside of rafters" => Ok(Self::UndersideOfRafters),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RadiantBarrierLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum RadonTestLocation {
    Kitchen,
    Crawlspace,
    Basement,
    Bedroom,
    LivingRoom,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RadonTestLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Kitchen => Ok(Some(::std::borrow::Cow::Borrowed("kitchen"))),
            Self::Crawlspace => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Bedroom => Ok(Some(::std::borrow::Cow::Borrowed("bedroom"))),
            Self::LivingRoom => Ok(Some(::std::borrow::Cow::Borrowed("living room"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RadonTestLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RadonTestLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kitchen" => Ok(Self::Kitchen),
            b"crawlspace" => Ok(Self::Crawlspace),
            b"basement" => Ok(Self::Basement),
            b"bedroom" => Ok(Self::Bedroom),
            b"living room" => Ok(Self::LivingRoom),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RadonTestLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum RadonTestTypes {
    ActivatedCharcoalAbsorption,
    AlphaTrackDetectors,
    UnfilteredTrackDetection,
    ShortTermElectretIonChamber,
    LongTermElectretIonChamber,
    ContinuousRadonMonitoring,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RadonTestTypes {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ActivatedCharcoalAbsorption => Ok(Some(::std::borrow::Cow::Borrowed(
                "activated charcoal absorption",
            ))),
            Self::AlphaTrackDetectors => {
                Ok(Some(::std::borrow::Cow::Borrowed("alpha-track detectors")))
            }
            Self::UnfilteredTrackDetection => Ok(Some(::std::borrow::Cow::Borrowed(
                "unfiltered track detection",
            ))),
            Self::ShortTermElectretIonChamber => Ok(Some(::std::borrow::Cow::Borrowed(
                "short term electret ion chamber",
            ))),
            Self::LongTermElectretIonChamber => Ok(Some(::std::borrow::Cow::Borrowed(
                "long term electret ion chamber",
            ))),
            Self::ContinuousRadonMonitoring => Ok(Some(::std::borrow::Cow::Borrowed(
                "continuous radon monitoring",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RadonTestTypes {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RadonTestTypes {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"activated charcoal absorption" => Ok(Self::ActivatedCharcoalAbsorption),
            b"alpha-track detectors" => Ok(Self::AlphaTrackDetectors),
            b"unfiltered track detection" => Ok(Self::UnfilteredTrackDetection),
            b"short term electret ion chamber" => Ok(Self::ShortTermElectretIonChamber),
            b"long term electret ion chamber" => Ok(Self::LongTermElectretIonChamber),
            b"continuous radon monitoring" => Ok(Self::ContinuousRadonMonitoring),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RadonTestTypes {}
#[derive(Clone, Debug, PartialEq)]
pub struct RadonZone(pub ::core::primitive::i32);
impl RadonZone {
    pub fn new(
        inner: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::i32 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::i32,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 1i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("1"));
        }
        if *value > 3i32 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "3",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<RadonZone> for ::core::primitive::i32 {
    fn from(value: RadonZone) -> ::core::primitive::i32 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::i32> for RadonZone {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::i32,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for RadonZone {
    type Target = ::core::primitive::i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RadonZone {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RadonZone {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RadonZone {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::i32::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RadonZone {}
#[derive(Clone, Debug, PartialEq)]
pub struct RatedAnnualkWh(pub ::core::primitive::f64);
impl RatedAnnualkWh {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<RatedAnnualkWh> for ::core::primitive::f64 {
    fn from(value: RatedAnnualkWh) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for RatedAnnualkWh {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for RatedAnnualkWh {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RatedAnnualkWh {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RatedAnnualkWh {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RatedAnnualkWh {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RatedAnnualkWh {}
#[derive(Clone, Debug, PartialEq)]
pub struct RatedWaterGalPerCycle(pub ::core::primitive::f64);
impl RatedWaterGalPerCycle {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<RatedWaterGalPerCycle> for ::core::primitive::f64 {
    fn from(value: RatedWaterGalPerCycle) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for RatedWaterGalPerCycle {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for RatedWaterGalPerCycle {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RatedWaterGalPerCycle {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RatedWaterGalPerCycle {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RatedWaterGalPerCycle {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RatedWaterGalPerCycle {}
pub type ReceivingSystemIdentifierType = ::std::string::String;
pub type ReceivingSystemIdentifierValue = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum RecirculationControlType {
    NoControl,
    Timer,
    Temperature,
    PresenceSensorDemandControl,
    ManualDemandControl,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RecirculationControlType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::NoControl => Ok(Some(::std::borrow::Cow::Borrowed("no control"))),
            Self::Timer => Ok(Some(::std::borrow::Cow::Borrowed("timer"))),
            Self::Temperature => Ok(Some(::std::borrow::Cow::Borrowed("temperature"))),
            Self::PresenceSensorDemandControl => Ok(Some(::std::borrow::Cow::Borrowed(
                "presence sensor demand control",
            ))),
            Self::ManualDemandControl => {
                Ok(Some(::std::borrow::Cow::Borrowed("manual demand control")))
            }
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RecirculationControlType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RecirculationControlType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"no control" => Ok(Self::NoControl),
            b"timer" => Ok(Self::Timer),
            b"temperature" => Ok(Self::Temperature),
            b"presence sensor demand control" => Ok(Self::PresenceSensorDemandControl),
            b"manual demand control" => Ok(Self::ManualDemandControl),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RecirculationControlType {}
#[derive(Clone, Debug, PartialEq)]
pub enum Recommendation {
    Require,
    Recommend,
    NoRecommendation,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Recommendation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Require => Ok(Some(::std::borrow::Cow::Borrowed("require"))),
            Self::Recommend => Ok(Some(::std::borrow::Cow::Borrowed("recommend"))),
            Self::NoRecommendation => Ok(Some(::std::borrow::Cow::Borrowed("no recommendation"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Recommendation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Recommendation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"require" => Ok(Self::Require),
            b"recommend" => Ok(Self::Recommend),
            b"no recommendation" => Ok(Self::NoRecommendation),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Recommendation {}
#[derive(Clone, Debug, PartialEq)]
pub struct RecoveryEfficiency(pub ::core::primitive::f64);
impl RecoveryEfficiency {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        if *value > 5f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "5",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<RecoveryEfficiency> for ::core::primitive::f64 {
    fn from(value: RecoveryEfficiency) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for RecoveryEfficiency {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for RecoveryEfficiency {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RecoveryEfficiency {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RecoveryEfficiency {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RecoveryEfficiency {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RecoveryEfficiency {}
#[derive(Clone, Debug, PartialEq)]
pub enum RefrigeratorLocation {
    Kitchen,
    LivingSpace,
    Basement,
    Garage,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RefrigeratorLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Kitchen => Ok(Some(::std::borrow::Cow::Borrowed("kitchen"))),
            Self::LivingSpace => Ok(Some(::std::borrow::Cow::Borrowed("living space"))),
            Self::Basement => Ok(Some(::std::borrow::Cow::Borrowed("basement"))),
            Self::Garage => Ok(Some(::std::borrow::Cow::Borrowed("garage"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RefrigeratorLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RefrigeratorLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kitchen" => Ok(Self::Kitchen),
            b"living space" => Ok(Self::LivingSpace),
            b"basement" => Ok(Self::Basement),
            b"garage" => Ok(Self::Garage),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RefrigeratorLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum RefrigeratorStyle {
    SideBySide,
    TopFreezer,
    BottomFreezer,
    SingleDoor,
    FullSizeOneDoor,
    FullSizeTwoDoors,
    HalfOrQuarterSize,
    WalkIn,
    OpenCase,
    ClosedCase,
    Uncategorized,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RefrigeratorStyle {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SideBySide => Ok(Some(::std::borrow::Cow::Borrowed("side-by-side"))),
            Self::TopFreezer => Ok(Some(::std::borrow::Cow::Borrowed("top freezer"))),
            Self::BottomFreezer => Ok(Some(::std::borrow::Cow::Borrowed("bottom freezer"))),
            Self::SingleDoor => Ok(Some(::std::borrow::Cow::Borrowed("single door"))),
            Self::FullSizeOneDoor => Ok(Some(::std::borrow::Cow::Borrowed("full-size one door"))),
            Self::FullSizeTwoDoors => Ok(Some(::std::borrow::Cow::Borrowed("full-size two doors"))),
            Self::HalfOrQuarterSize => {
                Ok(Some(::std::borrow::Cow::Borrowed("half or quarter size")))
            }
            Self::WalkIn => Ok(Some(::std::borrow::Cow::Borrowed("walk-in"))),
            Self::OpenCase => Ok(Some(::std::borrow::Cow::Borrowed("open case"))),
            Self::ClosedCase => Ok(Some(::std::borrow::Cow::Borrowed("closed case"))),
            Self::Uncategorized => Ok(Some(::std::borrow::Cow::Borrowed("uncategorized"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RefrigeratorStyle {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RefrigeratorStyle {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"side-by-side" => Ok(Self::SideBySide),
            b"top freezer" => Ok(Self::TopFreezer),
            b"bottom freezer" => Ok(Self::BottomFreezer),
            b"single door" => Ok(Self::SingleDoor),
            b"full-size one door" => Ok(Self::FullSizeOneDoor),
            b"full-size two doors" => Ok(Self::FullSizeTwoDoors),
            b"half or quarter size" => Ok(Self::HalfOrQuarterSize),
            b"walk-in" => Ok(Self::WalkIn),
            b"open case" => Ok(Self::OpenCase),
            b"closed case" => Ok(Self::ClosedCase),
            b"uncategorized" => Ok(Self::Uncategorized),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RefrigeratorStyle {}
#[derive(Clone, Debug, PartialEq)]
pub enum ResidentPopulationType {
    NoSpecificResidentPopulation,
    Student,
    Military,
    Senior,
    SpecialAccessibilityNeeds,
    YoungChildren,
    AtRisk,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ResidentPopulationType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::NoSpecificResidentPopulation => Ok(Some(::std::borrow::Cow::Borrowed(
                "no specific resident population",
            ))),
            Self::Student => Ok(Some(::std::borrow::Cow::Borrowed("student"))),
            Self::Military => Ok(Some(::std::borrow::Cow::Borrowed("military"))),
            Self::Senior => Ok(Some(::std::borrow::Cow::Borrowed("senior"))),
            Self::SpecialAccessibilityNeeds => Ok(Some(::std::borrow::Cow::Borrowed(
                "special accessibility needs",
            ))),
            Self::YoungChildren => Ok(Some(::std::borrow::Cow::Borrowed("young children"))),
            Self::AtRisk => Ok(Some(::std::borrow::Cow::Borrowed("at risk"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ResidentPopulationType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ResidentPopulationType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"no specific resident population" => Ok(Self::NoSpecificResidentPopulation),
            b"student" => Ok(Self::Student),
            b"military" => Ok(Self::Military),
            b"senior" => Ok(Self::Senior),
            b"special accessibility needs" => Ok(Self::SpecialAccessibilityNeeds),
            b"young children" => Ok(Self::YoungChildren),
            b"at risk" => Ok(Self::AtRisk),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ResidentPopulationType {}
#[derive(Clone, Debug, PartialEq)]
pub enum ResidentialFacilityType {
    SingleFamilyDetached,
    SingleFamilyAttached,
    ManufacturedHome,
    _24UnitBuilding,
    _5UnitBuilding,
    MultiFamilyUncategorized,
    MultiFamilyTownHomes,
    MultiFamilyCondos,
    ApartmentUnit,
    StudioUnit,
    Other,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ResidentialFacilityType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SingleFamilyDetached => {
                Ok(Some(::std::borrow::Cow::Borrowed("single-family detached")))
            }
            Self::SingleFamilyAttached => {
                Ok(Some(::std::borrow::Cow::Borrowed("single-family attached")))
            }
            Self::ManufacturedHome => Ok(Some(::std::borrow::Cow::Borrowed("manufactured home"))),
            Self::_24UnitBuilding => Ok(Some(::std::borrow::Cow::Borrowed("2-4 unit building"))),
            Self::_5UnitBuilding => Ok(Some(::std::borrow::Cow::Borrowed("5+ unit building"))),
            Self::MultiFamilyUncategorized => Ok(Some(::std::borrow::Cow::Borrowed(
                "multi-family - uncategorized",
            ))),
            Self::MultiFamilyTownHomes => Ok(Some(::std::borrow::Cow::Borrowed(
                "multi-family - town homes",
            ))),
            Self::MultiFamilyCondos => {
                Ok(Some(::std::borrow::Cow::Borrowed("multi-family - condos")))
            }
            Self::ApartmentUnit => Ok(Some(::std::borrow::Cow::Borrowed("apartment unit"))),
            Self::StudioUnit => Ok(Some(::std::borrow::Cow::Borrowed("studio unit"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ResidentialFacilityType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ResidentialFacilityType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"single-family detached" => Ok(Self::SingleFamilyDetached),
            b"single-family attached" => Ok(Self::SingleFamilyAttached),
            b"manufactured home" => Ok(Self::ManufacturedHome),
            b"2-4 unit building" => Ok(Self::_24UnitBuilding),
            b"5+ unit building" => Ok(Self::_5UnitBuilding),
            b"multi-family - uncategorized" => Ok(Self::MultiFamilyUncategorized),
            b"multi-family - town homes" => Ok(Self::MultiFamilyTownHomes),
            b"multi-family - condos" => Ok(Self::MultiFamilyCondos),
            b"apartment unit" => Ok(Self::ApartmentUnit),
            b"studio unit" => Ok(Self::StudioUnit),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ResidentialFacilityType {}
pub type ResourceTypeCode = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum RoofType {
    Shingles,
    SlateOrTileShingles,
    WoodShinglesOrShakes,
    AsphaltOrFiberglassShingles,
    MetalSurfacing,
    ExpandedPolystyreneSheathing,
    PlasticRubberSyntheticSheeting,
    Concrete,
    CoolRoof,
    GreenRoof,
    NoOneMajorType,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for RoofType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Shingles => Ok(Some(::std::borrow::Cow::Borrowed("shingles"))),
            Self::SlateOrTileShingles => {
                Ok(Some(::std::borrow::Cow::Borrowed("slate or tile shingles")))
            }
            Self::WoodShinglesOrShakes => Ok(Some(::std::borrow::Cow::Borrowed(
                "wood shingles or shakes",
            ))),
            Self::AsphaltOrFiberglassShingles => Ok(Some(::std::borrow::Cow::Borrowed(
                "asphalt or fiberglass shingles",
            ))),
            Self::MetalSurfacing => Ok(Some(::std::borrow::Cow::Borrowed("metal surfacing"))),
            Self::ExpandedPolystyreneSheathing => Ok(Some(::std::borrow::Cow::Borrowed(
                "expanded polystyrene sheathing",
            ))),
            Self::PlasticRubberSyntheticSheeting => Ok(Some(::std::borrow::Cow::Borrowed(
                "plastic/rubber/synthetic sheeting",
            ))),
            Self::Concrete => Ok(Some(::std::borrow::Cow::Borrowed("concrete"))),
            Self::CoolRoof => Ok(Some(::std::borrow::Cow::Borrowed("cool roof"))),
            Self::GreenRoof => Ok(Some(::std::borrow::Cow::Borrowed("green roof"))),
            Self::NoOneMajorType => Ok(Some(::std::borrow::Cow::Borrowed("no one major type"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for RoofType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for RoofType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"shingles" => Ok(Self::Shingles),
            b"slate or tile shingles" => Ok(Self::SlateOrTileShingles),
            b"wood shingles or shakes" => Ok(Self::WoodShinglesOrShakes),
            b"asphalt or fiberglass shingles" => Ok(Self::AsphaltOrFiberglassShingles),
            b"metal surfacing" => Ok(Self::MetalSurfacing),
            b"expanded polystyrene sheathing" => Ok(Self::ExpandedPolystyreneSheathing),
            b"plastic/rubber/synthetic sheeting" => Ok(Self::PlasticRubberSyntheticSheeting),
            b"concrete" => Ok(Self::Concrete),
            b"cool roof" => Ok(Self::CoolRoof),
            b"green roof" => Ok(Self::GreenRoof),
            b"no one major type" => Ok(Self::NoOneMajorType),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for RoofType {}
#[derive(Clone, Debug, PartialEq)]
pub struct Seer(pub ::core::primitive::f64);
impl Seer {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Seer> for ::core::primitive::f64 {
    fn from(value: Seer) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Seer {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Seer {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Seer {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Seer {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Seer {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Seer {}
#[derive(Clone, Debug, PartialEq)]
pub struct Shgc(pub ::core::primitive::f64);
impl Shgc {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        if *value >= 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterEqualThan("1"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Shgc> for ::core::primitive::f64 {
    fn from(value: Shgc) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Shgc {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Shgc {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Shgc {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Shgc {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Shgc {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Shgc {}
pub type SendingSystemIdentifierType = ::std::string::String;
pub type SendingSystemIdentifierValue = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum SharedEnergySystem {
    Yes,
    No,
    CommonMeter,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SharedEnergySystem {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Yes => Ok(Some(::std::borrow::Cow::Borrowed("yes"))),
            Self::No => Ok(Some(::std::borrow::Cow::Borrowed("no"))),
            Self::CommonMeter => Ok(Some(::std::borrow::Cow::Borrowed("common meter"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SharedEnergySystem {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SharedEnergySystem {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"yes" => Ok(Self::Yes),
            b"no" => Ok(Self::No),
            b"common meter" => Ok(Self::CommonMeter),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SharedEnergySystem {}
#[derive(Clone, Debug, PartialEq)]
pub enum ShieldingofHome {
    WellShielded,
    Normal,
    Exposed,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ShieldingofHome {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::WellShielded => Ok(Some(::std::borrow::Cow::Borrowed("well-shielded"))),
            Self::Normal => Ok(Some(::std::borrow::Cow::Borrowed("normal"))),
            Self::Exposed => Ok(Some(::std::borrow::Cow::Borrowed("exposed"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ShieldingofHome {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ShieldingofHome {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"well-shielded" => Ok(Self::WellShielded),
            b"normal" => Ok(Self::Normal),
            b"exposed" => Ok(Self::Exposed),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ShieldingofHome {}
#[derive(Clone, Debug, PartialEq)]
pub enum Siding {
    WoodSiding,
    Stucco,
    SyntheticStucco,
    VinylSiding,
    AluminumSiding,
    BrickVeneer,
    AsbestosSiding,
    FiberCementSiding,
    CompositeShingleSiding,
    MasoniteSiding,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Siding {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::WoodSiding => Ok(Some(::std::borrow::Cow::Borrowed("wood siding"))),
            Self::Stucco => Ok(Some(::std::borrow::Cow::Borrowed("stucco"))),
            Self::SyntheticStucco => Ok(Some(::std::borrow::Cow::Borrowed("synthetic stucco"))),
            Self::VinylSiding => Ok(Some(::std::borrow::Cow::Borrowed("vinyl siding"))),
            Self::AluminumSiding => Ok(Some(::std::borrow::Cow::Borrowed("aluminum siding"))),
            Self::BrickVeneer => Ok(Some(::std::borrow::Cow::Borrowed("brick veneer"))),
            Self::AsbestosSiding => Ok(Some(::std::borrow::Cow::Borrowed("asbestos siding"))),
            Self::FiberCementSiding => {
                Ok(Some(::std::borrow::Cow::Borrowed("fiber cement siding")))
            }
            Self::CompositeShingleSiding => Ok(Some(::std::borrow::Cow::Borrowed(
                "composite shingle siding",
            ))),
            Self::MasoniteSiding => Ok(Some(::std::borrow::Cow::Borrowed("masonite siding"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Siding {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Siding {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"wood siding" => Ok(Self::WoodSiding),
            b"stucco" => Ok(Self::Stucco),
            b"synthetic stucco" => Ok(Self::SyntheticStucco),
            b"vinyl siding" => Ok(Self::VinylSiding),
            b"aluminum siding" => Ok(Self::AluminumSiding),
            b"brick veneer" => Ok(Self::BrickVeneer),
            b"asbestos siding" => Ok(Self::AsbestosSiding),
            b"fiber cement siding" => Ok(Self::FiberCementSiding),
            b"composite shingle siding" => Ok(Self::CompositeShingleSiding),
            b"masonite siding" => Ok(Self::MasoniteSiding),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Siding {}
#[derive(Clone, Debug, PartialEq)]
pub enum SiteType {
    Rural,
    Suburban,
    Urban,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SiteType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Rural => Ok(Some(::std::borrow::Cow::Borrowed("rural"))),
            Self::Suburban => Ok(Some(::std::borrow::Cow::Borrowed("suburban"))),
            Self::Urban => Ok(Some(::std::borrow::Cow::Borrowed("urban"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SiteType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SiteType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"rural" => Ok(Self::Rural),
            b"suburban" => Ok(Self::Suburban),
            b"urban" => Ok(Self::Urban),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SiteType {}
pub type SoftwareProgramUsed = ::std::string::String;
pub type SoftwareProgramVersion = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct SolarAbsorptance(pub ::core::primitive::f64);
impl SolarAbsorptance {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "1",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<SolarAbsorptance> for ::core::primitive::f64 {
    fn from(value: SolarAbsorptance) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for SolarAbsorptance {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for SolarAbsorptance {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SolarAbsorptance {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SolarAbsorptance {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SolarAbsorptance {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SolarAbsorptance {}
#[derive(Clone, Debug, PartialEq)]
pub enum SolarThermalCollectorLoopType {
    AirDirect,
    AirIndirect,
    LiquidDirect,
    LiquidIndirect,
    PassiveThermosyphon,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SolarThermalCollectorLoopType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AirDirect => Ok(Some(::std::borrow::Cow::Borrowed("air direct"))),
            Self::AirIndirect => Ok(Some(::std::borrow::Cow::Borrowed("air indirect"))),
            Self::LiquidDirect => Ok(Some(::std::borrow::Cow::Borrowed("liquid direct"))),
            Self::LiquidIndirect => Ok(Some(::std::borrow::Cow::Borrowed("liquid indirect"))),
            Self::PassiveThermosyphon => {
                Ok(Some(::std::borrow::Cow::Borrowed("passive thermosyphon")))
            }
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SolarThermalCollectorLoopType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SolarThermalCollectorLoopType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"air direct" => Ok(Self::AirDirect),
            b"air indirect" => Ok(Self::AirIndirect),
            b"liquid direct" => Ok(Self::LiquidDirect),
            b"liquid indirect" => Ok(Self::LiquidIndirect),
            b"passive thermosyphon" => Ok(Self::PassiveThermosyphon),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SolarThermalCollectorLoopType {}
#[derive(Clone, Debug, PartialEq)]
pub enum SolarThermalCollectorType {
    SingleGlazingBlack,
    SingleGlazingSelective,
    DoubleGlazingBlack,
    DoubleGlazingSelective,
    EvacuatedTube,
    IntegratedCollectorStorage,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SolarThermalCollectorType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::SingleGlazingBlack => {
                Ok(Some(::std::borrow::Cow::Borrowed("single glazing black")))
            }
            Self::SingleGlazingSelective => Ok(Some(::std::borrow::Cow::Borrowed(
                "single glazing selective",
            ))),
            Self::DoubleGlazingBlack => {
                Ok(Some(::std::borrow::Cow::Borrowed("double glazing black")))
            }
            Self::DoubleGlazingSelective => Ok(Some(::std::borrow::Cow::Borrowed(
                "double glazing selective",
            ))),
            Self::EvacuatedTube => Ok(Some(::std::borrow::Cow::Borrowed("evacuated tube"))),
            Self::IntegratedCollectorStorage => Ok(Some(::std::borrow::Cow::Borrowed(
                "integrated collector storage",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SolarThermalCollectorType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SolarThermalCollectorType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"single glazing black" => Ok(Self::SingleGlazingBlack),
            b"single glazing selective" => Ok(Self::SingleGlazingSelective),
            b"double glazing black" => Ok(Self::DoubleGlazingBlack),
            b"double glazing selective" => Ok(Self::DoubleGlazingSelective),
            b"evacuated tube" => Ok(Self::EvacuatedTube),
            b"integrated collector storage" => Ok(Self::IntegratedCollectorStorage),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SolarThermalCollectorType {}
#[derive(Clone, Debug, PartialEq)]
pub enum SolarThermalSystemType {
    HotWater,
    HotWaterAndSpaceHeating,
    SpaceHeating,
    HybridSystem,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SolarThermalSystemType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::HotWater => Ok(Some(::std::borrow::Cow::Borrowed("hot water"))),
            Self::HotWaterAndSpaceHeating => Ok(Some(::std::borrow::Cow::Borrowed(
                "hot water and space heating",
            ))),
            Self::SpaceHeating => Ok(Some(::std::borrow::Cow::Borrowed("space heating"))),
            Self::HybridSystem => Ok(Some(::std::borrow::Cow::Borrowed("hybrid system"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SolarThermalSystemType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SolarThermalSystemType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"hot water" => Ok(Self::HotWater),
            b"hot water and space heating" => Ok(Self::HotWaterAndSpaceHeating),
            b"space heating" => Ok(Self::SpaceHeating),
            b"hybrid system" => Ok(Self::HybridSystem),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SolarThermalSystemType {}
#[derive(Clone, Debug, PartialEq)]
pub enum SpaceAboveGarage {
    ConditionedArea,
    UnconditionedAttic,
    Crawlspace,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SpaceAboveGarage {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ConditionedArea => Ok(Some(::std::borrow::Cow::Borrowed("conditioned area"))),
            Self::UnconditionedAttic => {
                Ok(Some(::std::borrow::Cow::Borrowed("unconditioned attic")))
            }
            Self::Crawlspace => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SpaceAboveGarage {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SpaceAboveGarage {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"conditioned area" => Ok(Self::ConditionedArea),
            b"unconditioned attic" => Ok(Self::UnconditionedAttic),
            b"crawlspace" => Ok(Self::Crawlspace),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SpaceAboveGarage {}
#[derive(Clone, Debug, PartialEq)]
pub struct Speed(pub ::core::primitive::f64);
impl Speed {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<Speed> for ::core::primitive::f64 {
    fn from(value: Speed) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Speed {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Speed {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Speed {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Speed {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Speed {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Speed {}
#[derive(Clone, Debug, PartialEq)]
pub enum SpotVentilationLocation {
    Kitchen,
    Bath,
    Garage,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SpotVentilationLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Kitchen => Ok(Some(::std::borrow::Cow::Borrowed("kitchen"))),
            Self::Bath => Ok(Some(::std::borrow::Cow::Borrowed("bath"))),
            Self::Garage => Ok(Some(::std::borrow::Cow::Borrowed("garage"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SpotVentilationLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SpotVentilationLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"kitchen" => Ok(Self::Kitchen),
            b"bath" => Ok(Self::Bath),
            b"garage" => Ok(Self::Garage),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SpotVentilationLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum SpotVentilationUnits {
    Cfm,
    Ach,
    LS,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SpotVentilationUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cfm => Ok(Some(::std::borrow::Cow::Borrowed("CFM"))),
            Self::Ach => Ok(Some(::std::borrow::Cow::Borrowed("ACH"))),
            Self::LS => Ok(Some(::std::borrow::Cow::Borrowed("L/s"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SpotVentilationUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SpotVentilationUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"CFM" => Ok(Self::Cfm),
            b"ACH" => Ok(Self::Ach),
            b"L/s" => Ok(Self::LS),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SpotVentilationUnits {}
pub type StartDate = ::std::string::String;
pub type StateCode = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum StaticPressureSource {
    AsMeasured,
    PerDesignReport,
    PerOemDocumentation,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for StaticPressureSource {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AsMeasured => Ok(Some(::std::borrow::Cow::Borrowed("as measured"))),
            Self::PerDesignReport => Ok(Some(::std::borrow::Cow::Borrowed("per design report"))),
            Self::PerOemDocumentation => {
                Ok(Some(::std::borrow::Cow::Borrowed("per OEM documentation")))
            }
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for StaticPressureSource {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for StaticPressureSource {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"as measured" => Ok(Self::AsMeasured),
            b"per design report" => Ok(Self::PerDesignReport),
            b"per OEM documentation" => Ok(Self::PerOemDocumentation),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for StaticPressureSource {}
#[derive(Clone, Debug, PartialEq)]
pub enum StudMaterial {
    Wood,
    Metal,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for StudMaterial {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Wood => Ok(Some(::std::borrow::Cow::Borrowed("wood"))),
            Self::Metal => Ok(Some(::std::borrow::Cow::Borrowed("metal"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for StudMaterial {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for StudMaterial {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"wood" => Ok(Self::Wood),
            b"metal" => Ok(Self::Metal),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for StudMaterial {}
#[derive(Clone, Debug, PartialEq)]
pub enum StudSize {
    _2X2,
    _2X3,
    _2X4,
    _2X6,
    _2X8,
    _2X10,
    _2X12,
    _2X14,
    _2X16,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for StudSize {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_2X2 => Ok(Some(::std::borrow::Cow::Borrowed("2x2"))),
            Self::_2X3 => Ok(Some(::std::borrow::Cow::Borrowed("2x3"))),
            Self::_2X4 => Ok(Some(::std::borrow::Cow::Borrowed("2x4"))),
            Self::_2X6 => Ok(Some(::std::borrow::Cow::Borrowed("2x6"))),
            Self::_2X8 => Ok(Some(::std::borrow::Cow::Borrowed("2x8"))),
            Self::_2X10 => Ok(Some(::std::borrow::Cow::Borrowed("2x10"))),
            Self::_2X12 => Ok(Some(::std::borrow::Cow::Borrowed("2x12"))),
            Self::_2X14 => Ok(Some(::std::borrow::Cow::Borrowed("2x14"))),
            Self::_2X16 => Ok(Some(::std::borrow::Cow::Borrowed("2x16"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for StudSize {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for StudSize {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"2x2" => Ok(Self::_2X2),
            b"2x3" => Ok(Self::_2X3),
            b"2x4" => Ok(Self::_2X4),
            b"2x6" => Ok(Self::_2X6),
            b"2x8" => Ok(Self::_2X8),
            b"2x10" => Ok(Self::_2X10),
            b"2x12" => Ok(Self::_2X12),
            b"2x14" => Ok(Self::_2X14),
            b"2x16" => Ok(Self::_2X16),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for StudSize {}
pub type SuffixName = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceArea(pub ::core::primitive::f64);
impl SurfaceArea {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<SurfaceArea> for ::core::primitive::f64 {
    fn from(value: SurfaceArea) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for SurfaceArea {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for SurfaceArea {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SurfaceArea {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SurfaceArea {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SurfaceArea {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SurfaceArea {}
#[derive(Clone, Debug, PartialEq)]
pub enum Surroundings {
    StandAlone,
    AttachedOnOneSide,
    AttachedOnTwoSides,
    AttachedOnThreeSides,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Surroundings {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::StandAlone => Ok(Some(::std::borrow::Cow::Borrowed("stand-alone"))),
            Self::AttachedOnOneSide => {
                Ok(Some(::std::borrow::Cow::Borrowed("attached on one side")))
            }
            Self::AttachedOnTwoSides => {
                Ok(Some(::std::borrow::Cow::Borrowed("attached on two sides")))
            }
            Self::AttachedOnThreeSides => Ok(Some(::std::borrow::Cow::Borrowed(
                "attached on three sides",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Surroundings {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Surroundings {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"stand-alone" => Ok(Self::StandAlone),
            b"attached on one side" => Ok(Self::AttachedOnOneSide),
            b"attached on two sides" => Ok(Self::AttachedOnTwoSides),
            b"attached on three sides" => Ok(Self::AttachedOnThreeSides),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Surroundings {}
pub type TelephoneExtension = ::std::string::String;
pub type TelephoneNumber = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum TelephoneTypeCode {
    Day,
    Evening,
    Mobile,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TelephoneTypeCode {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Day => Ok(Some(::std::borrow::Cow::Borrowed("day"))),
            Self::Evening => Ok(Some(::std::borrow::Cow::Borrowed("evening"))),
            Self::Mobile => Ok(Some(::std::borrow::Cow::Borrowed("mobile"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TelephoneTypeCode {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TelephoneTypeCode {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"day" => Ok(Self::Day),
            b"evening" => Ok(Self::Evening),
            b"mobile" => Ok(Self::Mobile),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TelephoneTypeCode {}
pub type Temperature = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum TermiteZone {
    NoneToSlight,
    SlightToModerate,
    ModerateToHeavy,
    VeryHeavy,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TermiteZone {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::NoneToSlight => Ok(Some(::std::borrow::Cow::Borrowed("none to slight"))),
            Self::SlightToModerate => Ok(Some(::std::borrow::Cow::Borrowed("slight to moderate"))),
            Self::ModerateToHeavy => Ok(Some(::std::borrow::Cow::Borrowed("moderate to heavy"))),
            Self::VeryHeavy => Ok(Some(::std::borrow::Cow::Borrowed("very heavy"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TermiteZone {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TermiteZone {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"none to slight" => Ok(Self::NoneToSlight),
            b"slight to moderate" => Ok(Self::SlightToModerate),
            b"moderate to heavy" => Ok(Self::ModerateToHeavy),
            b"very heavy" => Ok(Self::VeryHeavy),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TermiteZone {}
pub type TestDate = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum TestInorOut {
    TestIn,
    TestOut,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TestInorOut {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::TestIn => Ok(Some(::std::borrow::Cow::Borrowed("test in"))),
            Self::TestOut => Ok(Some(::std::borrow::Cow::Borrowed("test out"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TestInorOut {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TestInorOut {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"test in" => Ok(Self::TestIn),
            b"test out" => Ok(Self::TestOut),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TestInorOut {}
#[derive(Clone, Debug, PartialEq)]
pub enum TestResultType {
    Passed,
    Failed,
    NotTested,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TestResultType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Passed => Ok(Some(::std::borrow::Cow::Borrowed("passed"))),
            Self::Failed => Ok(Some(::std::borrow::Cow::Borrowed("failed"))),
            Self::NotTested => Ok(Some(::std::borrow::Cow::Borrowed("not tested"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TestResultType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TestResultType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"passed" => Ok(Self::Passed),
            b"failed" => Ok(Self::Failed),
            b"not tested" => Ok(Self::NotTested),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TestResultType {}
#[derive(Clone, Debug, PartialEq)]
pub struct ThermalEfficiency(pub ::core::primitive::f64);
impl ThermalEfficiency {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        if *value > 1f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "1",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<ThermalEfficiency> for ::core::primitive::f64 {
    fn from(value: ThermalEfficiency) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for ThermalEfficiency {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for ThermalEfficiency {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ThermalEfficiency {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ThermalEfficiency {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ThermalEfficiency {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ThermalEfficiency {}
#[derive(Clone, Debug, PartialEq)]
pub enum ThermostatType {
    ProgrammableThermostat,
    ManualThermostat,
    DigitalThermostat,
    Timer,
    Emcs,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ThermostatType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ProgrammableThermostat => Ok(Some(::std::borrow::Cow::Borrowed(
                "programmable thermostat",
            ))),
            Self::ManualThermostat => Ok(Some(::std::borrow::Cow::Borrowed("manual thermostat"))),
            Self::DigitalThermostat => Ok(Some(::std::borrow::Cow::Borrowed("digital thermostat"))),
            Self::Timer => Ok(Some(::std::borrow::Cow::Borrowed("timer"))),
            Self::Emcs => Ok(Some(::std::borrow::Cow::Borrowed("EMCS"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ThermostatType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ThermostatType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"programmable thermostat" => Ok(Self::ProgrammableThermostat),
            b"manual thermostat" => Ok(Self::ManualThermostat),
            b"digital thermostat" => Ok(Self::DigitalThermostat),
            b"timer" => Ok(Self::Timer),
            b"EMCS" => Ok(Self::Emcs),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ThermostatType {}
#[derive(Clone, Debug, PartialEq)]
pub struct Tilt(pub ::core::primitive::f64);
impl Tilt {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        if *value > 90f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::GraterThan(
                "90",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Tilt> for ::core::primitive::f64 {
    fn from(value: Tilt) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Tilt {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Tilt {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Tilt {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Tilt {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Tilt {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Tilt {}
pub type Title = ::std::string::String;
pub type TotalAmount = ::core::primitive::f64;
pub type TotalCostHealthSafetyMeasures = ::core::primitive::f64;
pub type TotalCostQualEnergyMeasures = ::core::primitive::f64;
#[derive(Clone, Debug, PartialEq)]
pub enum TransactionType {
    Create,
    Update,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TransactionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Create => Ok(Some(::std::borrow::Cow::Borrowed("create"))),
            Self::Update => Ok(Some(::std::borrow::Cow::Borrowed("update"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TransactionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TransactionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"create" => Ok(Self::Create),
            b"update" => Ok(Self::Update),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TransactionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum Treatments {
    WindowFilm,
    SolarScreen,
    Shading,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Treatments {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::WindowFilm => Ok(Some(::std::borrow::Cow::Borrowed("window film"))),
            Self::SolarScreen => Ok(Some(::std::borrow::Cow::Borrowed("solar screen"))),
            Self::Shading => Ok(Some(::std::borrow::Cow::Borrowed("shading"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Treatments {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Treatments {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"window film" => Ok(Self::WindowFilm),
            b"solar screen" => Ok(Self::SolarScreen),
            b"shading" => Ok(Self::Shading),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Treatments {}
#[derive(Clone, Debug, PartialEq)]
pub enum TypeofBlowerDoorTest {
    Pressurization,
    Depressurization,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TypeofBlowerDoorTest {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Pressurization => Ok(Some(::std::borrow::Cow::Borrowed("pressurization"))),
            Self::Depressurization => Ok(Some(::std::borrow::Cow::Borrowed("depressurization"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TypeofBlowerDoorTest {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TypeofBlowerDoorTest {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"pressurization" => Ok(Self::Pressurization),
            b"depressurization" => Ok(Self::Depressurization),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TypeofBlowerDoorTest {}
#[derive(Clone, Debug, PartialEq)]
pub enum TypeofInfiltrationMeasurement {
    BlowerDoor,
    TracerGas,
    Estimate,
    Checklist,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for TypeofInfiltrationMeasurement {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::BlowerDoor => Ok(Some(::std::borrow::Cow::Borrowed("blower door"))),
            Self::TracerGas => Ok(Some(::std::borrow::Cow::Borrowed("tracer gas"))),
            Self::Estimate => Ok(Some(::std::borrow::Cow::Borrowed("estimate"))),
            Self::Checklist => Ok(Some(::std::borrow::Cow::Borrowed("checklist"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for TypeofInfiltrationMeasurement {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for TypeofInfiltrationMeasurement {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"blower door" => Ok(Self::BlowerDoor),
            b"tracer gas" => Ok(Self::TracerGas),
            b"estimate" => Ok(Self::Estimate),
            b"checklist" => Ok(Self::Checklist),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for TypeofInfiltrationMeasurement {}
#[derive(Clone, Debug, PartialEq)]
pub struct UFactor(pub ::core::primitive::f64);
impl UFactor {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<UFactor> for ::core::primitive::f64 {
    fn from(value: UFactor) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for UFactor {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for UFactor {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for UFactor {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for UFactor {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for UFactor {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for UFactor {}
pub type UspsBarCode = ::std::string::String;
#[derive(Clone, Debug, PartialEq)]
pub enum UnitLocation {
    AtticConditioned,
    AtticUnconditioned,
    BasementConditioned,
    BasementUnconditioned,
    ConditionedSpace,
    CrawlspaceVented,
    CrawlspaceUnvented,
    GarageConditioned,
    GarageUnconditioned,
    MechanicalCloset,
    OtherInterior,
    OtherExterior,
    RoofDeck,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for UnitLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::AtticConditioned => Ok(Some(::std::borrow::Cow::Borrowed("attic - conditioned"))),
            Self::AtticUnconditioned => {
                Ok(Some(::std::borrow::Cow::Borrowed("attic - unconditioned")))
            }
            Self::BasementConditioned => {
                Ok(Some(::std::borrow::Cow::Borrowed("basement - conditioned")))
            }
            Self::BasementUnconditioned => Ok(Some(::std::borrow::Cow::Borrowed(
                "basement - unconditioned",
            ))),
            Self::ConditionedSpace => Ok(Some(::std::borrow::Cow::Borrowed("conditioned space"))),
            Self::CrawlspaceVented => Ok(Some(::std::borrow::Cow::Borrowed("crawlspace - vented"))),
            Self::CrawlspaceUnvented => {
                Ok(Some(::std::borrow::Cow::Borrowed("crawlspace - unvented")))
            }
            Self::GarageConditioned => {
                Ok(Some(::std::borrow::Cow::Borrowed("garage - conditioned")))
            }
            Self::GarageUnconditioned => {
                Ok(Some(::std::borrow::Cow::Borrowed("garage - unconditioned")))
            }
            Self::MechanicalCloset => Ok(Some(::std::borrow::Cow::Borrowed("mechanical closet"))),
            Self::OtherInterior => Ok(Some(::std::borrow::Cow::Borrowed("other interior"))),
            Self::OtherExterior => Ok(Some(::std::borrow::Cow::Borrowed("other exterior"))),
            Self::RoofDeck => Ok(Some(::std::borrow::Cow::Borrowed("roof deck"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for UnitLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for UnitLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"attic - conditioned" => Ok(Self::AtticConditioned),
            b"attic - unconditioned" => Ok(Self::AtticUnconditioned),
            b"basement - conditioned" => Ok(Self::BasementConditioned),
            b"basement - unconditioned" => Ok(Self::BasementUnconditioned),
            b"conditioned space" => Ok(Self::ConditionedSpace),
            b"crawlspace - vented" => Ok(Self::CrawlspaceVented),
            b"crawlspace - unvented" => Ok(Self::CrawlspaceUnvented),
            b"garage - conditioned" => Ok(Self::GarageConditioned),
            b"garage - unconditioned" => Ok(Self::GarageUnconditioned),
            b"mechanical closet" => Ok(Self::MechanicalCloset),
            b"other interior" => Ok(Self::OtherInterior),
            b"other exterior" => Ok(Self::OtherExterior),
            b"roof deck" => Ok(Self::RoofDeck),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for UnitLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum VentSystem {
    Atmospheric,
    InducedDraft,
    PowerVentedAtUnit,
    PowerVentedAtExterior,
    DirectVented,
    SealedCombustion,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VentSystem {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Atmospheric => Ok(Some(::std::borrow::Cow::Borrowed("atmospheric"))),
            Self::InducedDraft => Ok(Some(::std::borrow::Cow::Borrowed("induced draft"))),
            Self::PowerVentedAtUnit => {
                Ok(Some(::std::borrow::Cow::Borrowed("power vented (at unit)")))
            }
            Self::PowerVentedAtExterior => Ok(Some(::std::borrow::Cow::Borrowed(
                "power vented (at exterior)",
            ))),
            Self::DirectVented => Ok(Some(::std::borrow::Cow::Borrowed("direct vented"))),
            Self::SealedCombustion => Ok(Some(::std::borrow::Cow::Borrowed("sealed combustion"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VentSystem {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VentSystem {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"atmospheric" => Ok(Self::Atmospheric),
            b"induced draft" => Ok(Self::InducedDraft),
            b"power vented (at unit)" => Ok(Self::PowerVentedAtUnit),
            b"power vented (at exterior)" => Ok(Self::PowerVentedAtExterior),
            b"direct vented" => Ok(Self::DirectVented),
            b"sealed combustion" => Ok(Self::SealedCombustion),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for VentSystem {}
#[derive(Clone, Debug, PartialEq)]
pub enum VentilationFanLocation {
    Bath,
    Kitchen,
    Hallway,
    Garage,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VentilationFanLocation {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Bath => Ok(Some(::std::borrow::Cow::Borrowed("bath"))),
            Self::Kitchen => Ok(Some(::std::borrow::Cow::Borrowed("kitchen"))),
            Self::Hallway => Ok(Some(::std::borrow::Cow::Borrowed("hallway"))),
            Self::Garage => Ok(Some(::std::borrow::Cow::Borrowed("garage"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VentilationFanLocation {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VentilationFanLocation {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"bath" => Ok(Self::Bath),
            b"kitchen" => Ok(Self::Kitchen),
            b"hallway" => Ok(Self::Hallway),
            b"garage" => Ok(Self::Garage),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for VentilationFanLocation {}
#[derive(Clone, Debug, PartialEq)]
pub enum VentilationFanThirdPartyCertification {
    EnergyStar,
    HomeVentilationInstitute,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VentilationFanThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::HomeVentilationInstitute => Ok(Some(::std::borrow::Cow::Borrowed(
                "Home Ventilation Institute",
            ))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VentilationFanThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VentilationFanThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"Home Ventilation Institute" => Ok(Self::HomeVentilationInstitute),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for VentilationFanThirdPartyCertification
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum VentilationFanType {
    ExhaustOnly,
    SupplyOnly,
    HeatRecoveryVentilator,
    EnergyRecoveryVentilator,
    Balanced,
    CentralFanIntegratedSupply,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VentilationFanType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::ExhaustOnly => Ok(Some(::std::borrow::Cow::Borrowed("exhaust only"))),
            Self::SupplyOnly => Ok(Some(::std::borrow::Cow::Borrowed("supply only"))),
            Self::HeatRecoveryVentilator => Ok(Some(::std::borrow::Cow::Borrowed(
                "heat recovery ventilator",
            ))),
            Self::EnergyRecoveryVentilator => Ok(Some(::std::borrow::Cow::Borrowed(
                "energy recovery ventilator",
            ))),
            Self::Balanced => Ok(Some(::std::borrow::Cow::Borrowed("balanced"))),
            Self::CentralFanIntegratedSupply => Ok(Some(::std::borrow::Cow::Borrowed(
                "central fan integrated supply",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VentilationFanType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VentilationFanType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"exhaust only" => Ok(Self::ExhaustOnly),
            b"supply only" => Ok(Self::SupplyOnly),
            b"heat recovery ventilator" => Ok(Self::HeatRecoveryVentilator),
            b"energy recovery ventilator" => Ok(Self::EnergyRecoveryVentilator),
            b"balanced" => Ok(Self::Balanced),
            b"central fan integrated supply" => Ok(Self::CentralFanIntegratedSupply),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for VentilationFanType {}
#[derive(Clone, Debug, PartialEq)]
pub enum VentilationRateUnits {
    Ach,
    CfMnat,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VentilationRateUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Ach => Ok(Some(::std::borrow::Cow::Borrowed("ACH"))),
            Self::CfMnat => Ok(Some(::std::borrow::Cow::Borrowed("CFMnat"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VentilationRateUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VentilationRateUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"ACH" => Ok(Self::Ach),
            b"CFMnat" => Ok(Self::CfMnat),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for VentilationRateUnits {}
#[derive(Clone, Debug, PartialEq)]
pub enum VerticalSurroundings {
    UnitAbove,
    UnitBelow,
    UnitAboveAndBelow,
    NoUnitsAboveOrBelow,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for VerticalSurroundings {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::UnitAbove => Ok(Some(::std::borrow::Cow::Borrowed("unit above"))),
            Self::UnitBelow => Ok(Some(::std::borrow::Cow::Borrowed("unit below"))),
            Self::UnitAboveAndBelow => {
                Ok(Some(::std::borrow::Cow::Borrowed("unit above and below")))
            }
            Self::NoUnitsAboveOrBelow => Ok(Some(::std::borrow::Cow::Borrowed(
                "no units above or below",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for VerticalSurroundings {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for VerticalSurroundings {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"unit above" => Ok(Self::UnitAbove),
            b"unit below" => Ok(Self::UnitBelow),
            b"unit above and below" => Ok(Self::UnitAboveAndBelow),
            b"no units above or below" => Ok(Self::NoUnitsAboveOrBelow),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for VerticalSurroundings {}
#[derive(Clone, Debug, PartialEq)]
pub struct Volume(pub ::core::primitive::f64);
impl Volume {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value <= 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessEqualThan(
                "0",
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<Volume> for ::core::primitive::f64 {
    fn from(value: Volume) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for Volume {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for Volume {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for Volume {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for Volume {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for Volume {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for Volume {}
#[derive(Clone, Debug, PartialEq)]
pub enum WallAndRoofColor {
    Light,
    Medium,
    MediumDark,
    Dark,
    Reflective,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WallAndRoofColor {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Light => Ok(Some(::std::borrow::Cow::Borrowed("light"))),
            Self::Medium => Ok(Some(::std::borrow::Cow::Borrowed("medium"))),
            Self::MediumDark => Ok(Some(::std::borrow::Cow::Borrowed("medium dark"))),
            Self::Dark => Ok(Some(::std::borrow::Cow::Borrowed("dark"))),
            Self::Reflective => Ok(Some(::std::borrow::Cow::Borrowed("reflective"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WallAndRoofColor {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WallAndRoofColor {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"light" => Ok(Self::Light),
            b"medium" => Ok(Self::Medium),
            b"medium dark" => Ok(Self::MediumDark),
            b"dark" => Ok(Self::Dark),
            b"reflective" => Ok(Self::Reflective),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WallAndRoofColor {}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterFixtureThirdPartyCertification {
    EnergyStar,
    EnergyStarMostEfficient,
    WaterSense,
    CeeTier1,
    CeeTier2,
    CeeTier3,
    Other,
    Unknown,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterFixtureThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::EnergyStarMostEfficient => Ok(Some(::std::borrow::Cow::Borrowed(
                "Energy Star Most Efficient",
            ))),
            Self::WaterSense => Ok(Some(::std::borrow::Cow::Borrowed("WaterSense"))),
            Self::CeeTier1 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 1"))),
            Self::CeeTier2 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 2"))),
            Self::CeeTier3 => Ok(Some(::std::borrow::Cow::Borrowed("CEE Tier 3"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
            Self::Unknown => Ok(Some(::std::borrow::Cow::Borrowed("unknown"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterFixtureThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterFixtureThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"Energy Star Most Efficient" => Ok(Self::EnergyStarMostEfficient),
            b"WaterSense" => Ok(Self::WaterSense),
            b"CEE Tier 1" => Ok(Self::CeeTier1),
            b"CEE Tier 2" => Ok(Self::CeeTier2),
            b"CEE Tier 3" => Ok(Self::CeeTier3),
            b"other" => Ok(Self::Other),
            b"unknown" => Ok(Self::Unknown),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for WaterFixtureThirdPartyCertification
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterFixtureType {
    Faucet,
    ShowerHead,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterFixtureType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Faucet => Ok(Some(::std::borrow::Cow::Borrowed("faucet"))),
            Self::ShowerHead => Ok(Some(::std::borrow::Cow::Borrowed("shower head"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterFixtureType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterFixtureType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"faucet" => Ok(Self::Faucet),
            b"shower head" => Ok(Self::ShowerHead),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WaterFixtureType {}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterHeaterType {
    StorageWaterHeater,
    DedicatedBoilerWithStorageTank,
    InstantaneousWaterHeater,
    HeatPumpWaterHeater,
    SpaceHeatingBoilerWithStorageTank,
    SpaceHeatingBoilerWithTanklessCoil,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterHeaterType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::StorageWaterHeater => {
                Ok(Some(::std::borrow::Cow::Borrowed("storage water heater")))
            }
            Self::DedicatedBoilerWithStorageTank => Ok(Some(::std::borrow::Cow::Borrowed(
                "dedicated boiler with storage tank",
            ))),
            Self::InstantaneousWaterHeater => Ok(Some(::std::borrow::Cow::Borrowed(
                "instantaneous water heater",
            ))),
            Self::HeatPumpWaterHeater => {
                Ok(Some(::std::borrow::Cow::Borrowed("heat pump water heater")))
            }
            Self::SpaceHeatingBoilerWithStorageTank => Ok(Some(::std::borrow::Cow::Borrowed(
                "space-heating boiler with storage tank",
            ))),
            Self::SpaceHeatingBoilerWithTanklessCoil => Ok(Some(::std::borrow::Cow::Borrowed(
                "space-heating boiler with tankless coil",
            ))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterHeaterType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterHeaterType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"storage water heater" => Ok(Self::StorageWaterHeater),
            b"dedicated boiler with storage tank" => Ok(Self::DedicatedBoilerWithStorageTank),
            b"instantaneous water heater" => Ok(Self::InstantaneousWaterHeater),
            b"heat pump water heater" => Ok(Self::HeatPumpWaterHeater),
            b"space-heating boiler with storage tank" => {
                Ok(Self::SpaceHeatingBoilerWithStorageTank)
            }
            b"space-heating boiler with tankless coil" => {
                Ok(Self::SpaceHeatingBoilerWithTanklessCoil)
            }
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WaterHeaterType {}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterType {
    IndoorAndOutdoorWater,
    IndoorWater,
    OutdoorWater,
    WastewaterSewer,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::IndoorAndOutdoorWater => Ok(Some(::std::borrow::Cow::Borrowed(
                "indoor and outdoor water",
            ))),
            Self::IndoorWater => Ok(Some(::std::borrow::Cow::Borrowed("indoor water"))),
            Self::OutdoorWater => Ok(Some(::std::borrow::Cow::Borrowed("outdoor water"))),
            Self::WastewaterSewer => Ok(Some(::std::borrow::Cow::Borrowed("wastewater/sewer"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"indoor and outdoor water" => Ok(Self::IndoorAndOutdoorWater),
            b"indoor water" => Ok(Self::IndoorWater),
            b"outdoor water" => Ok(Self::OutdoorWater),
            b"wastewater/sewer" => Ok(Self::WastewaterSewer),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WaterType {}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterUseIntensityUnits {
    GalSqFt,
    GalDayPerson,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterUseIntensityUnits {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::GalSqFt => Ok(Some(::std::borrow::Cow::Borrowed("gal/sq.ft."))),
            Self::GalDayPerson => Ok(Some(::std::borrow::Cow::Borrowed("gal/day/person"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterUseIntensityUnits {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterUseIntensityUnits {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"gal/sq.ft." => Ok(Self::GalSqFt),
            b"gal/day/person" => Ok(Self::GalDayPerson),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WaterUseIntensityUnits {}
#[derive(Clone, Debug, PartialEq)]
pub enum WeatherStationType {
    Tmy,
    Tmy2,
    Tmy3,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WeatherStationType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Tmy => Ok(Some(::std::borrow::Cow::Borrowed("TMY"))),
            Self::Tmy2 => Ok(Some(::std::borrow::Cow::Borrowed("TMY2"))),
            Self::Tmy3 => Ok(Some(::std::borrow::Cow::Borrowed("TMY3"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WeatherStationType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WeatherStationType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"TMY" => Ok(Self::Tmy),
            b"TMY2" => Ok(Self::Tmy2),
            b"TMY3" => Ok(Self::Tmy3),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WeatherStationType {}
#[derive(Clone, Debug, PartialEq)]
pub enum WeatherStationUse {
    BillingAnalysis,
    EnergyModeling,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WeatherStationUse {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::BillingAnalysis => Ok(Some(::std::borrow::Cow::Borrowed("billing analysis"))),
            Self::EnergyModeling => Ok(Some(::std::borrow::Cow::Borrowed("energy modeling"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WeatherStationUse {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WeatherStationUse {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"billing analysis" => Ok(Self::BillingAnalysis),
            b"energy modeling" => Ok(Self::EnergyModeling),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WeatherStationUse {}
#[derive(Clone, Debug, PartialEq)]
pub enum WholeBldgVentilationRequirementMethod {
    Ashrae6221989,
    Ashrae6222007,
    Ashrae6222010,
    Ashrae6222013,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WholeBldgVentilationRequirementMethod {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Ashrae6221989 => Ok(Some(::std::borrow::Cow::Borrowed("ASHRAE 62.2-1989"))),
            Self::Ashrae6222007 => Ok(Some(::std::borrow::Cow::Borrowed("ASHRAE 62.2-2007"))),
            Self::Ashrae6222010 => Ok(Some(::std::borrow::Cow::Borrowed("ASHRAE 62.2-2010"))),
            Self::Ashrae6222013 => Ok(Some(::std::borrow::Cow::Borrowed("ASHRAE 62.2-2013"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WholeBldgVentilationRequirementMethod {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WholeBldgVentilationRequirementMethod {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"ASHRAE 62.2-1989" => Ok(Self::Ashrae6221989),
            b"ASHRAE 62.2-2007" => Ok(Self::Ashrae6222007),
            b"ASHRAE 62.2-2010" => Ok(Self::Ashrae6222010),
            b"ASHRAE 62.2-2013" => Ok(Self::Ashrae6222013),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes
    for WholeBldgVentilationRequirementMethod
{
}
#[derive(Clone, Debug, PartialEq)]
pub enum WindConditions {
    Windy,
    Normal,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WindConditions {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Windy => Ok(Some(::std::borrow::Cow::Borrowed("windy"))),
            Self::Normal => Ok(Some(::std::borrow::Cow::Borrowed("normal"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WindConditions {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WindConditions {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"windy" => Ok(Self::Windy),
            b"normal" => Ok(Self::Normal),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WindConditions {}
#[derive(Clone, Debug, PartialEq)]
pub enum WindThirdPartyCertification {
    Awea912009,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WindThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Awea912009 => Ok(Some(::std::borrow::Cow::Borrowed("AWEA 9.1-2009"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WindThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WindThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"AWEA 9.1-2009" => Ok(Self::Awea912009),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WindThirdPartyCertification {}
#[derive(Clone, Debug, PartialEq)]
pub enum WindowCondition {
    Good,
    Moderate,
    Poor,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WindowCondition {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Good => Ok(Some(::std::borrow::Cow::Borrowed("good"))),
            Self::Moderate => Ok(Some(::std::borrow::Cow::Borrowed("moderate"))),
            Self::Poor => Ok(Some(::std::borrow::Cow::Borrowed("poor"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WindowCondition {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WindowCondition {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"good" => Ok(Self::Good),
            b"moderate" => Ok(Self::Moderate),
            b"poor" => Ok(Self::Poor),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WindowCondition {}
#[derive(Clone, Debug, PartialEq)]
pub enum WindowThirdPartyCertification {
    EnergyStar,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WindowThirdPartyCertification {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::EnergyStar => Ok(Some(::std::borrow::Cow::Borrowed("Energy Star"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WindowThirdPartyCertification {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WindowThirdPartyCertification {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Energy Star" => Ok(Self::EnergyStar),
            b"other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WindowThirdPartyCertification {}
pub type XmlGeneratedBy = ::std::string::String;
pub type XmlType = ::std::string::String;
pub type Year = ::core::primitive::i32;
#[derive(Clone, Debug, PartialEq)]
pub struct ZipCode(pub ::std::string::String);
impl ZipCode {
    pub fn new(
        inner: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::std::string::String {
        self.0
    }
    pub fn validate_str(
        s: &str,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        static PATTERNS: ::std::sync::LazyLock<
            [(&::core::primitive::str, ::regex::Regex); 1usize],
        > = ::std::sync::LazyLock::new(|| {
            [(
                "[0-9]{5}(-[0-9]{4})?",
                ::regex::Regex::new("^(?:[0-9]{5}(-[0-9]{4})?)$").unwrap(),
            )]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(::xsd_parser_types::quick_xml::ValidateError::Pattern(
                PATTERNS[0usize].0,
            ));
        }
        Ok(())
    }
}
impl ::core::convert::From<ZipCode> for ::std::string::String {
    fn from(value: ZipCode) -> ::std::string::String {
        value.0
    }
}
impl ::core::convert::TryFrom<::std::string::String> for ZipCode {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for ZipCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ZipCode {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ZipCode {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ZipCode {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let s =
            ::core::str::from_utf8(bytes).map_err(::xsd_parser_types::quick_xml::Error::from)?;
        Self::validate_str(s).map_err(|error| (bytes, error))?;
        let inner = ::std::string::String::deserialize_str(helper, s)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ZipCode {}
#[derive(Clone, Debug, PartialEq)]
pub enum ZoneType {
    Conditioned,
    Unconditioned,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for ZoneType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Conditioned => Ok(Some(::std::borrow::Cow::Borrowed("conditioned"))),
            Self::Unconditioned => Ok(Some(::std::borrow::Cow::Borrowed("unconditioned"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for ZoneType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for ZoneType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"conditioned" => Ok(Self::Conditioned),
            b"unconditioned" => Ok(Self::Unconditioned),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for ZoneType {}
#[derive(Clone, Debug, PartialEq)]
pub enum EGridRegions {
    Alaska,
    Eastern,
    Ercot,
    Hawaii,
    Western,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EGridRegions {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Alaska => Ok(Some(::std::borrow::Cow::Borrowed("Alaska"))),
            Self::Eastern => Ok(Some(::std::borrow::Cow::Borrowed("Eastern"))),
            Self::Ercot => Ok(Some(::std::borrow::Cow::Borrowed("ERCOT"))),
            Self::Hawaii => Ok(Some(::std::borrow::Cow::Borrowed("Hawaii"))),
            Self::Western => Ok(Some(::std::borrow::Cow::Borrowed("Western"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EGridRegions {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EGridRegions {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Alaska" => Ok(Self::Alaska),
            b"Eastern" => Ok(Self::Eastern),
            b"ERCOT" => Ok(Self::Ercot),
            b"Hawaii" => Ok(Self::Hawaii),
            b"Western" => Ok(Self::Western),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EGridRegions {}
#[derive(Clone, Debug, PartialEq)]
pub enum EndUseType {
    Heating,
    Cooling,
    HotWater,
    Appliance,
    Lighting,
    Pv,
    SolarThermal,
    Other,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EndUseType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Heating => Ok(Some(::std::borrow::Cow::Borrowed("Heating"))),
            Self::Cooling => Ok(Some(::std::borrow::Cow::Borrowed("Cooling"))),
            Self::HotWater => Ok(Some(::std::borrow::Cow::Borrowed("HotWater"))),
            Self::Appliance => Ok(Some(::std::borrow::Cow::Borrowed("Appliance"))),
            Self::Lighting => Ok(Some(::std::borrow::Cow::Borrowed("Lighting"))),
            Self::Pv => Ok(Some(::std::borrow::Cow::Borrowed("PV"))),
            Self::SolarThermal => Ok(Some(::std::borrow::Cow::Borrowed("SolarThermal"))),
            Self::Other => Ok(Some(::std::borrow::Cow::Borrowed("Other"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EndUseType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EndUseType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"Heating" => Ok(Self::Heating),
            b"Cooling" => Ok(Self::Cooling),
            b"HotWater" => Ok(Self::HotWater),
            b"Appliance" => Ok(Self::Appliance),
            b"Lighting" => Ok(Self::Lighting),
            b"PV" => Ok(Self::Pv),
            b"SolarThermal" => Ok(Self::SolarThermal),
            b"Other" => Ok(Self::Other),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EndUseType {}
#[derive(Clone, Debug, PartialEq)]
pub enum EnergyUnitType {
    Cmh,
    Ccf,
    Kcf,
    Mcf,
    Cfh,
    KWh,
    MWh,
    Btu,
    KBtu,
    MBtu,
    Therms,
    Lbs,
    KLbs,
    MLbs,
    Tonnes,
    Cords,
    Gal,
    Kgal,
    TonHours,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for EnergyUnitType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Cmh => Ok(Some(::std::borrow::Cow::Borrowed("cmh"))),
            Self::Ccf => Ok(Some(::std::borrow::Cow::Borrowed("ccf"))),
            Self::Kcf => Ok(Some(::std::borrow::Cow::Borrowed("kcf"))),
            Self::Mcf => Ok(Some(::std::borrow::Cow::Borrowed("Mcf"))),
            Self::Cfh => Ok(Some(::std::borrow::Cow::Borrowed("cfh"))),
            Self::KWh => Ok(Some(::std::borrow::Cow::Borrowed("kWh"))),
            Self::MWh => Ok(Some(::std::borrow::Cow::Borrowed("MWh"))),
            Self::Btu => Ok(Some(::std::borrow::Cow::Borrowed("Btu"))),
            Self::KBtu => Ok(Some(::std::borrow::Cow::Borrowed("kBtu"))),
            Self::MBtu => Ok(Some(::std::borrow::Cow::Borrowed("MBtu"))),
            Self::Therms => Ok(Some(::std::borrow::Cow::Borrowed("therms"))),
            Self::Lbs => Ok(Some(::std::borrow::Cow::Borrowed("lbs"))),
            Self::KLbs => Ok(Some(::std::borrow::Cow::Borrowed("kLbs"))),
            Self::MLbs => Ok(Some(::std::borrow::Cow::Borrowed("MLbs"))),
            Self::Tonnes => Ok(Some(::std::borrow::Cow::Borrowed("tonnes"))),
            Self::Cords => Ok(Some(::std::borrow::Cow::Borrowed("cords"))),
            Self::Gal => Ok(Some(::std::borrow::Cow::Borrowed("gal"))),
            Self::Kgal => Ok(Some(::std::borrow::Cow::Borrowed("kgal"))),
            Self::TonHours => Ok(Some(::std::borrow::Cow::Borrowed("ton hours"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for EnergyUnitType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for EnergyUnitType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"cmh" => Ok(Self::Cmh),
            b"ccf" => Ok(Self::Ccf),
            b"kcf" => Ok(Self::Kcf),
            b"Mcf" => Ok(Self::Mcf),
            b"cfh" => Ok(Self::Cfh),
            b"kWh" => Ok(Self::KWh),
            b"MWh" => Ok(Self::MWh),
            b"Btu" => Ok(Self::Btu),
            b"kBtu" => Ok(Self::KBtu),
            b"MBtu" => Ok(Self::MBtu),
            b"therms" => Ok(Self::Therms),
            b"lbs" => Ok(Self::Lbs),
            b"kLbs" => Ok(Self::KLbs),
            b"MLbs" => Ok(Self::MLbs),
            b"tonnes" => Ok(Self::Tonnes),
            b"cords" => Ok(Self::Cords),
            b"gal" => Ok(Self::Gal),
            b"kgal" => Ok(Self::Kgal),
            b"ton hours" => Ok(Self::TonHours),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for EnergyUnitType {}
#[derive(Clone, Debug, PartialEq)]
pub struct KWperTon(pub ::core::primitive::f64);
impl KWperTon {
    pub fn new(
        inner: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> ::core::primitive::f64 {
        self.0
    }
    pub fn validate_value(
        value: &::core::primitive::f64,
    ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::ValidateError> {
        if *value < 0f64 {
            return Err(::xsd_parser_types::quick_xml::ValidateError::LessThan("0"));
        }
        Ok(())
    }
}
impl ::core::convert::From<KWperTon> for ::core::primitive::f64 {
    fn from(value: KWperTon) -> ::core::primitive::f64 {
        value.0
    }
}
impl ::core::convert::TryFrom<::core::primitive::f64> for KWperTon {
    type Error = ::xsd_parser_types::quick_xml::ValidateError;
    fn try_from(
        value: ::core::primitive::f64,
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::ValidateError> {
        Self::new(value)
    }
}
impl ::core::ops::Deref for KWperTon {
    type Target = ::core::primitive::f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for KWperTon {
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
impl xsd_parser_types::quick_xml::WithSerializeToBytes for KWperTon {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for KWperTon {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        let inner = ::core::primitive::f64::deserialize_bytes(helper, bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for KWperTon {}
#[derive(Clone, Debug, PartialEq)]
pub enum SchemaVersionType {
    _20,
    _21,
    _22,
    _221,
    _23,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for SchemaVersionType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::_20 => Ok(Some(::std::borrow::Cow::Borrowed("2.0"))),
            Self::_21 => Ok(Some(::std::borrow::Cow::Borrowed("2.1"))),
            Self::_22 => Ok(Some(::std::borrow::Cow::Borrowed("2.2"))),
            Self::_221 => Ok(Some(::std::borrow::Cow::Borrowed("2.2.1"))),
            Self::_23 => Ok(Some(::std::borrow::Cow::Borrowed("2.3"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for SchemaVersionType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for SchemaVersionType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"2.0" => Ok(Self::_20),
            b"2.1" => Ok(Self::_21),
            b"2.2" => Ok(Self::_22),
            b"2.2.1" => Ok(Self::_221),
            b"2.3" => Ok(Self::_23),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for SchemaVersionType {}
#[derive(Clone, Debug, PartialEq)]
pub enum WaterUnitType {
    Gal,
    Kgal,
    Mgal,
    Cf,
    Ccf,
    Kcf,
    Mcf,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes for WaterUnitType {
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Gal => Ok(Some(::std::borrow::Cow::Borrowed("gal"))),
            Self::Kgal => Ok(Some(::std::borrow::Cow::Borrowed("kgal"))),
            Self::Mgal => Ok(Some(::std::borrow::Cow::Borrowed("Mgal"))),
            Self::Cf => Ok(Some(::std::borrow::Cow::Borrowed("cf"))),
            Self::Ccf => Ok(Some(::std::borrow::Cow::Borrowed("ccf"))),
            Self::Kcf => Ok(Some(::std::borrow::Cow::Borrowed("kcf"))),
            Self::Mcf => Ok(Some(::std::borrow::Cow::Borrowed("Mcf"))),
        }
    }
}
impl xsd_parser_types::quick_xml::WithSerializeToBytes for WaterUnitType {}
impl ::xsd_parser_types::quick_xml::DeserializeBytes for WaterUnitType {
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"gal" => Ok(Self::Gal),
            b"kgal" => Ok(Self::Kgal),
            b"Mgal" => Ok(Self::Mgal),
            b"cf" => Ok(Self::Cf),
            b"ccf" => Ok(Self::Ccf),
            b"kcf" => Ok(Self::Kcf),
            b"Mcf" => Ok(Self::Mcf),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
impl xsd_parser_types::quick_xml::WithDeserializerFromBytes for WaterUnitType {}
