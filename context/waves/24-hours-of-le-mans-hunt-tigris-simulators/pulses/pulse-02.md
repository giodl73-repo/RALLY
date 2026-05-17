# Pulse 02 - Pit Wall

## Status

Done.

## Work

- Added `tools/tigris-sim` in TIGRIS.
- Simulates Parliament axis drafting, stakes, adjacency collisions,
  defenses/refutations, adoption pressure, and chair traces.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke
```
