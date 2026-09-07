//! Parse an HPXML v4 file with custom safety limits and demonstrate error handling.
//!
//! Usage:
//!   cargo run -p hpxml-core --features v4 --example safe_parse -- <path> [max_mib]
//!
//! Example:
//!   cargo run -p hpxml-core --features v4 --example safe_parse -- crates/hpxml-core/tests/data/v4/audit.xml 1

#[cfg(feature = "v4")]
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(feature = "v4")]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    use hpxml_core::{ParseConfig, ParseError};

    let path = std::env::args()
        .nth(1)
        .ok_or("usage: safe_parse <path> [max_mib]")?;

    let max_mib: usize = std::env::args()
        .nth(2)
        .map(|s| s.parse().map_err(|_| format!("invalid max_mib: {s:?}")))
        .transpose()?
        .unwrap_or(10);

    let bytes = std::fs::read(&path).map_err(|e| format!("failed to read {path}: {e}"))?;
    println!("File size: {} bytes", bytes.len());

    let config = ParseConfig {
        max_bytes: max_mib.saturating_mul(1024 * 1024),
        max_depth: 64,
        ..ParseConfig::default()
    };
    println!(
        "Limits: max_bytes={}, max_depth={}",
        config.max_bytes, config.max_depth
    );

    match hpxml_core::v4::parse_with_config(&bytes, &config) {
        Ok(doc) => {
            println!("Parsed successfully: {} buildings", doc.building.len());
            Ok(())
        }
        Err(ParseError::DocumentTooLarge { size, limit }) => {
            Err(format!("rejected: document is {size} bytes, limit is {limit} bytes").into())
        }
        Err(ParseError::DepthLimitExceeded { depth, limit }) => {
            Err(format!("rejected: XML depth {depth} exceeds limit {limit}").into())
        }
        Err(ParseError::DtdNotAllowed) => {
            Err("rejected: document contains a DTD declaration".into())
        }
        Err(ParseError::Xml { message, position }) => {
            if let Some(pos) = position {
                Err(format!("XML error at byte {pos}: {message}").into())
            } else {
                Err(format!("XML error: {message}").into())
            }
        }
        Err(e) => Err(format!("parse error: {e}").into()),
    }
}

#[cfg(not(feature = "v4"))]
fn main() {
    eprintln!("This example requires the v4 feature: --features v4");
    std::process::exit(1);
}
