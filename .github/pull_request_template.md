## Summary

<!-- One paragraph: what and why. Link issues with "Closes #N". -->

## Checklist

- [ ] `cargo fmt --check` clean
- [ ] `cargo clippy --workspace --all-features` clean
- [ ] `cargo test --workspace --all-features` passes
- [ ] No hand edits under `crates/hpxml-types-vN/` (regenerate via `scripts/codegen.sh`; CI drift check must pass)
- [ ] Docs updated (`README.md`, `docs/`, or crate rustdoc as appropriate)
- [ ] `cargo audit` clean (or new `.cargo/audit.toml` entry with upstream unblock noted)

## Notes for reviewers

<!-- Anything non-obvious: schema versions touched, upstream xsd-parser relevance, semver impact per docs/releasing.md. -->
