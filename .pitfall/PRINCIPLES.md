# RALLY Principles

## RALLY-P-01: Shared Core Stays Product-Neutral

**Status:** ACTIVE

**Statement:** RALLY owns deterministic run, trace, validation, comparison, and
evidence-packet primitives, while each consumer owns its game, creative,
economic, review, and release policy.

**Decision rule:** Reject shared-core changes that encode escape-room, D&D,
puzzle-hunt, board-game, or CERES-specific semantics.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `CLAUDE.md`,
`.roles/ROLE.md`, and `docs/consumer-compatibility.md`.

## RALLY-P-02: Determinism Is The Proof Surface

**Status:** ACTIVE

**Statement:** RALLY evidence is useful because seeded runs, event rows,
validation reports, and comparison outputs can be reproduced.

**Decision rule:** Any change to seeds, ordering, dice, event JSON, status
values, or report fields needs retained fixture or compatibility evidence.

**Test:** `cargo test proof_fixtures_record_pass_and_structured_failure` and
`cargo test --test consumer_contracts`.

## RALLY-P-03: Consumer Compatibility Is Representative, Not Exhaustive

**Status:** ACTIVE

**Statement:** RALLY's consumer matrix protects representative neutral
surfaces, not every downstream import or product behavior.

**Decision rule:** Treat a passing provider rehearsal as compatibility evidence
only for the named surface and require affected consumer tests before migration.

**Evidence:** `docs/consumer-compatibility.md` and
`contracts/consumer-surfaces.json`.

## RALLY-P-04: RUNE Describes The Neutral Spine

**Status:** ACTIVE

**Statement:** RUNE descriptors document RALLY's neutral simulation and
validation evidence fields without moving HUNT, TIGRIS, BANISH, QUEST, or CERES
policy into RALLY.

**Decision rule:** Add RUNE metadata only for shared field contracts and keep
product-specific meanings in adapters or consumer repos.

**Evidence:** `docs/rune/README.md`, `docs/rune/simulation_contracts.json`,
and `context/waves/24-hours-of-le-mans-hunt-tigris-simulators/pulses/pulse-10.md`.

## RALLY-P-05: Shared Fixtures Must Not Leak Private Playtest Content

**Status:** ACTIVE

**Statement:** RALLY fixtures and packets should be synthetic or product-neutral
and must not publish private room, campaign, vendor, player, or playtest data.

**Decision rule:** Invoke the Privacy Reviewer before adding fixtures,
examples, packets, or publication artifacts.

**Evidence:** `.roles/stakeholders/privacy-reviewer.md`, `CLAUDE.md`, and
`README.md`.
