use std::ffi::OsStr;
use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let data_dir = Path::new(manifest_dir).join("tests/data");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("fixture_parse_tests.rs");

    let mut code = String::new();

    for version in ["v2", "v3", "v4", "v5"] {
        let version_dir = data_dir.join(version);
        // Re-run if fixtures change
        println!("cargo::rerun-if-changed={}", version_dir.display());

        if !version_dir.is_dir() {
            continue;
        }

        let mut xml_files: Vec<String> = fs::read_dir(&version_dir)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", version_dir.display()))
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.extension() == Some(OsStr::new("xml"))
                    && path.file_stem() != Some(OsStr::new("invalid"))
                {
                    Some(path.file_stem().unwrap().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect();
        xml_files.sort();

        writeln!(code, "#[cfg(feature = \"{version}\")]").unwrap();
        writeln!(code, "mod {version}_fixture_parse {{").unwrap();

        for stem in &xml_files {
            // Sanitize filename to valid Rust identifier
            let test_name = stem.replace('-', "_");
            let rel_path = format!("{version}/{stem}.xml");

            writeln!(code, "    #[test]").unwrap();
            writeln!(code, "    fn parse_{test_name}() {{").unwrap();
            writeln!(
                code,
                "        let bytes = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/tests/data/{rel_path}\"));"
            )
            .unwrap();
            writeln!(
                code,
                "        hpxml_core::{version}::parse(bytes).unwrap_or_else(|e| panic!(\"failed to parse {rel_path}: {{e}}\"));"
            )
            .unwrap();
            writeln!(code, "    }}").unwrap();
        }

        writeln!(code, "}}").unwrap();
        writeln!(code).unwrap();
    }

    fs::write(&out_path, code).unwrap();
}
