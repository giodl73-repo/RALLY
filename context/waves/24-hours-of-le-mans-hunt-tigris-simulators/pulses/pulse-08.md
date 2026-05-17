# Pulse 08 - Second Simulator Pilots

## Goal

Prove the RALLY-backed simulator pattern extends beyond the first adapters
without moving game-specific policy into RALLY.

## Changes

- HUNT adds a Game Night simulator for `scenarios/boardgames/`.
  - Models five independent module authors, answer readiness, admin visibility,
    rework pressure, meta integration, and target-window risk.
  - Compares editorial, standup, meta-briefing, parallel-review, and ship-room
    variants with RALLY comparison reports.
- TIGRIS adds an UPSTAGE simulator for `parlor/games/0001-upstage/`.
  - Models player-count robustness, physical trigger fires, committed upstages,
    DOUBLE moments, false-upstage social-contract risk, pile-on chaos, and score
    spread.
  - Compares warmup, clearer-trigger, double-spotlight, and eight-player guardrail
    variants with RALLY comparison reports.

## Early signal

- Game Night: serial baseline integration exceeds the target window; parallel
  review is the schedule unlock, and `ship-room` is the cleanest combined fix.
- UPSTAGE: `double-spotlight` improves memorable co-play, while
  `eight-player-chain-limit` removes pile-on chaos at the cost of some raw
  upstage pressure.

## Validation

```powershell
cd repos\games-design\hunt\tools\hunt-sim
cargo test --quiet
cargo run --quiet -- --scenario boardgames --seed game-night-smoke --compare-variants --runs 12

cd ..\..\..\tigris\tools\tigris-sim
cargo test --quiet
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8 --compare-variants --runs 24
```
