# RALLY RUNE contracts

RALLY retains a first RUNE descriptor collection for the neutral simulator and
validation evidence spine:

| Fixture | Purpose |
|---|---|
| `simulation_contracts.json` | RUNE descriptor collection for `SimulationRun`, `ActorTrace`, `SimulationMetric`, `ComparisonDelta`, `ValidationFinding`, and `ValidationReport`. |

The fixture preserves field metadata for required status, units, sensitivity,
examples, stability, and aliases. It does not encode HUNT, TIGRIS, AMAZE,
BANISH, QUEST, or CERES product policy.

Validation:

```powershell
cargo test --quiet
```
