//! Error types for HPXML parsing and inspection.

use std::error::Error;
use std::fmt;

/// Error type for HPXML document inspection (pre-parsing validation).
#[derive(Debug)]
#[non_exhaustive]
pub enum InspectError {
    /// The namespace and schema version do not match.
    NamespaceVersionMismatch {
        /// The schema version from the document.
        schema_version: String,
        /// The namespace URI from the document.
        namespace: String,
    },
    /// The namespace URI is not recognized.
    UnknownNamespace(String),
    /// The `<HPXML>` element is missing the `xmlns` attribute.
    MissingNamespace,
    /// The `schemaVersion` attribute is absent from the document.
    MissingSchemaVersion,
    /// The XML is malformed.
    MalformedXml(String),
    /// The document exceeds the configured size limit.
    DocumentTooLarge {
        /// Actual document size in bytes.
        size: usize,
        /// Configured size limit in bytes.
        limit: usize,
    },
    /// The root element is not an HPXML document.
    NotHpxml {
        /// The element found at the root.
        found_element: String,
    },
    /// The document contains a DTD declaration, which is not permitted.
    DtdNotAllowed,
}

impl fmt::Display for InspectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InspectError::NamespaceVersionMismatch {
                schema_version,
                namespace,
            } => {
                write!(
                    f,
                    "namespace '{}' does not match schema version '{}'",
                    namespace, schema_version
                )
            }
            InspectError::UnknownNamespace(ns) => {
                write!(f, "unknown HPXML namespace: {}", ns)
            }
            InspectError::MissingNamespace => {
                write!(f, "missing xmlns attribute on <HPXML> element")
            }
            InspectError::MissingSchemaVersion => {
                write!(f, "missing schemaVersion attribute")
            }
            InspectError::MalformedXml(msg) => {
                write!(f, "malformed XML: {}", msg)
            }
            InspectError::DocumentTooLarge { size, limit } => {
                write!(
                    f,
                    "document too large: {} bytes (limit: {} bytes)",
                    size, limit
                )
            }
            InspectError::NotHpxml { found_element } => {
                write!(
                    f,
                    "not an HPXML document: root element is <{}>",
                    found_element
                )
            }
            InspectError::DtdNotAllowed => {
                write!(f, "DTD declarations are not allowed")
            }
        }
    }
}

impl Error for InspectError {}

/// Error type for HPXML document parsing.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ParseError {
    /// The document exceeds the configured size limit.
    DocumentTooLarge {
        /// Actual document size in bytes.
        size: usize,
        /// Configured size limit in bytes.
        limit: usize,
    },
    /// The XML nesting depth exceeds the configured limit.
    DepthLimitExceeded {
        /// Actual nesting depth.
        depth: usize,
        /// Configured depth limit.
        limit: usize,
    },
    /// The document contains a DTD declaration, which is not permitted.
    DtdNotAllowed,
    /// XML parsing error with optional position.
    Xml {
        /// Error message from the XML parser.
        message: String,
        /// Byte offset where parsing failed (if available).
        position: Option<usize>,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::DocumentTooLarge { size, limit } => {
                write!(
                    f,
                    "document too large: {} bytes (limit: {} bytes)",
                    size, limit
                )
            }
            ParseError::DepthLimitExceeded { depth, limit } => {
                write!(f, "XML depth {} exceeds limit {}", depth, limit)
            }
            ParseError::DtdNotAllowed => {
                write!(f, "DTD declarations are not allowed")
            }
            ParseError::Xml { message, position } => {
                if let Some(pos) = position {
                    write!(f, "XML error at position {}: {}", pos, message)
                } else {
                    write!(f, "XML error: {}", message)
                }
            }
        }
    }
}

impl Error for ParseError {}

/// Error type for HPXML document serialization.
#[derive(Debug)]
#[non_exhaustive]
pub enum SerializeError {
    /// XML serialization error.
    Xml(String),
    /// I/O error during serialization.
    Io(std::io::Error),
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::Xml(msg) => {
                write!(f, "XML serialization error: {}", msg)
            }
            SerializeError::Io(e) => {
                write!(f, "I/O error: {}", e)
            }
        }
    }
}

impl Error for SerializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerializeError::Io(e) => Some(e),
            SerializeError::Xml(_) => None,
        }
    }
}

impl From<std::io::Error> for SerializeError {
    fn from(e: std::io::Error) -> Self {
        SerializeError::Io(e)
    }
}
