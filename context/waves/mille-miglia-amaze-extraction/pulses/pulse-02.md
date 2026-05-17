# Pulse 02: AMAZE compatibility wrapper

## Goal

Wire AMAZE's local simulation API to RALLY primitives without changing command
behavior or escape-room policy ownership.

## Validation

- `cargo fmt --manifest-path tools\amaze-harness\Cargo.toml --check`
- `cargo test --manifest-path tools\amaze-harness\Cargo.toml`
- `cargo run --manifest-path tools\amaze-harness\Cargo.toml -- simulate --room rooms\0004-brineworks-at-low-tide --runs 10 --seed 42 --target 45`
- `cargo run --manifest-path tools\amaze-harness\Cargo.toml -- score --room rooms\0004-brineworks-at-low-tide`
- `git diff --check`

## Status

Done.
