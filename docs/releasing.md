# Releasing

## Semver policy

Pre-1.0: `y` bumps in `0.y.z` are breaking. Initial release: **0.1.0**.

- Schema additions (new optional fields, enum variants): **minor**
- Schema fixes: **patch**
- New version module (e.g. `v5::`): **minor**
- Removing a version module or changing handwritten API: **breaking**

Bump to 1.0 after first real consumer integration.

## MSRV

All crates declare `rust-version = "1.86"` (Edition 2024 floor). CI builds with the
toolchain pinned in `rust-toolchain.toml` (currently 1.94).

## Publishing checklist

Publish in dependency order with ~30s waits for crates.io indexing:

```bash
# 1. Verify no codegen drift
scripts/codegen.sh && git diff --exit-code

# 2. Run all tests
cargo test --workspace --all-features

# 3. Publish leaf crate
cargo publish -p hpxml-common

# 4. Publish type crates (can be parallel after step 3)
cargo publish -p hpxml-types-v2
cargo publish -p hpxml-types-v3
cargo publish -p hpxml-types-v4
cargo publish -p hpxml-types-v5

# 5. Publish core (depends on common + type crates)
cargo publish -p hpxml-core

# 6. Publish facade (depends on core)
cargo publish -p hpxml
```

## CI pipeline

Workflow: `.github/workflows/ci.yml`

**On every push/PR:**
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-features` (handwritten crates deny warnings via `[lints] workspace = true`; generated type crates allow only `clippy::never_loop`)
3. `cargo test --workspace --all-features`
4. Codegen drift check
5. Publish dry-run for leaf crates (`hpxml-common`, `hpxml-types-v{2,3,4,5}`)

Release-tag automation (publish + multi-platform matrix) is not yet implemented.
Publishing is currently manual per the checklist above.

## CI notes

- `submodules: true` required in checkout; `vendor/xsd-parser/` is a git submodule
- `codegen-runner` is excluded from workspace so `cargo test --workspace` doesn't compile xsd-parser
- Cargo.lock is committed for reproducibility
