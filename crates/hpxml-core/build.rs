use std::ffi::OsStr;
use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

/// Collects the stem names of parsable fixture files for a version directory.
///
/// Files named `invalid.xml` are excluded: they must fail parsing and are
/// covered by dedicated rejection tests instead.
fn fixture_stems(version_dir: &Path) -> Vec<String> {
    let entries = fs::read_dir(version_dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", version_dir.display()));
    let mut stems: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry
                .unwrap_or_else(|e| {
                    panic!("failed to read entry in {}: {e}", version_dir.display())
                })
                .path();
            if path.extension() != Some(OsStr::new("xml"))
                || path.file_stem() == Some(OsStr::new("invalid"))
            {
                return None;
            }
            let stem = path
                .file_stem()
                .unwrap_or_else(|| panic!("missing file stem for {}", path.display()))
                .to_string_lossy()
                .into_owned();
            Some(stem)
        })
        .collect();
    stems.sort();
    stems
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let data_dir = Path::new(manifest_dir).join("tests/data");

    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let parse_path = Path::new(&out_dir).join("fixture_parse_tests.rs");
    let roundtrip_path = Path::new(&out_dir).join("fixture_roundtrip_tests.rs");

    let mut parse_code = String::new();
    let mut roundtrip_code = String::new();

    for version in ["v2", "v3", "v4", "v5"] {
        let version_dir = data_dir.join(version);
        // Re-run if any fixture is added, removed, or renamed.
        println!("cargo::rerun-if-changed={}", version_dir.display());

        if !version_dir.is_dir() {
            continue;
        }

        let xml_files = fixture_stems(&version_dir);

        writeln!(parse_code, "#[cfg(feature = \"{version}\")]").unwrap();
        writeln!(parse_code, "mod {version}_fixture_parse {{").unwrap();

        writeln!(roundtrip_code, "#[cfg(feature = \"{version}\")]").unwrap();
        writeln!(roundtrip_code, "mod {version}_fixture_roundtrip {{").unwrap();

        for stem in &xml_files {
            // Sanitize filename to valid Rust identifier
            let test_name = stem.replace('-', "_");
            let rel_path = format!("{version}/{stem}.xml");

            writeln!(parse_code, "    #[test]").unwrap();
            writeln!(parse_code, "    fn parse_{test_name}() {{").unwrap();
            writeln!(
                parse_code,
                "        let bytes = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/{rel_path}\"));"
            )
            .unwrap();
            writeln!(
                parse_code,
                "        hpxml_core::{version}::parse(bytes).unwrap_or_else(|e| panic!(\"failed to parse {rel_path}: {{e}}\"));"
            )
            .unwrap();
            writeln!(parse_code, "    }}").unwrap();

            writeln!(roundtrip_code, "    #[test]").unwrap();
            writeln!(roundtrip_code, "    fn roundtrip_{test_name}() {{").unwrap();
            writeln!(
                roundtrip_code,
                "        let bytes = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/{rel_path}\"));"
            )
            .unwrap();
            writeln!(
                roundtrip_code,
                "        let doc1 = hpxml_core::{version}::parse(bytes).unwrap_or_else(|e| panic!(\"failed to parse {rel_path}: {{e}}\"));"
            )
            .unwrap();
            writeln!(
                roundtrip_code,
                "        let xml = hpxml_core::HpxmlSerialize::to_xml(&doc1).unwrap_or_else(|e| panic!(\"failed to serialize {rel_path}: {{e}}\"));"
            )
            .unwrap();
            writeln!(
                roundtrip_code,
                "        let normalized = crate::common::normalize_xsi(&xml);"
            )
            .unwrap();
            writeln!(
                roundtrip_code,
                "        let doc2 = hpxml_core::{version}::parse(&normalized).unwrap_or_else(|e| panic!(\"failed to re-parse {rel_path}: {{e}}\"));"
            )
            .unwrap();
            writeln!(
                roundtrip_code,
                "        assert_eq!(doc1, doc2, \"round-trip mismatch for {rel_path}\");"
            )
            .unwrap();
            writeln!(roundtrip_code, "    }}").unwrap();
        }

        writeln!(parse_code, "}}").unwrap();
        writeln!(parse_code).unwrap();
        writeln!(roundtrip_code, "}}").unwrap();
        writeln!(roundtrip_code).unwrap();
    }

    fs::write(&parse_path, parse_code).expect("failed to write fixture_parse_tests.rs");
    fs::write(&roundtrip_path, roundtrip_code).expect("failed to write fixture_roundtrip_tests.rs");
}
