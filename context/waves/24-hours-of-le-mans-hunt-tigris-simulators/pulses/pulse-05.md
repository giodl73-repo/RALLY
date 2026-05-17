# Pulse 05 - Variant Garage

## Status

Done.

## Work

- Upgraded TIGRIS `tigris-sim` with rule variants for Parliament.
- Added `--variant` for targeted runs and `--compare-variants` for batch
  comparison.
- Tested adoption-pressure knobs:
  - `expanded-adjacency`
  - `lower-adoption`
  - `collision-boost`
  - `tournament-pressure`

## Initial finding

`tournament-pressure` is the best next candidate: it improves adoption pressure
without making adoption automatic. `lower-adoption` reaches 100% adoption-rate
in the first comparison batch, which likely makes Parliament too permissive.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke --compare-variants --runs 20 --players 4
cargo run --quiet -- --seed parliament-smoke --runs 20 --players 4 --variant tournament-pressure
```
