# Contributing to hpxml-rs

## Getting started

### Prerequisites

- Rust 1.86+ (the [rust-toolchain.toml](rust-toolchain.toml) pins 1.94 for development)
- `curl` (for fetching XSD schemas)

### Clone and build

```bash
git clone --recurse-submodules https://github.com/AZX-PBC-OSS/hpxml-rs.git
cd hpxml-rs
cargo check --workspace --all-features
```

If you already cloned without `--recurse-submodules`:

```bash
git submodule update --init
```

### Run tests

```bash
cargo test -p hpxml-core --features full
```

### Run examples

```bash
cargo run -p hpxml-core --features v4 --example parse_file -- tests/data/v4/audit.xml
cargo run -p hpxml-core --features full --example inspect_file -- tests/data/v4/audit.xml
cargo run -p hpxml-core --features v4 --example roundtrip -- tests/data/v4/audit.xml
```

## Project structure

```
crates/
  hpxml/              Public facade crate (cargo add hpxml)
  hpxml-core/         Core logic: inspect, parse, serialize, examples
  hpxml-common/       Shared types: errors, ParseConfig, HpxmlVersion
  hpxml-types-v{N}/   Generated types per schema version (~440-525K LOC each)
scripts/
  fetch-schema.sh     Fetch XSD schemas from hpxmlwg/hpxml
  codegen.sh          Run code generation
  codegen-runner/     Standalone codegen binary (excluded from workspace)
tests/data/           Test fixtures (see tests/data/README.md)
vendor/xsd-parser/    Git submodule, patched xsd-parser (temporary, until upstream publishes fixes)
```

See [docs/architecture.md](docs/architecture.md) for design rationale and crate dependencies.

## Code generation

All types in `crates/hpxml-types-v{N}/` are generated from XSD schemas. **Never hand-edit
generated code.** To regenerate:

```bash
# Fetch schemas (if not already present)
./scripts/fetch-schema.sh v4

# Generate all versions
./scripts/codegen.sh

# Generate specific versions
./scripts/codegen.sh v4
```

CI runs a drift check (`scripts/codegen.sh && git diff --exit-code`) to verify committed
code matches fresh output.

See [docs/codegen.md](docs/codegen.md) for configuration details and XSD-to-Rust mapping.

## Submitting changes

### Before opening a PR

```bash
cargo fmt --check
cargo clippy --workspace --all-features
cargo test --workspace --all-features
```

CI runs all of these plus a `cargo doc` build and codegen drift check.

### PR guidelines

- Keep commits small and focused. Each commit should compile and pass tests.
- Add or update tests for any behavioral change.
- Don't modify generated code in `crates/hpxml-types-v{N}/`. If the generated output
  needs to change, update the codegen configuration or contribute upstream to
  [xsd-parser](https://github.com/Bergmann89/xsd-parser).

## Adding a new HPXML version

Adding a version is mostly automated. See the existing `v2`/`v3`/`v4`/`v5` modules for
the pattern:

1. Add a schema tag mapping in `scripts/fetch-schema.sh`
2. Create `crates/hpxml-types-vN/` with a minimal `Cargo.toml` and `src/lib.rs`
3. Add the feature gate in `crates/hpxml-core/Cargo.toml` and `crates/hpxml/Cargo.toml`
4. Register the version module in `crates/hpxml-core/src/` (~8 lines using `impl_version_module!`)
5. Add the namespace mapping in `crates/hpxml-common/src/version.rs`
6. Run `./scripts/codegen.sh vN`
7. Add test fixtures in `tests/data/vN/`

## The vendor/xsd-parser submodule

The `vendor/xsd-parser/` submodule provides a patched version of
[xsd-parser](https://github.com/Bergmann89/xsd-parser) with fixes not yet published
to crates.io. It is used in two ways:

- **Build time**: `scripts/codegen-runner/` links against it to run code generation
- **Runtime**: `[patch.crates-io]` in the workspace `Cargo.toml` overrides
  `xsd-parser-types` with the vendored copy

This is temporary. Once upstream publishes the fixes, the submodule and patch will be
removed, and the workspace will depend on the crates.io version directly.

## Reporting issues

Please open an issue on [GitHub](https://github.com/AZX-PBC-OSS/hpxml-rs/issues).
Include the HPXML schema version, a minimal XML reproducer if applicable, and the
error message or unexpected behavior.

## License

By contributing, you agree that your contributions will be licensed under the
[BSD-3-Clause](LICENSE) license.
