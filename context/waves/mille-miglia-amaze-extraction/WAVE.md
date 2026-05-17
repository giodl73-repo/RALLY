# Mille Miglia - AMAZE Extraction

## Goal

Move the first product-neutral seeded simulation primitives from the AMAZE Rust
harness into RALLY while preserving the existing AMAZE CLI surface.

## Thesis

AMAZE is the best first consumer because it already has a real Rust harness with
deterministic simulation, scoring, timing, reliability pressure, and packet
surfaces. RALLY should absorb reusable primitives only after an active consumer
proves the boundary.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Bounded run primitives | done | Added numeric seeds, bounded sampling, percent chance, and percent-of helpers to `rally-core`. |
| 02 | AMAZE compatibility wrapper | done | Wired AMAZE harness `sim.rs` to RALLY while keeping the local API stable. |
| 03 | Role review and adoption docs | done | Recorded the adapter boundary and adoption status for the next extraction wave. |

## Success criteria

- RALLY validates with `cargo fmt --check`, `cargo test`, and `git diff --check`.
- AMAZE validates with its harness tests and active-room simulation/score smoke
  commands.
- AMAZE depends on RALLY through GitHub, not crates.io.
- Escape-room policy remains in AMAZE.
