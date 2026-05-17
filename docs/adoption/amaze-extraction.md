# AMAZE Extraction Plan

## Decision

AMAZE should be the first RALLY consumer because it already has a Rust harness
with seeded simulation, room validation, evidence queues, timing reports, and
packet outputs.

## Extract to RALLY first

| Candidate | RALLY ownership | AMAZE adapter ownership |
|---|---|---|
| Seeded run primitive | Seed parsing, deterministic number stream, run IDs. | Team archetypes, behavior probes, room-specific sampling weights. |
| Validation findings | Severity, code, location, message, report status. | Room contract rules, safety/build/ops semantics. |
| Event log rows | JSONL row shape for run, scenario, scene, beat, event type, and message. | Escape-room event taxonomy and operator-facing language. |
| Packet manifests | Artifact list and stable packet IDs. | Which room artifacts are written and how they are named. |

## Do not extract yet

- Room-pack file contract.
- Escape-room safety, egress, build, reset, and operator policy.
- Team archetype catalog.
- BOM criticality and admin recovery semantics.
- Visual readiness gates.

## Compatibility gate

The first AMAZE adapter wave should preserve existing AMAZE commands while
replacing internal primitives with RALLY equivalents:

```powershell
cargo test
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- simulate --room rooms\TEMPLATE --runs 10 --seed 42 --target 45
cargo run --manifest-path tools\amaze-harness\Cargo.toml -- score --room rooms\TEMPLATE
```

## Non-goals

- Do not move private room content into RALLY.
- Do not make RALLY depend on AMAZE.
- Do not require AMAZE to rewrite all command modules in one pulse.
