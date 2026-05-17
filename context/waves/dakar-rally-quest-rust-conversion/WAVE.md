# Dakar Rally - QUEST Rust Conversion

## Status

Done.

## Goal

Replace QUEST's Python playtest harness with a Rust CLI that uses RALLY's
deterministic seed primitives while preserving the workshop's table workflow:
start, status, resume, route setting, explicit dice rolls, event logging, and
module binding.

## Scope

- Add a QUEST-local Rust crate and binary.
- Depend on `rally-core` from the RALLY GitHub repository.
- Port the active CLI surfaces from Python to Rust.
- Remove the Python harness, pytest suite, and obsolete requirements file.
- Update active QUEST docs and skills to invoke Rust commands.

## Validation

Run from `repos\games-design\quest`:

```powershell
cargo test --quiet
cargo run --quiet -- status
cargo run --quiet -- start --adventure 0007-the-silver-ingot-confession --session S01 --party compact-wardens
cargo run --quiet -- set-route D
cargo run --quiet -- roll 1d20+5 --seed S01-smoke --adv --bless
```

## Outcome

QUEST now owns game-specific policy in Rust, while shared deterministic run
behavior comes from RALLY.
