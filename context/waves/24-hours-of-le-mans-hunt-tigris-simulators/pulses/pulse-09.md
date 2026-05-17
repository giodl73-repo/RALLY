# Pulse 09 - Findings Promotion

## Goal

Close the loop from simulator output back into product guidance while keeping
RALLY product-neutral.

## Changes

- HUNT promotes the Game Night simulator finding into scenario guidance:
  - keep module isolation,
  - avoid serial admin review,
  - use a ship-room cadence for multi-author integration,
  - treat parallel review as the first schedule unlock.
- TIGRIS promotes the UPSTAGE simulator finding into game guidance:
  - test `double-spotlight` next because it raises memorable co-play,
  - keep first-arrival as the default resolution rule,
  - reserve the 8-player chain limit as an observed-chaos guardrail.

## Boundary

RALLY records the pulse and validation commands only. HUNT owns puzzle-hunt
workflow policy. TIGRIS owns board-game rule policy.

## Validation

```powershell
cd repos\games-design\hunt\tools\hunt-sim
cargo test --quiet
cargo run --quiet -- --scenario boardgames --seed game-night-smoke --compare-variants --runs 12

cd ..\..\..\tigris\tools\tigris-sim
cargo test --quiet
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8 --compare-variants --runs 24
```
