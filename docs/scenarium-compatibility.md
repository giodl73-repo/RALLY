# SCENARIUM compatibility

RALLY remains authoritative for dice, turn order, board/card primitives, hidden
zones, event traces, and game/playtest policy. SCENARIUM can replace only the
neutral run, metric, comparison, finding, and evidence-record layer.

## Retained proof

`tests/scenarium_compat.rs` maps RALLY's retained accepted and rejected report
fixtures through a RALLY-local adapter. It also maps the retained improved
comparison and proves two intentional incompatibilities:

- an empty RALLY comparison reports `empty`; SCENARIUM rejects it with
  `EmptyMetricSet`;
- a non-finite RALLY metric can reach JSON as `null`; SCENARIUM rejects it with
  `NonFiniteMetric`.

Run the proof with:

```powershell
cargo test --test scenarium_compat
```

## Migration and deletion ledger

The following current RALLY definitions are neutral duplicates eligible for
deletion only after affected consumers pin SCENARIUM and pass their own suites:

| RALLY definition | Current source range | SCENARIUM replacement | Recorded consumers |
|---|---|---|---|
| `SimulationRun` | `src/lib.rs:673-718` | `Scenario`, `RunRecord`, `RunVariant` | HUNT, BANISH |
| `SimulationMetric` | `src/lib.rs:800-825` | `Metric`, `MetricDirection` | BANISH |
| `ComparisonDelta` | `src/lib.rs:838-914` | `MetricDelta` produced by `compare_runs` | BANISH, CERES |
| `ComparisonReport` | `src/lib.rs:916-959` | `ComparisonReport` | BANISH, CERES |
| `ValidationFinding` | `src/lib.rs:1094-1152` | `Finding`, `Severity` | HUNT, BANISH, CERES |
| `ValidationReport` | `src/lib.rs:1166-1209` | findings on `RunRecord` | HUNT, BANISH, CERES |
| `PacketManifest` | `src/lib.rs:1215-1243` | `EvidencePacket`, `ArtifactRef` | CERES |

Associated RUNE registrations and tests must be removed or redirected in the
same migration commit. `RunSeed`, `ActorTrace`, `BeatRef`, `EventLogEntry`, and
all tabletop/board helpers remain in RALLY because SCENARIUM does not own game
mechanics or actor/event policy.

Rollback is a revision pin to the last passing RALLY commit. Historical RALLY
fixtures remain immutable; migration adds SCENARIUM documents rather than
rewriting prior evidence.
