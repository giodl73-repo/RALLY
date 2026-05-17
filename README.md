# RALLY

Shared Rust playtest and simulation-validation infrastructure for the Games
Design series.

RALLY is the common substrate for deterministic runs, event traces, validation
reports, and evidence packets across repos such as AMAZE, QUEST, HUNT, and
TIGRIS. It should keep reusable mechanics here while each game repo keeps its
own creative policy, Markdown contracts, personas, and review language.

## First command

```powershell
cargo test
```

## Product thesis

Game-design repos need repeatable validation without flattening their media.
Escape rooms, D&D campaigns, puzzle hunts, and board games all have different
creative contracts, but they share a validation shape:

```text
SCENARIO -> SEEDED RUN -> EVENT LOG -> VALIDATION REPORT -> EVIDENCE PACKET
```

RALLY owns that shared shape. Repo-local adapters decide what a scene, beat,
player, puzzle, Parliament stake, or room mechanism means.

## Phase motif

RALLY phases use famous speed races as planning mnemonics:

1. **Cannonball Run** - foundation and first adoption plans.
2. **Mille Miglia** - AMAZE adapter extraction.
3. **Dakar Rally** - QUEST mechanics compatibility.
4. **24 Hours of Le Mans** - HUNT and TIGRIS endurance validation.

## Initial core

The first crate, `rally-core`, provides:

- deterministic seeded run primitives,
- deterministic dice primitives for reusable mechanical resolution,
- tabletop turn order, score track, and token-pool helpers,
- deterministic shuffle/draw piles, phase tracks, action budgets, and grid
  coordinates for board/card/table games,
- token conversion, market rows, area-control majority checks, and vote tallies,
- bounded sampling and percentage helpers for repeatable simulations,
- stable scenario/scene/beat references,
- JSONL event-log row output,
- validation findings and report status,
- packet manifest scaffolding.

## First consumers

| Repo | Target use |
|---|---|
| AMAZE | Extract product-neutral seeded simulation, validation rows, and packet conventions from the Rust room harness. |
| QUEST | Gradually port deterministic dice, event logging, checkpoint validation, and mechanical beat validation from Python. |
| HUNT | Validate puzzle graphs, solver coverage, blind-test manifests, hint timing, and publish readiness. |
| TIGRIS | Validate Parliament stage completeness, axis/stake rows, seeded playthrough manifests, and corpus/rubric consistency. |

## Non-goals

- RALLY does not own escape-room, D&D, puzzle-hunt, or board-game creative policy.
- RALLY does not replace repo-local skills or review panels.
- RALLY does not require every game repo to rewrite all existing tools at once.
- RALLY does not publish private room, campaign, vendor, or playtest details.

## License

MIT License. Copyright (c) Gio Della-Libera.
