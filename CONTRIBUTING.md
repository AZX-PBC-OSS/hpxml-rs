# Contributing to hpxml-rs

## Getting started

### Prerequisites

- Rust 1.86+ (the [rust-toolchain.toml](rust-toolchain.toml) pins 1.94 for development)
- `curl` and `jq` (for fetching XSD schemas)

### Clone and build

```bash
git clone https://github.com/AZX-PBC-OSS/hpxml-rs.git
cd hpxml-rs
cargo check --workspace --all-features
```

### Run tests

```bash
cargo test -p hpxml-core --features full
```

### Run examples

```bash
cargo run -p hpxml-core --features v4 --example parse_file -- crates/hpxml-core/tests/data/v4/audit.xml
cargo run -p hpxml-core --features full --example inspect_file -- crates/hpxml-core/tests/data/v4/audit.xml
cargo run -p hpxml-core --features v4 --example roundtrip -- crates/hpxml-core/tests/data/v4/audit.xml
```

## Project structure

```
crates/
  hpxml/              Public facade crate (cargo add hpxml)
  hpxml-core/         Core logic: inspect, parse, serialize, examples
  hpxml-common/       Shared types: errors, ParseConfig, HpxmlVersion
  hpxml-types-v{N}/   Generated types per schema version (~290-515K LOC each)
scripts/
  fetch-schema.sh     Fetch XSD schemas from hpxmlwg/hpxml
  codegen.sh          Run code generation (comma-separated versions, e.g. `./scripts/codegen.sh v3,v4`)
  codegen-runner/     Standalone codegen binary (excluded from workspace, uses crates.io xsd-parser)
crates/hpxml-core/tests/data/   Test fixtures (see crates/hpxml-core/tests/data/README.md)
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
cargo clippy --workspace --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
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

1. No schema-tag mapping exists: XSD files are auto-detected per tag via the
   GitHub API. Verify `./scripts/fetch-schema.sh --list vN` lists the new
   tag's XSDs.
2. Create `crates/hpxml-types-vN/` with a minimal `Cargo.toml` and `src/lib.rs`
3. Add the feature gate in `crates/hpxml-core/Cargo.toml` and `crates/hpxml/Cargo.toml`
4. Register the version module in `crates/hpxml-core/src/` (~8 lines using `impl_version_module!`)
5. Add the namespace mapping in `crates/hpxml-common/src/version.rs`
6. Run `./scripts/codegen.sh vN`
7. Add test fixtures in `tests/data/vN/`

## Upstream xsd-parser fixes

All fixes we previously carried in a vendored `xsd-parser` fork are merged
upstream and released in `xsd-parser 1.5.2` / `xsd-parser-types 0.2.1`:

- missing comma separator between multiple `xs:pattern` facets
- multiple `xs:pattern` facets using AND instead of OR semantics
- group ref with `maxOccurs="unbounded"` rejecting the following element
- `ElementSerializer` emitting invalid `xmlns:=` / duplicate namespaces

The workspace depends on crates.io directly. If codegen output needs to
change further, contribute upstream to
[xsd-parser](https://github.com/Bergmann89/xsd-parser).

## Reporting issues

Please open an issue on [GitHub](https://github.com/AZX-PBC-OSS/hpxml-rs/issues).
Include the HPXML schema version, a minimal XML reproducer if applicable, and the
error message or unexpected behavior.

Security vulnerabilities must be reported privately per
[SECURITY.md](SECURITY.md), never via public issues.

## Code of conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contributions will be licensed under the
[BSD-3-Clause](LICENSE) license.
