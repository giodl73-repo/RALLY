# Pulse 03: QUEST compatibility plan

## Goal

Define the compatibility checks for gradually moving QUEST deterministic
mechanics behind a RALLY adapter.

## Changes

- Added `docs/adoption/quest-compatibility.md`.
- Scoped compatibility to dice parity, JSONL event traces, checkpoint schema
  validation, and beat mechanics.
- Kept narrative/session authorship in QUEST.

## Validation

- `cargo fmt --check`
- `cargo test`
- `git diff --check`

## Status

Done.
