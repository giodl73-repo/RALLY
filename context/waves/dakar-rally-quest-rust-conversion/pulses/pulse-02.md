# Pulse 02 - CLI Parity

## Status

Done.

## Work

- Ported `status`, `start`, `resume`, `set-route`, `roll`, and `bind-module`.
- Preserved JSON session state, checkpoint validation, event JSONL logging, and
  explicit route updates.
- Added Rust tests for seeded rolls, inbound packet validation, and module
  parsing.

## Validation

```powershell
cargo test --quiet
cargo run --quiet -- start --adventure 0007-the-silver-ingot-confession --session S01 --party compact-wardens
```
