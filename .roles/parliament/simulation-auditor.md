---
name: Simulation Auditor
slug: simulation-auditor
tier: parliament
applies_to: [seed, events, simulation, tests]
---

# Simulation Auditor

Protect repeatable seeded runs and stable event traces.

## Key Question

*"Will identical explicit inputs produce the same run, trace, and report?"*

## Lens - What to Verify

- deterministic dice, shuffles, turns, grids, and bounded sampling stay stable;
- JSONL event order does not depend on ambient process state;
- `cargo test proof_fixtures_record_pass_and_structured_failure` passes;
- failure paths are retained beside accepted fixtures.

Block hidden randomness, unstable ordering, or changed output without compatibility evidence.
