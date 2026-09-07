use std::env;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::ToTokens;
use xsd_parser::config::{
    GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
};
use xsd_parser::models::code::SubModules;
use xsd_parser::models::ExplicitNaming;
use xsd_parser::{generate_modules, Config};

fn rustfmt(code: &str) -> String {
    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        // rustfmt not available, fall back to minimal formatting
        return code.replace("; ", ";\n").replace("} ", "}\n");
    };
    let mut stdin = child.stdin.take().unwrap();
    let owned = code.to_string();
    std::thread::spawn(move || {
        let _ = stdin.write_all(owned.as_bytes());
    });
    let output = child.wait_with_output().unwrap();
    if output.status.success() {
        String::from_utf8(output.stdout).unwrap_or_else(|_| code.replace("; ", ";\n"))
    } else {
        // rustfmt failed, fall back
        code.replace("; ", ";\n").replace("} ", "}\n")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let versions = env::var("HPXML_CODEGEN_VERSIONS").unwrap_or_else(|_| "v2,v3,v4,v5".to_string());
    let versions: Vec<&str> = versions
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let all_versions = ["v2", "v3", "v4", "v5"];

    // Write stub files for skipped versions
    for version in all_versions {
        let dir = out_dir.join(format!("generated_{version}"));
        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join("root.rs"),
            "// skipped by HPXML_CODEGEN_VERSIONS filter\n",
        )?;
    }

    for version in versions {
        let schema_path = repo_root.join("schemas").join(version).join("HPXML.xsd");
        println!("cargo:rerun-if-changed={}", schema_path.display());

        let mut config = Config::default()
            .with_naming(ExplicitNaming::new())
            .with_quick_xml()
            .with_derive(["Debug", "PartialEq"]);
        config.parser.flags = ParserFlags::all();
        config.parser.resolver = vec![Resolver::File];
        config.parser.schemas = vec![Schema::File(schema_path.canonicalize()?)];

        // Exclude WITH_NUM_BIG_INT: BigInt is not FFI-safe.
        config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
        config.optimizer.flags = OptimizerFlags::empty();
        // ADVANCED_ENUMS emits invalid code for some HPXML enum values.
        let mut generator_flags = GeneratorFlags::all();
        generator_flags.remove(GeneratorFlags::ADVANCED_ENUMS);
        config.generator.flags = generator_flags;

        let module = generate_modules(config)?;
        let version_dir = out_dir.join(format!("generated_{version}"));
        fs::create_dir_all(&version_dir)?;

        // Write each submodule as a separate formatted file
        let mut root_code = String::new();
        for (name, submodule) in &module.modules {
            let mut tokens = TokenStream::new();
            submodule.to_code(&mut tokens, SubModules::Inline);
            let code = rustfmt(&tokens.to_string());
            fs::write(version_dir.join(format!("{name}.rs")), &code)?;
            writeln!(
                root_code,
                "#[path = \"{dir}/{name}.rs\"] pub mod {name};",
                dir = version_dir.display()
            )?;
        }

        // Write root-level code (namespace constants, etc.)
        let mut root_tokens = TokenStream::new();
        module.code.to_tokens(&mut root_tokens);
        writeln!(root_code, "{}", root_tokens)?;

        fs::write(version_dir.join("root.rs"), root_code)?;
    }

    Ok(())
}
