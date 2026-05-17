# Cannonball Run - Foundation

## Goal

Create the RALLY repo foundation and first tested, product-neutral validation
contract for Games Design simulation harnesses.

## Thesis

AMAZE, QUEST, HUNT, and TIGRIS have different creative media, but all need
repeatable seeded runs, event traces, validation reports, and evidence packets.
RALLY should provide those shared primitives without moving repo-specific game
policy out of the consumer repos.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | done | Created Rust crate, docs, roles, skills, and first tested contract. |
| 02 | AMAZE extraction plan | done | Identified product-neutral harness pieces to extract first. |
| 03 | QUEST compatibility plan | done | Defined Python-to-Rust compatibility checks for dice, event logs, and checkpoints. |

## Success criteria

- README explains the repo purpose and first command.
- Product plan names speed-race phases, consumer repos, and non-goals.
- Wave/pulse scaffolding exists.
- Skills exist for future wave, pulse, and research execution.
- `rally-core` has passing tests for deterministic seeds, validation status, and
  event-log output.
- AMAZE extraction and QUEST compatibility plans exist before consumer adoption.
