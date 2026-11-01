# RALLY RUNE contracts

RALLY retains a first RUNE descriptor collection for the neutral simulator and
validation evidence spine:

| Fixture | Purpose |
|---|---|
| `simulation_contracts.json` | RUNE descriptor collection for `SimulationRun`, `ActorTrace`, `SimulationMetric`, `ComparisonDelta`, `ValidationFinding`, and `ValidationReport`. |

The fixture preserves field metadata for required status, units, sensitivity,
examples, stability, and aliases. It does not encode HUNT, TIGRIS, AMAZE,
BANISH, QUEST, or CERES product policy, and it does not approve downstream
product adoption. The JSON repeats that boundary at collection level so
generated descriptor views cannot read neutral RALLY metadata as consumer
contract authority.

Validation:

```powershell
cargo test --quiet
```
