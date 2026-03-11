//! Parse an HPXML v4 file and print building information.
//!
//! Usage:
//!   cargo run -p hpxml-core --features v4 --example parse_file -- <path>
//!
//! Example:
//!   cargo run -p hpxml-core --features v4 --example parse_file -- crates/hpxml-core/tests/data/v4/audit.xml

#[cfg(feature = "v4")]
fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Usage: parse_file <path-to-hpxml-v4.xml>"));

    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
    let doc = hpxml_core::v4::parse(&bytes).unwrap_or_else(|e| panic!("Parse error: {e}"));

    println!("Schema version: {:?}", doc.schema_version);
    println!("Buildings: {}", doc.building.len());

    for (i, building) in doc.building.iter().enumerate() {
        match building.as_ref() {
            Some(b) => {
                println!(
                    "  Building {}: id={}, event={:?}",
                    i, b.building_id.id, b.project_status.event_type.content
                );
            }
            None => println!("  Building {}: nil", i),
        }
    }

    if !doc.customer.is_empty() {
        println!("Customers: {}", doc.customer.len());
        for (i, customer) in doc.customer.iter().enumerate() {
            if let Some(c) = customer.as_ref() {
                println!(
                    "  Customer {}: {} {}",
                    i,
                    c.customer_details.person.name.first_name.content,
                    c.customer_details.person.name.last_name.content
                );
            }
        }
    }

    if !doc.contractor.is_empty() {
        println!("Contractors: {}", doc.contractor.len());
    }
}

#[cfg(not(feature = "v4"))]
fn main() {
    eprintln!("This example requires the v4 feature: --features v4");
    std::process::exit(1);
}
