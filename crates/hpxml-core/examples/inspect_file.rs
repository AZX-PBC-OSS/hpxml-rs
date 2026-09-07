//! Inspect an HPXML file to determine its version before parsing.
//!
//! Usage:
//!   cargo run -p hpxml-core --features full --example inspect_file -- <path>
//!
//! Example:
//!   cargo run -p hpxml-core --features full --example inspect_file -- crates/hpxml-core/tests/data/v4/audit.xml

#[cfg(any(
    feature = "v2",
    feature = "v3",
    feature = "v4",
    feature = "v5"
))]
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(any(
    feature = "v2",
    feature = "v3",
    feature = "v4",
    feature = "v5"
)))]
fn main() {
    eprintln!("This example requires a version feature: --features v4 (or full)");
    std::process::exit(1);
}

#[cfg(any(
    feature = "v2",
    feature = "v3",
    feature = "v4",
    feature = "v5"
))]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: inspect_file <path-to-hpxml.xml>")?;

    let bytes = std::fs::read(&path).map_err(|e| format!("failed to read {path}: {e}"))?;

    let info = hpxml_core::inspect::inspect(&bytes).map_err(|e| format!("inspect error: {e}"))?;

    println!("Version:        {:?}", info.version);
    println!("Schema version: {}", info.schema_version);

    // Dispatch to the correct versioned parser
    match info.version {
        #[cfg(feature = "v2")]
        hpxml_core::HpxmlVersion::V2 => {
            let doc = hpxml_core::v2::parse(&bytes).map_err(|e| format!("v2 parse failed: {e}"))?;
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v3")]
        hpxml_core::HpxmlVersion::V3 => {
            let doc = hpxml_core::v3::parse(&bytes).map_err(|e| format!("v3 parse failed: {e}"))?;
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v4")]
        hpxml_core::HpxmlVersion::V4 => {
            let doc = hpxml_core::v4::parse(&bytes).map_err(|e| format!("v4 parse failed: {e}"))?;
            println!("Buildings:      {}", doc.building.len());
        }
        #[cfg(feature = "v5")]
        hpxml_core::HpxmlVersion::V5 => {
            let doc = hpxml_core::v5::parse(&bytes).map_err(|e| format!("v5 parse failed: {e}"))?;
            println!("Buildings:      {}", doc.building.len());
        }
        _ => {
            return Err(format!("version {:?} not enabled in features", info.version).into());
        }
    }
    Ok(())
}
