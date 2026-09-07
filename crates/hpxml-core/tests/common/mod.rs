macro_rules! fixture {
    ($path:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $path))
    };
}

pub mod io_fail;

/// Strips the unconditional `xmlns:xsi` declaration emitted by xsd-parser
/// codegen on elements with `xs:any` content.
///
/// The declaration is a serialization artifact, not document data: re-parsing
/// without it must yield a struct-equal document. If upstream xsd-parser ever
/// stops emitting it, `serializer_emits_xsi_on_any_extensions` fails and this
/// helper (plus its callers) should be removed.
/// See <https://github.com/Bergmann89/xsd-parser/issues/256>.
///
/// Only the round-trip target uses this; the other integration binaries still
/// compile this shared module, so unused use must not warn.
#[allow(dead_code)]
pub fn normalize_xsi(xml: &[u8]) -> Vec<u8> {
    const MARKER: &[u8] = b" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"";
    let mut out = Vec::with_capacity(xml.len());
    let mut rest = xml;
    while let Some(pos) = rest.windows(MARKER.len()).position(|w| w == MARKER) {
        out.extend_from_slice(&rest[..pos]);
        rest = &rest[pos + MARKER.len()..];
    }
    out.extend_from_slice(rest);
    out
}
