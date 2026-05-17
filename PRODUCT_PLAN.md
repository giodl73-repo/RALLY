# RALLY Product Plan

## Wedge

Shared deterministic playtest and simulation-validation core for the Games
Design series.

## Repository role

RALLY is shared infrastructure in the Games Design series. It is not a product
game and not general developer tooling. It exists so game-design repos can share
the same proof surface for seeded runs, event logs, validation reports, and
evidence packets while preserving repo-local creative contracts.

## Phase naming motif

RALLY phases use famous speed races as planning mnemonics for repeatable runs,
adapter endurance, and validation pressure. The names are product metaphors for
testing and simulation flow; they are not endorsements of unsafe driving or
illegal racing.

| Phase | Race motif | RALLY focus |
|---:|---|---|
| 1 | Cannonball Run | Foundation, seeded core, and first consumer plans. |
| 2 | Mille Miglia | AMAZE adapter extraction from an existing Rust harness. |
| 3 | Dakar Rally | QUEST full Rust mechanics conversion across rough narrative/mechanical terrain. |
| 4 | 24 Hours of Le Mans | HUNT and TIGRIS endurance validation across long pipelines and corpora. |

## Phase 1: Cannonball Run - Foundation

Goal: establish the product-neutral Rust core and the first adoption contract.

Planned capabilities:

- Seeded deterministic run primitives.
- Stable scenario, scene, and beat references.
- JSONL event-log row output.
- Validation report status and finding rows.
- Packet manifest skeleton.
- Explicit consumer map for AMAZE, QUEST, HUNT, and TIGRIS.

## Phase 2: Mille Miglia - AMAZE adapter extraction

Goal: move product-neutral pieces from the AMAZE harness into RALLY without
breaking AMAZE commands.

Planned capabilities:

- Shared seeded simulation primitives.
- Shared validation finding/report rows.
- Shared packet manifest conventions.
- AMAZE adapter compatibility tests.

## Phase 3: Dakar Rally - QUEST Rust conversion

Goal: replace the QUEST Python harness with a Rust CLI backed by RALLY seeded
run primitives.

Planned capabilities:

- Rust-only QUEST CLI for start/status/resume/set-route/roll/bind-module.
- Dice/event-log compatibility against prior QUEST behavior.
- Checkpoint schema validation.
- Mechanical beat validation surfaces.
- No narrative rewrite.

## Phase 4: 24 Hours of Le Mans - HUNT and TIGRIS adapters

Goal: prove the RALLY contract across puzzle hunts and board-game Parliament
records.

Planned capabilities:

- HUNT puzzle graph and solver coverage validation.
- TIGRIS stage/stake/rubric consistency validation.
- Adapter-owned policy with RALLY-owned run/report/packet primitives.

First pilots:

- `HUNT/tools/hunt-sim`: WAVELENGTH feeder/meta solve-time simulation.
- `TIGRIS/tools/tigris-sim`: Parliament axis-collision/adoption simulation.

## Non-goals

- RALLY does not centralize game design policy.
- RALLY does not replace Markdown-first authoring.
- RALLY does not absorb the existing AMAZE harness wholesale.
- RALLY does not centralize QUEST-specific adventure policy.
- RALLY does not depend on crates.io publishing; downstream use should depend on
  GitHub repositories.
