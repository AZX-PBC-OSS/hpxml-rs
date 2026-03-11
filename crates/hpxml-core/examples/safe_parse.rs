//! Parse an HPXML v4 file with custom safety limits and demonstrate error handling.
//!
//! Usage:
//!   cargo run -p hpxml-core --features v4 --example safe_parse -- <path> [max_mb]
//!
//! Example:
//!   cargo run -p hpxml-core --features v4 --example safe_parse -- crates/hpxml-core/tests/data/v4/audit.xml 1

#[cfg(feature = "v4")]
fn main() {
    use hpxml_core::{ParseConfig, ParseError};

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Usage: safe_parse <path> [max_mb]"));

    let max_mb: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
    println!("File size: {} bytes", bytes.len());

    let config = ParseConfig {
        max_bytes: max_mb * 1_000_000,
        max_depth: 64,
    };
    println!(
        "Limits: max_bytes={}, max_depth={}",
        config.max_bytes, config.max_depth
    );

    match hpxml_core::v4::parse_with_config(&bytes, &config) {
        Ok(doc) => {
            println!("Parsed successfully: {} buildings", doc.building.len());
        }
        Err(ParseError::DocumentTooLarge { size, limit }) => {
            eprintln!("Rejected: document is {size} bytes, limit is {limit} bytes");
            std::process::exit(1);
        }
        Err(ParseError::DepthLimitExceeded { depth, limit }) => {
            eprintln!("Rejected: XML depth {depth} exceeds limit {limit}");
            std::process::exit(1);
        }
        Err(ParseError::DtdNotAllowed) => {
            eprintln!("Rejected: document contains a DTD declaration");
            std::process::exit(1);
        }
        Err(ParseError::Xml { message, position }) => {
            if let Some(pos) = position {
                eprintln!("XML error at byte {pos}: {message}");
            } else {
                eprintln!("XML error: {message}");
            }
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "v4"))]
fn main() {
    eprintln!("This example requires the v4 feature: --features v4");
    std::process::exit(1);
}
