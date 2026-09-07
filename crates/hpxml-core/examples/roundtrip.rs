//! Parse an HPXML v4 file, modify it, serialize back to XML, and verify the round-trip.
//!
//! Usage:
//!   cargo run -p hpxml-core --features v4 --example roundtrip -- <path>
//!
//! Example:
//!   cargo run -p hpxml-core --features v4 --example roundtrip -- crates/hpxml-core/tests/data/v4/audit.xml

#[cfg(feature = "v4")]
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(feature = "v4")]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    use hpxml_core::HpxmlSerialize;

    let path = std::env::args()
        .nth(1)
        .ok_or("usage: roundtrip <path-to-hpxml-v4.xml>")?;

    let bytes = std::fs::read(&path).map_err(|e| format!("failed to read {path}: {e}"))?;

    // Parse
    let mut doc = hpxml_core::v4::parse(&bytes).map_err(|e| format!("parse error: {e}"))?;
    println!("Parsed: {} buildings", doc.building.len());

    // Modify: strip optional sections from the first building
    if let Some(b) = doc.building.first_mut().and_then(|b| b.as_mut()) {
        println!(
            "Stripping optional fields from building '{}'",
            b.building_id.id
        );
        b.building_details.lighting = None;
        b.building_details.appliances = None;
        b.building_details.pools = None;
        b.building_details.spas = None;
    }

    // Serialize to XML bytes
    let xml_bytes = doc.to_xml().map_err(|e| format!("serialize error: {e}"))?;
    println!("Serialized: {} bytes", xml_bytes.len());

    // Re-parse and verify structural equality
    let reparsed = hpxml_core::v4::parse(&xml_bytes).map_err(|e| format!("re-parse error: {e}"))?;

    if doc == reparsed {
        println!("Round-trip OK: documents are structurally equal");
        Ok(())
    } else {
        Err("round-trip FAILED: documents differ after re-parse".into())
    }
}

#[cfg(not(feature = "v4"))]
fn main() {
    eprintln!("This example requires the v4 feature: --features v4");
    std::process::exit(1);
}
