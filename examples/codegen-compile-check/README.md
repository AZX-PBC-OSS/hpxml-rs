# codegen-compile-check

Small standalone project that generates full HPXML bindings (v2/v3/v4/v5) with `xsd-parser` and compiles them.

## What it checks

- `xsd-parser` config from `docs/codegen.md` compiles full generated modules.
- Optional strict mode (`deny-warnings`) to check warning posture.

## Run

```bash
cd examples/codegen-compile-check
cargo check
```

Check one version only (faster):

```bash
HPXML_CODEGEN_VERSIONS=v4 cargo check
```

Strict warnings mode:

```bash
HPXML_CODEGEN_VERSIONS=v4 cargo check --features deny-warnings
```

## Notes

- Build script uses `ExplicitNaming`, `InterpreterFlags::all()`, `OptimizerFlags::empty()`, `GeneratorFlags::all()`, and `with_quick_xml()`.
- Generated modules are included from `OUT_DIR` and are not committed.
