# Code Generation

Types are generated from official HPXML XSD schemas using
[xsd-parser](https://github.com/Bergmann89/xsd-parser) (v1.5.0, MIT).

## Workflow

```bash
# Fetch schemas (requires curl)
./scripts/fetch-schema.sh

# Generate all versions
./scripts/codegen.sh

# Generate specific versions
./scripts/codegen.sh v4
```

Generated code is committed to `crates/hpxml-types-v{N}/src/`. The codegen runner
(`scripts/codegen-runner/`) is a standalone binary excluded from the workspace; it
is never a dependency of published crates.

## Configuration

```rust
Config::default()
    .with_naming(ExplicitNaming::new())
    .with_quick_xml()
    .with_derive(["Clone", "Debug", "PartialEq"])
```

### Key decisions

| Setting | Choice | Rationale |
|---|---|---|
| `ExplicitNaming` | On | Default naming causes collisions (e.g. `Contractor`/`ContractorType`) |
| `xs:integer` | `i32` (default) | All 14 HPXML integer types are bounded; BigInt unnecessary |
| `OptimizerFlags` | Empty | `REMOVE_DUPLICATES` collapses semantically distinct types |
| `ADVANCED_ENUMS` | Off | Not needed for HPXML's string-valued enums |

## Generated output per version

3 Rust modules matching the 3 XSD files: `hpxml.rs`, `hpxml_base_elements.rs`,
`hpxml_data_types.rs`, plus `root.rs` (module declarations, namespace constants)
and `xs.rs` (XML Schema support types).

## XSD construct mapping

| XSD | Generated Rust |
|---|---|
| `xs:sequence` | Struct fields in schema order |
| `xs:choice` | `enum` with variant per option |
| `xs:extension` | Base fields flattened into derived struct |
| `xs:restriction` (enum) | `enum`; digit-prefixed values get `_` prefix |
| `xs:restriction` (numeric) | Newtype with `validate_value()` |
| `xs:any` (extensionType) | `Vec<AnyElement>` (lossless round-trip) |
| Recursive types | `Box<T>` inserted automatically |

## Drift check

CI verifies committed code matches fresh codegen output:

```bash
scripts/codegen.sh && git diff --exit-code
```
