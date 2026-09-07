# Security Policy

## Supported versions

Only the latest `0.y.z` release line is supported with security fixes.
Pre-1.0 `y` bumps may be breaking; see `docs/releasing.md` for the semver
policy.

## Reporting a vulnerability

**Do not open a public issue.** Use
[private vulnerability reporting](https://github.com/AZX-PBC-OSS/hpxml-rs/security/advisories/new)
so we can fix the issue before disclosure.

Include:

- crate name and version (`hpxml`, `hpxml-core`, `hpxml-types-vN`, …)
- HPXML schema version and a minimal XML reproducer, if applicable
- impact assessment (DoS, incorrect parse/serialize, …)

We aim to acknowledge within 3 business days and will coordinate a fix and
crates.io release before public disclosure.

## Supply-chain posture

- Dependencies are tracked by Dependabot (weekly, grouped patch/minor) for
  the workspace and both standalone manifests.
- CI runs `cargo audit --deny warnings` on **all three** lockfiles
  (`Cargo.lock`, `scripts/codegen-runner/Cargo.lock`,
  `examples/codegen-compile-check/Cargo.lock`).
- Known exceptions with justification live in `.cargo/audit.toml`. Each
  entry names the upstream release that unblocks its removal.
- Runtime XML hardening (`crates/hpxml-common/src/config.rs`): 50 MiB
  default size cap, 128-level depth cap, DTD/entity expansion rejected.

## Scope notes

- `hpxml-rs` is a parser/serializer, not a validator: malformed HPXML is
  rejected at parse time, but business-rule validation (Schematron, XSD
  facet enforcement beyond codegen types) is out of scope.
- Unknown elements are skipped by design in generated deserializers;
  unknown enum values are strict errors.
