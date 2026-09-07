# Releasing

## Semver policy

Pre-1.0: `y` bumps in `0.y.z` are breaking. Initial release: **0.1.0**.

- Schema additions (new optional fields, enum variants): **minor**
- Schema fixes: **patch**
- New version module (e.g. `v5::`): **minor**
- Removing a version module or changing handwritten API: **breaking**

Bump to 1.0 after first real consumer integration.

## MSRV

All crates declare `rust-version = "1.86"` (Edition 2024 floor). Day-to-day CI
uses the toolchain pinned in `rust-toolchain.toml` (currently 1.94); the
`msrv` job enforces `rust-version = "1.86"` with `check` + `test`.

`[profile.dist]` sets `panic = "abort"`: never run tests or benches under
`--profile dist`, and note that downstream `catch_unwind` around hpxml calls
would abort instead of unwinding in dist-profile binaries.

## Publishing checklist

Publishing goes through `.github/workflows/release.yml` (Trusted Publishing,
OIDC — no tokens), which follows the same pattern as `publish-crate` in
AZX-PBC-OSS/tors. One-time setup per crate on crates.io (Settings → Trusted
Publishers): owner `AZX-PBC-OSS`, repo `hpxml-rs`, workflow `release.yml`,
environment `crates-io`.

```bash
# 0. Bump the workspace version in the root Cargo.toml ([workspace.package])
#    and refresh Cargo.lock, commit, and tag (e.g. v0.2.0) once CI is green.

# 1. Verify no codegen drift (matches CI, including untracked files)
scripts/codegen.sh && git diff --exit-code && test -z "$(git status --porcelain crates/)"

# 2. Run all tests
cargo test --workspace --all-features --locked

# 3. Dry-run the release workflow (Actions → Release → Run workflow,
#    dry_run checked) and confirm the OIDC handshake succeeds.

# 4. Push the version tag (e.g. v0.2.0). The tag run publishes for real in
#    dependency order with index waits: hpxml-common → hpxml-types-v{2,3,4,5}
#    → hpxml-core → hpxml.

# 5. Attach release notes to the tag describing schema changes and any
#    handwritten-API changes per the semver policy above.
```

If the workflow ever needs to be bypassed, the manual fallback is the same
order with `cargo publish -p <crate>` and ~30-60s waits between crates for
indexing. Note hpxml-types-v{2,3,4,5} depend only on crates.io packages (not
on hpxml-common), so common and the type crates are independent leaves.

## CI pipeline

Workflow: `.github/workflows/ci.yml`

**On every push/PR:**
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-features --locked -- -D warnings` (handwritten crates deny warnings via `[lints] workspace = true`; generated type crates carry `#![allow(warnings)]` and are covered by the drift check, not clippy)
3. `cargo test --workspace --all-features --locked`
4. Feature matrix (`features` job): check + test `hpxml-core` under no-defaults, each single version, and `full` (the `hpxml` facade without version features is a deliberate `compile_error`, not a supported configuration)
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
