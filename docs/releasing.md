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
# 1. Verify no codegen drift (matches CI, including untracked files)
scripts/codegen.sh && git diff --exit-code && test -z "$(git status --porcelain crates/)"

# 2. Run all tests
cargo test --workspace --all-features --locked

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
2. `cargo clippy --workspace --all-features --locked -- -D warnings` (handwritten crates deny warnings via `[lints] workspace = true`; generated type crates carry `#![allow(warnings)]` and are covered by the drift check, not clippy)
3. `cargo test --workspace --all-features --locked`
4. Feature matrix (`features` job): check + test `hpxml-core` under no-defaults, each single version, and `full`, plus the `hpxml` facade without defaults
5. Conventional-commit PR title (`commitlint` job; squash-merge feeds release automation)
6. Codegen drift check
7. Publish dry-run for leaf crates (`hpxml-common`, `hpxml-types-v{2,3,4,5}`) plus `cargo package --list` for `hpxml-core`/`hpxml` (their `cargo publish` dry-run only passes once siblings are on the index, so it runs at release time in checklist order)
8. `cargo audit` (security job; known quick-xml advisories documented in `.cargo/audit.toml`)

Release-tag automation (publish + multi-platform matrix) is not yet implemented.
Publishing is currently manual per the checklist above.

## CI notes

- `codegen-runner` is excluded from workspace so `cargo test --workspace` doesn't compile xsd-parser
- `codegen-runner` and `examples/codegen-compile-check` depend on crates.io `xsd-parser 1.5.2`
- Cargo.lock is committed for reproducibility
