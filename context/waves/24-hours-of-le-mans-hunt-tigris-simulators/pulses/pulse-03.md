# Pulse 03 - Night Stint

## Status

Done.

## Work

- Added `tools/hunt-sim` in HUNT.
- Simulates WAVELENGTH feeder solve times, hint pressure, meta readiness, and
  target-window validation.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke
```
