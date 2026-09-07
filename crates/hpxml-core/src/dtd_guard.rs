//! Pre-parse guard that rejects documents containing a DTD (DOCTYPE declaration).
//!
//! quick-xml 0.38 does not expand entities or fetch external resources, but it
//! does parse and emit `Event::DocType` without error. Downstream code that calls
//! `decode_and_unescape_value` will fail on unknown entities, but only after
//! expending work. This guard short-circuits at the byte level before any XML
//! parsing begins, covering all three DTD attack classes:
//!
//! - Billion laughs (internal entity expansion bombs)
//! - XXE (external entity references)
//! - Inline DTD subset (element/attribute declarations)
//!
//! The scan is case-sensitive: the XML 1.0 spec requires `DOCTYPE` in
//! upper-case, so only the upper-case marker declares a DTD. A lower-case
//! variant is skipped by the parser and never produces `Event::DocType`;
//! `lowercase_doctype_is_not_a_dtd` pins that behavior.

/// Scan the full input for a `<!DOCTYPE` marker.
///
/// Returns `true` if a DTD declaration is present, `false` otherwise.
/// The input is already bounded by `ParseConfig::max_bytes`, so a full
/// scan stays O(n) relative to work parsing would do anyway. A fixed
/// window would let an attacker pad the prolog with comments/PIs/
/// whitespace (all legal before DOCTYPE per XML 1.0 §2.8) to push the
/// declaration past the window.
pub(crate) fn has_doctype(bytes: &[u8]) -> bool {
    const MARKER: &[u8] = b"<!DOCTYPE";

    if bytes.len() < MARKER.len() {
        return false;
    }
    bytes.windows(MARKER.len()).any(|chunk| chunk == MARKER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_billion_laughs() {
        let xml = include_bytes!("../tests/data/negative/billion-laughs.xml");
        assert!(has_doctype(xml));
    }

    #[test]
    fn detects_xxe() {
        let xml = include_bytes!("../tests/data/negative/xxe.xml");
        assert!(has_doctype(xml));
    }

    #[test]
    fn detects_dtd_internal() {
        let xml = include_bytes!("../tests/data/negative/dtd-internal.xml");
        assert!(has_doctype(xml));
    }

    #[test]
    fn clean_document_passes() {
        let xml = br#"<?xml version="1.0"?>
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0"/>"#;
        assert!(!has_doctype(xml));
    }

    #[test]
    fn doctype_marker_in_comment_is_rejected_conservatively() {
        // The guard is a byte scan without XML comment awareness, so a marker
        // inside a comment is still rejected. No legitimate HPXML document
        // mentions DOCTYPE in a pre-root comment, so the false-positive rate
        // on real inputs is zero.
        let xml = br#"<?xml version="1.0"?>
<!-- <!DOCTYPE fake> -->
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0"/>"#;
        assert!(has_doctype(xml));
    }

    #[test]
    fn doctype_after_long_prolog_is_detected() {
        let padding = vec![b' '; 4096];
        let mut xml = b"<?xml version=\"1.0\"?>".to_vec();
        xml.extend_from_slice(&padding);
        xml.extend_from_slice(b"<!DOCTYPE late []>");
        assert!(has_doctype(&xml));
    }

    #[test]
    fn lowercase_doctype_is_not_a_dtd() {
        let xml = b"<?xml version=\"1.0\"?><!doctype HPXML []>";
        assert!(!has_doctype(xml));
    }
}
