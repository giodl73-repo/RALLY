# Pulse 02: AMAZE extraction plan

## Goal

Identify the product-neutral pieces of the AMAZE Rust harness that should move
to RALLY first.

## Changes

- Added `docs/adoption/amaze-extraction.md`.
- Scoped the first extraction to seeded run, validation finding, event-log, and
  packet-manifest primitives.
- Kept escape-room policy in AMAZE adapters.

## Validation

- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
