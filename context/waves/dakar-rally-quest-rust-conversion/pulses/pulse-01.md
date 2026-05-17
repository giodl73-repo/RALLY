# Pulse 01 - Rust Harness Landing

## Status

Done.

## Work

- Created the QUEST Rust crate.
- Added the `quest` CLI binary.
- Wired `rally-core` as the shared seed dependency.
- Implemented deterministic roll parsing and RALLY-backed roll generation.

## Validation

```powershell
cargo test --quiet
```
