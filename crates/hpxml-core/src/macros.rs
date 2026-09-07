//! Version module macro for HPXML types.
//!
//! This macro generates `parse()`, `parse_with_config()`, and implements
//! `HpxmlSerialize` for each version's root type.

/// Implements version-specific parse functions and serialization for HPXML types.
///
/// This macro creates:
/// - `parse(bytes: &[u8])` - Parse with default config
/// - `parse_with_config(bytes: &[u8], config: &ParseConfig)` - Parse with custom config
/// - `impl HpxmlSerialize for $root_type` - Serialization implementation
///
/// # Arguments
///
/// - `$version`: The version module identifier (e.g., `v4`)
/// - `$root_type`: The root type from the generated code (e.g., `HpxmlType`)
#[doc(hidden)]
#[macro_export]
macro_rules! impl_version_module {
    ($version:ident, $root_type:ident) => {
        /// Parse an HPXML document with default configuration.
        ///
        /// # Errors
        ///
        /// Returns [`$crate::ParseError`] if parsing fails.
        pub fn parse(bytes: &[u8]) -> Result<$root_type, $crate::ParseError> {
            parse_with_config(bytes, &$crate::ParseConfig::default())
        }

        /// Parse an HPXML document with custom configuration.
        ///
        /// # Errors
        ///
        /// Returns [`$crate::ParseError`] if:
        /// - Document exceeds size limit (`DocumentTooLarge`)
        /// - Document contains a DTD declaration (`DtdNotAllowed`)
        /// - XML depth exceeds limit (`DepthLimitExceeded`)
        /// - XML parsing fails (`Xml`)
        pub fn parse_with_config(
            bytes: &[u8],
            config: &$crate::ParseConfig,
        ) -> Result<$root_type, $crate::ParseError> {
            let size = bytes.len();
            if size > config.max_bytes {
                return Err($crate::ParseError::DocumentTooLarge {
                    size,
                    limit: config.max_bytes,
                });
            }

            if $crate::dtd_guard::has_doctype(bytes) {
                return Err($crate::ParseError::DtdNotAllowed);
            }

            let mut reader = $crate::reader::DepthLimitedReader::new(bytes, *config);

            use xsd_parser_types::quick_xml::DeserializeSync;
            let result = <$root_type>::deserialize(&mut reader);

            result.map_err(|e| match &e.kind {
                xsd_parser_types::quick_xml::ErrorKind::Custom(source) => source
                    .as_ref()
                    .downcast_ref::<$crate::ParseError>()
                    .cloned()
                    .unwrap_or_else(|| $crate::ParseError::Xml {
                        message: e.to_string(),
                        position: e.position.and_then(|p| usize::try_from(p).ok()),
                    }),
                _ => $crate::ParseError::Xml {
                    message: e.to_string(),
                    position: e.position.and_then(|p| usize::try_from(p).ok()),
                },
            })
        }

        impl $crate::HpxmlSerialize for $root_type {
            fn to_xml(&self) -> Result<Vec<u8>, $crate::SerializeError> {
                self.to_xml_with_options(&$crate::SerializeOptions::default())
            }

            fn to_xml_into(
                &self,
                w: &mut impl std::io::Write,
            ) -> Result<(), $crate::SerializeError> {
                self.to_xml_into_with_options(w, &$crate::SerializeOptions::default())
            }

            fn to_xml_with_options(
                &self,
                options: &$crate::SerializeOptions,
            ) -> Result<Vec<u8>, $crate::SerializeError> {
                use xsd_parser_types::quick_xml::{SerializeSync, Writer};
                let mut buffer = Vec::new();
                {
                    let mut writer = Writer::new(&mut buffer);
                    if options.xml_declaration {
                        use quick_xml::events::{BytesDecl, Event};
                        writer
                            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
                            .map_err(|e| $crate::SerializeError::Xml(e.to_string()))?;
                    }
                    self.serialize("HPXML", &mut writer)
                        .map_err(|e| $crate::SerializeError::Xml(e.to_string()))?;
                }
                Ok(buffer)
            }

            fn to_xml_into_with_options(
                &self,
                w: &mut impl std::io::Write,
                options: &$crate::SerializeOptions,
            ) -> Result<(), $crate::SerializeError> {
                use xsd_parser_types::quick_xml::{SerializeSync, Writer};
                let mut writer = Writer::new(w);
                if options.xml_declaration {
                    use quick_xml::events::{BytesDecl, Event};
                    writer
                        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
                        .map_err(|e| $crate::SerializeError::Xml(e.to_string()))?;
                }
                self.serialize("HPXML", &mut writer)
                    .map_err(|e| $crate::SerializeError::Xml(e.to_string()))?;
                writer
                    .into_inner()
                    .flush()
                    .map_err($crate::SerializeError::Io)?;
                Ok(())
            }
        }
    };
}
