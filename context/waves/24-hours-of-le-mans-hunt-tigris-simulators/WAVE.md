# 24 Hours of Le Mans - HUNT/TIGRIS Simulators

## Status

Active.

## Goal

Prove RALLY can support long-running game-design validation without absorbing
game-specific policy. HUNT and TIGRIS get local Rust simulators; RALLY provides
seeded runs, actor traces, metrics, and validation rows.

## Scope

- Add product-neutral simulation primitives to `rally-core`.
- Add a TIGRIS board-game simulator pilot for Parliament.
- Add a HUNT puzzle-hunt simulator pilot for WAVELENGTH.
- Track adapter-owned policy explicitly so RALLY stays generic.

## Validation

```powershell
cd repos\games-design\rally
cargo test --quiet

cd ..\tigris\tools\tigris-sim
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke

cd ..\..\hunt\tools\hunt-sim
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke
```
