# Pulse 03 - Python Retirement

## Status

Done.

## Work

- Removed the Python harness under `scripts\`.
- Removed the pytest suite and `requirements-dev.txt`.
- Updated active QUEST README and skill docs to use the Rust CLI.
- Recorded Dakar as a completed RALLY phase.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- status
cargo run --quiet -- roll 1d20+5 --seed S01-smoke --adv --bless
```
