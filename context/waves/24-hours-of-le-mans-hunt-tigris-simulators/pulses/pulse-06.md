# Pulse 06 - Solver Garage

## Status

Done.

## Work

- Upgraded HUNT `hunt-sim` with WAVELENGTH variants.
- Added `--variant` for targeted runs and `--compare-variants` for batch
  comparison.
- Tested target-window and P5/P6 bottleneck knobs:
  - `stronger-hints`
  - `p5-p6-clue-relief`
  - `meta-prop-clarity`
  - `team-parallelism`
  - `guided-final-set`

## Initial finding

`guided-final-set` is the best next content candidate: it improves pass rate
without depending on the unrealistic perfect scaling of extra parallelism.
`p5-p6-clue-relief` also helps and moves bottleneck pressure away from the hard
final feeders. Pure `stronger-hints` increases hint usage but does not materially
fix pass rate.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke --compare-variants --runs 12
cargo run --quiet -- --seed wavelength-smoke --runs 12 --variant guided-final-set
```
