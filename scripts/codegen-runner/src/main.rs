use std::env;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use proc_macro2::TokenStream;
use quote::ToTokens;
use xsd_parser::config::{
    GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
};
use xsd_parser::models::code::SubModules;
use xsd_parser::models::ExplicitNaming;
use xsd_parser::{generate_modules, Config};

const HEADER_COMMENT: &str = r#"// GENERATED CODE - do not edit manually.
// Regenerate with: scripts/codegen.sh
"#;

fn rustfmt(code: &str) -> String {
    let mut child = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rustfmt - is it installed?");
    let mut stdin = child.stdin.take().unwrap();
    let owned = code.to_string();
    std::thread::spawn(move || {
        let _ = stdin.write_all(owned.as_bytes());
    });
    let output = child.wait_with_output().unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("rustfmt failed ({}): {}", output.status, stderr);
    }
    String::from_utf8(output.stdout).expect("rustfmt produced invalid UTF-8")
}

fn run_for_version(version: &str, repo_root: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
    let schema_path = repo_root.join("schemas").join(version).join("HPXML.xsd");
    if !schema_path.exists() {
        // Failing closed in CI keeps the drift gate from passing vacuously
        // when a schema fetch failed; locally, skipping lets developers
        // regenerate one version without fetching the rest.
        if env::var("CI").is_ok() {
            return Err(format!("schema not found for {version}: failing closed in CI").into());
        }
        eprintln!("Warning: schema not found for {}, skipping", version);
        return Ok(());
    }

    let mut config = Config::default()
        .with_naming(ExplicitNaming::new())
        .with_quick_xml()
        .with_derive(["Clone", "Debug", "PartialEq"]);
    // Keep generated type names aligned with XSD names (avoid `...TypeType`).
    config.generator.type_postfix.type_ = String::new();
    config.parser.flags = ParserFlags::all();
    config.parser.resolver = vec![Resolver::File];
    config.parser.schemas = vec![Schema::File(schema_path.canonicalize()?)];

    // Exclude WITH_NUM_BIG_INT: pulls in num-bigint for xs:integer; i32 suffices for HPXML.
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.optimizer.flags = OptimizerFlags::empty();
    // ADVANCED_ENUMS emits invalid code for some HPXML enum values.
    let mut generator_flags = GeneratorFlags::all();
    generator_flags.remove(GeneratorFlags::ADVANCED_ENUMS);
    config.generator.flags = generator_flags;

    println!("Generating code for {}...", version);

    let module = generate_modules(config)?;

    // Write output to crates/hpxml-types-v{N}/src/
    let crate_name = format!("hpxml-types-{}", version);
    let out_dir = repo_root.join("crates").join(crate_name).join("src");
    fs::create_dir_all(&out_dir)?;

    // Sort module names for deterministic ordering
    let mut module_names: Vec<&String> = module.modules.keys().collect();
    module_names.sort();

    // Write each submodule as a separate file (enables parallel compilation)
    for name in &module_names {
        let submodule = &module.modules[*name];
        let mut tokens = TokenStream::new();
        submodule.to_code(&mut tokens, SubModules::Inline);
        let code = rustfmt(&tokens.to_string());

        let file_content = format!("{}{}", HEADER_COMMENT, code);
        fs::write(out_dir.join(format!("{}.rs", name)), file_content)?;
    }

    // Write root.rs with path declarations for each module + namespace constants
    let mut root_code = String::new();
    for name in &module_names {
        writeln!(
            root_code,
            "#[path = \"{}.rs\"] pub mod {};",
            name, name
        )?;
    }

    // Add type alias for HpxmlType (common name used in HPXML docs)
    writeln!(root_code, "pub use hpxml::HpxmlElementType as HpxmlType;")?;

    // Write root-level code (namespace constants, etc.)
    let mut root_tokens = TokenStream::new();
    module.code.to_tokens(&mut root_tokens);
    root_code.push_str(&root_tokens.to_string());

    let formatted = rustfmt(&root_code);
    let root_content = format!("{}{}", HEADER_COMMENT, formatted);
    fs::write(out_dir.join("root.rs"), root_content)?;

    // Write lib.rs with #![allow(warnings)] and re-exports
    let lib_rs_content = format!(
        "{}#![allow(warnings)]\nmod root;\npub use root::*;\n",
        HEADER_COMMENT
    );
    let formatted_lib = rustfmt(&lib_rs_content);
    fs::write(out_dir.join("lib.rs"), formatted_lib)?;

    println!("  -> {}", out_dir.display());

    Ok(())
}

fn find_repo_root() -> PathBuf {
    // Walk up from current exe until we find the repo root (has schemas/ and Cargo.toml)
    let mut path = env::current_exe().unwrap_or_default();
    loop {
        if path.join("schemas").exists() && path.join("Cargo.toml").exists() {
            return path;
        }
        if !path.pop() {
            break;
        }
    }
    // Fallback: assume we're in repo root
    PathBuf::from(".")
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get versions from CLI args or use default
    let versions_arg = env::args().nth(1).unwrap_or_else(|| "v2,v3,v4,v5".to_string());
    let versions: Vec<&str> = versions_arg
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    if versions.is_empty() {
        eprintln!("Error: no versions specified");
        std::process::exit(1);
    }

    let repo_root = find_repo_root();
    println!("Generating HPXML code for versions: {:?}", versions);

    // Run versions in parallel using threads
    let repo_root = Arc::new(repo_root);
    let mut handles = Vec::new();

    for version in &versions {
        let version = version.to_string();
        let repo_root = Arc::clone(&repo_root);
        let ver_for_handle = version.clone();

        let handle = std::thread::spawn(move || {
            run_for_version(&version, &repo_root)
        });
        handles.push((ver_for_handle, handle));
    }

    let mut failed = false;
    for (version, handle) in handles {
        if let Err(e) = handle.join().expect("thread panicked") {
            eprintln!("Error generating {}: {}", version, e);
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }

    println!("Done.");
    Ok(())
}
