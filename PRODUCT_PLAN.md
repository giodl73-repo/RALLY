# RALLY Product Plan

## Wedge

Shared deterministic playtest and simulation-validation core for the Games
Design series.

## Repository role

RALLY is shared infrastructure in the Games Design series. It is not a product
game and not general developer tooling. It exists so game-design repos can share
the same proof surface for seeded runs, event logs, validation reports, and
evidence packets while preserving repo-local creative contracts.

## Phase 1: Campfire - Foundation

Goal: establish the product-neutral Rust core and the first adoption contract.

Planned capabilities:

- Seeded deterministic run primitives.
- Stable scenario, scene, and beat references.
- JSONL event-log row output.
- Validation report status and finding rows.
- Packet manifest skeleton.
- Explicit consumer map for AMAZE, QUEST, HUNT, and TIGRIS.

## Phase 2: Caravan - AMAZE adapter extraction

Goal: move product-neutral pieces from the AMAZE harness into RALLY without
breaking AMAZE commands.

Planned capabilities:

- Shared seeded simulation primitives.
- Shared validation finding/report rows.
- Shared packet manifest conventions.
- AMAZE adapter compatibility tests.

## Phase 3: Muster - QUEST mechanics port

Goal: prototype a QUEST adapter boundary for deterministic mechanics.

Planned capabilities:

- Dice/event-log compatibility against QUEST Python behavior.
- Checkpoint schema validation.
- Mechanical beat validation surfaces.
- No narrative rewrite.

## Phase 4: Circuit - HUNT and TIGRIS adapters

Goal: prove the RALLY contract across puzzle hunts and board-game Parliament
records.

Planned capabilities:

- HUNT puzzle graph and solver coverage validation.
- TIGRIS stage/stake/rubric consistency validation.
- Adapter-owned policy with RALLY-owned run/report/packet primitives.

## Non-goals

- RALLY does not centralize game design policy.
- RALLY does not replace Markdown-first authoring.
- RALLY does not absorb the existing AMAZE harness wholesale.
- RALLY does not force a big-bang QUEST rewrite.
- RALLY does not depend on crates.io publishing; downstream use should depend on
  GitHub repositories.
