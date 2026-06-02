# Pulse 10 - RUNE Contract Lap

## Goal

Adopt RUNE field-metadata contracts for RALLY's neutral simulation and
validation evidence spine before any product-specific game repo annotations.

## Changes

- Added pinned RUNE dependencies for descriptor derivation.
- Annotated `SimulationRun`, `ActorTrace`, `SimulationMetric`,
  `ComparisonDelta`, `ValidationFinding`, and `ValidationReport`.
- Added `RUNE_CONTRACTS` and `rune_descriptor_collection()` for deterministic
  contract collection.
- Retained `docs\rune\simulation_contracts.json`.

## Boundary

RALLY records only product-neutral run/report/evidence contract metadata. HUNT,
TIGRIS, AMAZE, BANISH, QUEST, and CERES still own scenario-specific rules,
creative policy, private content, and adapter semantics.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
git diff --check
```
