# Examples

Runnable end-to-end examples live in `crates/hpxml-core/examples/`:

- `parse_file`: parse a v4 file and print building info
- `inspect_file`: detect the version of any HPXML file, then parse it
- `roundtrip`: parse, modify, serialize, and verify round-trip equality
- `safe_parse`: parse with custom size limits and error handling

See the root [README](../README.md#runnable-examples) for commands.

- `codegen-compile-check`: generates full bindings with `xsd-parser` and compiles them.
