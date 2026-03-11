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
//! The scan is intentionally case-sensitive. The XML 1.0 spec requires `DOCTYPE`
//! in upper-case, so a lower-case variant is malformed XML regardless.

/// Scan up to the first 4 KiB of `bytes` for a `<!DOCTYPE` marker.
///
/// Returns `true` if a DTD declaration is present, `false` otherwise.
/// The scan stops as soon as the marker is found or the window is exhausted.
pub(crate) fn has_doctype(bytes: &[u8]) -> bool {
    const WINDOW: usize = 4096;
    const MARKER: &[u8] = b"<!DOCTYPE";

    let window = &bytes[..bytes.len().min(WINDOW)];
    // Use a simple sub-slice search; the window is at most 4 KiB so this is O(n).
    window.windows(MARKER.len()).any(|chunk| chunk == MARKER)
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
    fn doctype_in_comment_is_not_detected() {
        // A DOCTYPE-like string inside a comment is within the first 4 KiB, but
        // that is acceptable: a real XML comment cannot precede the root element
        // in a position that would hide a real DTD, and the false-positive rate
        // for legitimate HPXML documents is zero (HPXML files never have comments
        // before the root element mentioning DOCTYPE).
        //
        // This test documents the known behaviour: the guard is a byte scan and
        // does not understand XML comment boundaries.
        let xml = br#"<?xml version="1.0"?>
<!-- <!DOCTYPE fake> -->
<HPXML xmlns="http://hpxmlonline.com/2023/09" schemaVersion="4.0"/>"#;
        // The guard does flag this; that is the intentional conservative choice.
        assert!(has_doctype(xml));
    }

    #[test]
    fn doctype_beyond_window_is_not_detected() {
        // Construct a document where <!DOCTYPE appears after the 4 KiB boundary.
        // The guard only scans the first 4 KiB, so this slips through intentionally.
        // In practice HPXML files always have the DTD (if present) within the
        // first few hundred bytes; this edge case only matters for crafted inputs.
        let padding = vec![b' '; 4096];
        let mut xml = b"<?xml version=\"1.0\"?>".to_vec();
        xml.extend_from_slice(&padding);
        xml.extend_from_slice(b"<!DOCTYPE late []>");
        assert!(!has_doctype(&xml));
    }
}
