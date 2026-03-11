// GENERATED CODE - do not edit manually.
// Regenerate with: scripts/codegen.sh
#[path = "hpxml.rs"]
pub mod hpxml;
#[path = "hpxml_base_elements.rs"]
pub mod hpxml_base_elements;
#[path = "hpxml_data_types.rs"]
pub mod hpxml_data_types;
#[path = "xs.rs"]
pub mod xs;
pub use hpxml::HpxmlElementType as HpxmlType;
pub const NS_XS: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_UNNAMED_5: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://hpxmlonline.com/2025/12");
pub const PREFIX_XS: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xsi");
