# hpxml-rs

Rust-native parser and serializer for [HPXML](https://www.hpxmlonline.com/) (Home Performance XML), the residential building energy data standard used by DOE, ENERGY STAR, and weatherization programs.

All types are generated from the official XSD schemas via [xsd-parser](https://github.com/Bergmann89/xsd-parser). No handwritten type definitions.

## Quick start

```toml
[dependencies]
hpxml = "0.1"
```

v4 is the default. For other versions:

```toml
hpxml = { version = "0.1", default-features = false, features = ["v3"] }
```

Available features: `v2`, `v3`, `v4` (default), `v5`, `full` (all versions).

## Usage

### Parse an HPXML file

```rust
use hpxml::{v4, HpxmlSerialize};

let bytes = std::fs::read("audit.xml")?;
let doc = v4::parse(&bytes)?;

for building in &doc.building {
    if let Some(b) = building.as_ref() {
        println!("Building: {}", b.building_id.id);
    }
}
```

### Inspect version before parsing

```rust
use hpxml::inspect::inspect;

let bytes = std::fs::read("unknown_version.xml")?;
let info = inspect(&bytes)?;

println!("Version: {:?}, Schema: {}", info.version, info.schema_version);

match info.version {
    hpxml::HpxmlVersion::V4 => {
        let doc = hpxml::v4::parse(&bytes)?;
        // ...
    }
    _ => eprintln!("Unsupported version"),
}
```

### Modify and serialize

```rust
use hpxml::{v4, HpxmlSerialize, Nillable};

let bytes = std::fs::read("audit.xml")?;
let mut doc = v4::parse(&bytes)?;

// Remove optional sections
if let Some(b) = doc.building[0].as_mut() {
    b.building_details.lighting = None;
    b.building_details.appliances = None;
}

// Serialize back to XML
let xml_bytes = doc.to_xml()?;
std::fs::write("modified.xml", &xml_bytes)?;

// Or write directly to any std::io::Write
let file = std::fs::File::create("output.xml")?;
let mut writer = std::io::BufWriter::new(file);
doc.to_xml_into(&mut writer)?;
```

### Parse with safety limits

```rust
use hpxml::{v4, ParseConfig};

let bytes = std::fs::read("audit.xml")?;
let config = ParseConfig {
    max_bytes: 10_000_000,  // 10 MB limit
    max_depth: 64,          // XML nesting depth limit
    ..Default::default()
};
let doc = v4::parse_with_config(&bytes, &config)?;
```

### Error handling

All operations return typed errors that can be matched for specific failure modes:

```rust
use hpxml::{v4, ParseError, ParseConfig};

let bytes = std::fs::read("audit.xml")?;
let config = ParseConfig { max_bytes: 1_000, max_depth: 64, ..Default::default() };

match v4::parse_with_config(&bytes, &config) {
    Ok(doc) => println!("Parsed {} buildings", doc.building.len()),
    Err(ParseError::DocumentTooLarge { size, limit }) => {
        eprintln!("File too large: {size} bytes (limit: {limit})");
    }
    Err(ParseError::DepthLimitExceeded { depth, limit }) => {
        eprintln!("XML too deeply nested: depth {depth} (limit: {limit})");
    }
    Err(ParseError::DtdNotAllowed) => {
        eprintln!("DTD declarations are rejected for security");
    }
    Err(ParseError::Xml { message, position }) => {
        eprintln!("XML parse error: {message} at position {position:?}");
    }
    Err(e) => eprintln!("Other error: {e}"),
}
```

Three error types cover the full API:
- `InspectError`: returned by `inspect()` (unknown namespace, missing version, malformed XML, DTD)
- `ParseError`: returned by `parse()` / `parse_with_config()` (size/depth limits, XML errors, DTD)
- `SerializeError`: returned by `to_xml()` / `to_xml_into()` (XML serialization, I/O)

## Schema versions

| Feature | HPXML Version | Schema Versions |
|---------|--------------|-----------------|
| `v2`    | 2.x          | 2.0 - 2.3.1    |
| `v3`    | 3.x          | 3.0 - 3.1      |
| `v4`    | 4.x          | 4.0 - 4.2      |
| `v5`    | 5.x          | 5.0rc1+        |

## Development

### Prerequisites

- Rust 1.86+ (see `rust-toolchain.toml`)
- `rustfmt` (included with Rust)
- `curl` and `jq` (only for fetching XSD schemas via `./scripts/codegen.sh`)

### Code generation

Types are generated from XSD schemas and committed to the repo. To regenerate:

```bash
# Generate all versions (fetches schemas automatically)
./scripts/codegen.sh

# Generate specific versions
./scripts/codegen.sh v4
```

Generated code lives in `crates/hpxml-types-v{N}/src/`.

### Building and testing

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
# Fast loop on the core crate:
cargo test -p hpxml-core --features full
```

### Runnable examples

```bash
# Parse a v4 file and print building info
cargo run -p hpxml-core --features v4 --example parse_file -- crates/hpxml-core/tests/data/v4/audit.xml

# Detect the version of any HPXML file, then parse it
cargo run -p hpxml-core --features full --example inspect_file -- crates/hpxml-core/tests/data/v3/audit.xml

# Parse, modify, serialize, and verify round-trip equality
cargo run -p hpxml-core --features v4 --example roundtrip -- crates/hpxml-core/tests/data/v4/audit.xml

# Parse with custom size limits and error handling
cargo run -p hpxml-core --features v4 --example safe_parse -- crates/hpxml-core/tests/data/v4/audit.xml 1
```

## Documentation

- [Architecture](docs/architecture.md): crate structure, design decisions
- [Code generation](docs/codegen.md): XSD codegen workflow and configuration
- [Releasing](docs/releasing.md): publishing checklist, semver policy, CI
- [Contributing](CONTRIBUTING.md): setup, workflow, PR guidelines
- [Security](SECURITY.md): vulnerability reporting and supply-chain posture
- [Code of Conduct](CODE_OF_CONDUCT.md)

## License

BSD-3-Clause
