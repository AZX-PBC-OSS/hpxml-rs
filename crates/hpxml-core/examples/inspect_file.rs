//! Inspect an HPXML file to determine its version before parsing.
//!
//! Usage:
//!   cargo run -p hpxml-core --features full --example inspect_file -- <path>
//!
//! Example:
//!   cargo run -p hpxml-core --features full --example inspect_file -- crates/hpxml-core/tests/data/v4/audit.xml

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Usage: inspect_file <path-to-hpxml.xml>"));

    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));

    let info = match hpxml_core::inspect::inspect(&bytes) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Inspect error: {e}");
            std::process::exit(1);
        }
    };

    println!("Version:        {:?}", info.version);
    println!("Schema version: {}", info.schema_version);

    // Dispatch to the correct versioned parser
    match info.version {
        #[cfg(feature = "v2")]
        hpxml_core::HpxmlVersion::V2 => {
            let doc = hpxml_core::v2::parse(&bytes).expect("v2 parse failed");
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v3")]
        hpxml_core::HpxmlVersion::V3 => {
            let doc = hpxml_core::v3::parse(&bytes).expect("v3 parse failed");
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v4")]
        hpxml_core::HpxmlVersion::V4 => {
            let doc = hpxml_core::v4::parse(&bytes).expect("v4 parse failed");
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v5")]
        hpxml_core::HpxmlVersion::V5 => {
            let doc = hpxml_core::v5::parse(&bytes).expect("v5 parse failed");
            println!("Buildings:      {}", doc.building.len());
        }
        _ => {
            eprintln!("Version {:?} not enabled in features", info.version);
            std::process::exit(1);
        }
    }
}
