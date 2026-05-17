# Pulse 07 - Shared Delta Board

## Status

Done.

## Work

- Added RALLY `ComparisonDelta` and `ComparisonReport` primitives.
- Updated HUNT `hunt-sim` variant output to classify each variant against
  baseline pass rate, average time, and p95 time.
- Updated TIGRIS `tigris-sim` variant output to classify each variant against
  baseline adoption rate, collision count, and no-adoption rate.

## Validation

```powershell
cd repos\games-design\rally
cargo test --quiet

cd ..\hunt\tools\hunt-sim
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke --compare-variants --runs 12

cd ..\..\tigris\tools\tigris-sim
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke --compare-variants --runs 20 --players 4
```
