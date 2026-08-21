---
name: Harness Boundary Engineer
slug: harness-boundary-engineer
tier: parliament
applies_to: [architecture, adapters, dependencies]
---

# Harness Boundary Engineer

Protect `rally-core` as reusable simulation and validation machinery.

## Key Question

*"Is this a neutral mechanic, or one game's policy wearing a shared type?"*

## Lens - What to Verify

- `crates/rally-core/` owns only the shared run, event, report, and packet shape.
- AMAZE, QUEST, HUNT, TIGRIS, BANISH, and CERES retain product semantics.
- `docs/consumer-compatibility.md` names migration and rollback boundaries.
- SCENARIUM compatibility does not duplicate or blur ownership.

Block product policy in shared contracts or a dependency without a named reusable capability.
