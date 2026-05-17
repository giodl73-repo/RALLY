# RALLY - repo guide

RALLY is shared Rust playtest/simulation validation infrastructure for the Games
Design series.

## House rules

1. Keep `rally-core` product-neutral.
2. Put escape-room, D&D, puzzle-hunt, and board-game meanings in adapters, not in
   the core crate.
3. Prefer deterministic, seedable behavior and explicit event logs.
4. Preserve stable scenario, scene, beat, site, puzzle, game, and stage IDs from
   downstream repos.
5. Depend through GitHub repos when adopting portfolio crates; do not require
   crates.io publication.
6. Do not copy private room, campaign, vendor, player, or playtest content into
   this shared repo.

## Validation

```powershell
cargo fmt --check
cargo test
git diff --check
```

## Roles

Read `.roles/ROLE.md` before reviewing a wave, adapter boundary, or consumer
adoption change.
