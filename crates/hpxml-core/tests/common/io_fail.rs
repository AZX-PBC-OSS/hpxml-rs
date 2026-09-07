//! Shared I/O-failure helpers for integration tests.
//!
//! These live here (rather than in `src/test_helpers.rs`) because integration
//! tests in `tests/` cannot see `#[cfg(test)]` items inside the library.
//! The unit-test twins in `src/test_helpers.rs` are intentionally separate:
//! that split is forced by Cargo's test visibility rules, not copy-paste.

use std::io::{Error, ErrorKind, Result};

/// A writer that fails on every `write` (flush succeeds).
///
/// Serialization writes go through quick-xml first, so a write failure
/// surfaces as [`hpxml_core::SerializeError::Xml`].
///
/// Only some integration binaries use these helpers; the rest still compile
/// this shared module, so unused construction must not warn.
#[allow(dead_code)]
pub struct FailingWriter;

impl std::io::Write for FailingWriter {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "write failed"))
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// A writer that accepts writes but fails on `flush`.
///
/// Bytes are already serialized when `flush` runs, so this surfaces as
/// [`hpxml_core::SerializeError::Io`].
#[allow(dead_code)]
pub struct FailingFlusher {
    /// Sink for accepted bytes.
    pub buf: Vec<u8>,
}

impl std::io::Write for FailingFlusher {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "flush failed"))
    }
}
