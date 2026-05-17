# Pulse 04 - Endurance Telemetry

## Status

Done.

## Work

- Upgraded HUNT `hunt-sim` from a single WAVELENGTH run to batch mode across
  speedster, casual, and methodical solver profiles.
- Upgraded TIGRIS `tigris-sim` from a single Parliament run to batch mode with
  adoption rate, no-collision rate, no-adoption rate, and chair win spread.
- Documented the batch commands in both repos.

## Validation

```powershell
cargo run --quiet -- --seed wavelength-smoke --runs 12
cargo run --quiet -- --seed parliament-smoke --runs 20 --players 4
```
