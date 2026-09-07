//! Serialization traits for HPXML types.

use hpxml_common::{SerializeError, SerializeOptions};

/// Trait for serializing HPXML types to XML.
///
/// This trait is implemented for each version's root type via the
/// `impl_version_module!` macro. Callers can use `to_xml()` or
/// `to_xml_into()` to serialize an HPXML document.
///
/// # Example
///
/// ```no_run
/// # #![allow(unused_imports)]
/// use hpxml_core::{HpxmlSerialize, SerializeOptions};
///
/// # #[cfg(feature = "v4")]
/// # fn demo(doc: hpxml_core::v4::HpxmlType) -> Result<(), hpxml_core::SerializeError> {
/// let xml_bytes = doc.to_xml()?;
/// doc.to_xml_into(&mut std::io::Cursor::new(Vec::new()))?;
///
/// // With XML declaration
/// let opts = SerializeOptions { xml_declaration: true };
/// let declared = doc.to_xml_with_options(&opts)?;
/// # Ok(())
/// # }
/// ```
pub trait HpxmlSerialize {
    /// Serializes the HPXML document to a byte vector.
    ///
    /// Returns XML bytes without an XML declaration (`<?xml?>`).
    ///
    /// # Errors
    ///
    /// Returns [`SerializeError::Xml`] if serialization fails.
    fn to_xml(&self) -> Result<Vec<u8>, SerializeError>;

    /// Serializes the HPXML document into the provided writer.
    ///
    /// Does not emit an XML declaration (`<?xml?>`).
    ///
    /// # Errors
    ///
    /// Returns [`SerializeError::Xml`] if serialization fails,
    /// or [`SerializeError::Io`] if the writer fails.
    fn to_xml_into(&self, w: &mut impl std::io::Write) -> Result<(), SerializeError>;

    /// Serializes the HPXML document to a byte vector with the given options.
    ///
    /// # Errors
    ///
    /// Returns [`SerializeError::Xml`] if serialization fails.
    fn to_xml_with_options(&self, options: &SerializeOptions) -> Result<Vec<u8>, SerializeError>;

    /// Serializes the HPXML document into the provided writer with the given options.
    ///
    /// # Errors
    ///
    /// Returns [`SerializeError::Xml`] if serialization fails,
    /// or [`SerializeError::Io`] if the writer fails.
    fn to_xml_into_with_options(
        &self,
        w: &mut impl std::io::Write,
        options: &SerializeOptions,
    ) -> Result<(), SerializeError>;
}
