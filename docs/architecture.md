# Architecture

## Crate structure

```
hpxml                   ← public facade: `cargo add hpxml --features v4`
└── hpxml-core          ← glue: inspect, depth reader, parse/serialize wrappers
    ├── hpxml-common    ← shared types: errors, ParseConfig, HpxmlVersion
    ├── hpxml-types-v2  ← generated v2 types (~440K LOC), optional
    ├── hpxml-types-v3  ← generated v3 types (~440K LOC), optional
    ├── hpxml-types-v4  ← generated v4 types (~520K LOC), optional
    └── hpxml-types-v5  ← generated v5 types (~525K LOC), optional
```

Users depend only on `hpxml`. Internal crates are implementation details.

## Why separate type crates

Each version generates ~440-525K lines of Rust. Rust's compilation unit is the crate.
With all versions in a single crate, ~2M LOC compiles serially. Splitting into
`hpxml-types-v{N}` lets rustc compile all four in parallel, cutting wall-clock build
time significantly.

The type crates contain only generated types (structs, enums, derives). No runtime
logic, no dynamic dispatch, so the split has zero runtime cost.

## Feature gating

Version features on `hpxml` control which type crates are compiled:

```toml
[features]
default = ["v4"]
v2 = ["hpxml-core/v2"]   # pulls in hpxml-types-v2
v3 = ["hpxml-core/v3"]
v4 = ["hpxml-core/v4"]
v5 = ["hpxml-core/v5"]
full = ["v2", "v3", "v4", "v5"]
```

When a feature is disabled, the corresponding type crate is never downloaded or compiled.

## Design principles

1. **Type safety first**: users must know the version they're working with
2. **Codegen-maximalist**: every parser, serializer, and type is generated from XSD
3. **Minimal glue**: handwritten code is inspect, config, errors, depth-limiting reader,
   and `impl_version_module!` (~200 lines total)
4. **Adding a version = ~8 lines** of registration code, zero handwritten parser/serializer

## Two-phase API: inspect + parse

Separate `inspect()` and `parse()`, not combined. `inspect()` reads one XML event to
determine the version. The caller then dispatches to the versioned `parse()` for full
type safety.

## Safety

- **DTD/XXE**: rejected pre-parse by the `has_doctype` byte scan over the full
  input (bounded by `max_bytes`). quick-xml emits `DocType` without expanding
  entities, so the guard short-circuits billion-laughs/XXE/internal-subset
  before parsing.
- **Depth limit**: `DepthLimitedReader` wraps `NsReader`, tracks Start/End events (default: 128)
- **Size limit**: Pre-parse `bytes.len()` check (default: 50 MiB)
- **Unknown elements**: Silently skipped (structural in generated deserializer)
- **Unknown enums**: Parse error (strict by default)

## Non-goals

- No `parse_auto()` returning a version-erased type
- No validation layer; parse and validate are strictly separate
- No `idref`/`sameas` resolution; post-parse concern
