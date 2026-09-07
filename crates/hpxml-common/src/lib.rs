//! Common types for HPXML processing.
//!
//! This crate provides shared types used across the HPXML ecosystem:
//! - Error types ([`InspectError`], [`ParseError`], [`SerializeError`])
//! - Parsing configuration ([`ParseConfig`])
//! - Version types ([`HpxmlVersion`], [`HpxmlFileInfo`])

pub mod config;
pub mod error;
pub mod version;

pub use config::{ParseConfig, SerializeOptions};
pub use error::{InspectError, ParseError, SerializeError};
#[doc(hidden)]
pub use version::version_from_namespace;
pub use version::{HpxmlFileInfo, HpxmlVersion};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_sync_error_types() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<InspectError>();
        assert_send_sync::<ParseError>();
        assert_send_sync::<SerializeError>();
    }
}
