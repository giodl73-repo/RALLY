# RALLY Pitfalls

## RALLY-PF-01: Shared Harness Absorbs Product Policy

**Status:** OPEN

**Pattern:** Consumer convenience adds game-specific meanings, thresholds,
adventure state, puzzle policy, board policy, or CERES economic semantics to
the shared `rally-core` crate.

**Actor:** RALLY maintainer, downstream game maintainer, adapter author, or
future agent extracting shared helpers.

**Task:** Add a shared primitive, adapter surface, migration helper, or public
contract for multiple game-design consumers.

**Surface:** `rally-core`, adapter examples, consumer migration docs,
`contracts/consumer-surfaces.json`, and public compatibility claims.

**Likely mistake:** Move one consumer's creative, puzzle, adventure, board, or
economic policy into the shared harness because it looks reusable.

**Consequence:** RALLY becomes product-specific, consumers inherit unintended
policy, and shared compatibility starts constraining product-owned design.

**Owner:** RALLY owns neutral run/report/evidence primitives; consumer repos
own creative, economic, release, and domain semantics.

**Domain:** Shared Rust primitives, adapters, consumer migrations, and public
contracts.

**Detection difficulty:** The requested feature often starts as a reusable
helper, but the examples reveal one product's policy once several consumers are
compared.

**Structural solution:** Require the Harness Boundary Engineer and consumer
matrix to name the RALLY-owned neutral surface and the consumer-owned semantics
before adding shared types.

**Evidence:** `.roles/parliament/harness-boundary-engineer.md`,
`docs/consumer-compatibility.md`, and `PRODUCT_PLAN.md`.

**Test:** `tests/pitfall_policy.rs`

## RALLY-PF-02: Green Provider Canaries Become Consumer Readiness

**Status:** OPEN

**Pattern:** RALLY's provider-side compatibility tests pass and are treated as
proof that AMAZE, QUEST, HUNT, TIGRIS, BANISH, or CERES can migrate without
their own tests or workflow review.

**Actor:** Consumer advocate, migration owner, release maintainer, or future
agent reading RALLY's compatibility matrix.

**Task:** Decide whether a consumer can migrate to RALLY or claim readiness
from provider-side tests.

**Surface:** `docs/consumer-compatibility.md`, `tests/consumer_contracts.rs`,
release notes, adapter extraction docs, and migration plans.

**Likely mistake:** Treat a green RALLY canary as proof that the consumer's
workflow, fixtures, migration, rollback, and owner review are complete.

**Consequence:** Consumer repos can absorb a shared dependency before their
own behavior, release, and rollback evidence exists.

**Owner:** RALLY owns provider rehearsal; each consumer repo owns its own
workflow tests, migration instructions, rollback plan, and approval.

**Domain:** Consumer compatibility, migration planning, release notes, and
adapter extraction.

**Detection difficulty:** The retained matrix is real evidence, but it is a
representative canary rather than exhaustive downstream validation.

**Structural solution:** Keep provider rehearsal separate from affected
consumer tests, migration instructions, rollback instructions, and owner
approval.

**Evidence:** `docs/consumer-compatibility.md` and
`.roles/stakeholders/consumer-advocate.md`.

**Test:** `tests/pitfall_policy.rs`

## RALLY-PF-03: Validation Report Shape Drifts Without Failure Evidence

**Status:** MITIGATED

**Pattern:** Accepted report fixtures keep passing while structured failures
lose stable severity, code, location, message, ordering, or JSON shape.

**Domain:** Validation reports, evidence packets, fixture updates, and CLI/API
consumers.

**Detection difficulty:** Success-only examples hide the exact failure contract
that downstream diagnostics depend on.

**Structural solution:** Retain accepted and rejected proof fixtures and compare
complete machine-readable output through public APIs.

**Evidence:** `docs/proof-surface.md`, `fixtures/proof/accepted-report.json`,
`fixtures/proof/rejected-report.json`, and `src/lib.rs`.

## RALLY-PF-04: RUNE Metadata Becomes Product Contract Authority

**Status:** OPEN

**Pattern:** RALLY's RUNE descriptor collection is treated as approval or full
semantic coverage for HUNT, TIGRIS, BANISH, QUEST, CERES, or other product
repos.

**Actor:** Contract generator user, adapter maintainer, product repo maintainer,
or future agent reading generated descriptor docs.

**Task:** Use RUNE metadata to understand RALLY fields or decide whether a
product contract is covered.

**Surface:** `docs/rune/simulation_contracts.json`, `docs/rune/README.md`,
generated contract docs, adapter adoption notes, and traceability records.

**Likely mistake:** Treat generated neutral field metadata as approval,
semantic completeness, or product contract authority for a consumer repo.

**Consequence:** Product meaning can move into RALLY/RUNE metadata and bypass
consumer-owned docs, tests, and role review.

**Owner:** RALLY owns neutral descriptor coverage; product repos own semantic
coverage, policy docs, tests, and role reviews.

**Domain:** RUNE descriptors, generated contract docs, adapter adoption, and
cross-repo traceability.

**Detection difficulty:** Field metadata looks authoritative because it is
generated from Rust types and carries requirement references.

**Structural solution:** Keep RUNE metadata limited to the neutral spine and
require consumer-owned policy docs, tests, and role reviews for product meaning.

**Evidence:** `docs/rune/README.md`,
`context/waves/24-hours-of-le-mans-hunt-tigris-simulators/pulses/pulse-10.md`,
and `docs/consumer-compatibility.md`.

**Test:** `tests/pitfall_policy.rs`

## RALLY-PF-05: Shared Evidence Packet Leaks Private Playtest Detail

**Status:** OPEN

**Pattern:** A reusable report, packet, fixture, or example carries real room,
campaign, vendor, player, or private playtest information into the shared RALLY
repo.

**Actor:** Fixture author, adapter maintainer, publication author, privacy
reviewer, or future agent enriching examples.

**Task:** Add or update reusable reports, evidence packets, examples, fixtures,
or publication artifacts.

**Surface:** `fixtures/`, `docs/proof-surface.md`, packet examples, validation
reports, and public docs.

**Likely mistake:** Use realistic consumer data because it makes validation or
documentation easier, without stripping private playtest context.

**Consequence:** Shared infrastructure becomes an accidental publication
channel for private room, campaign, vendor, player, or playtest detail.

**Owner:** RALLY owns product-neutral fixtures; source consumer repos and the
privacy reviewer own private-content approval.

**Domain:** Fixtures, examples, reports, evidence packets, and publication
artifacts.

**Detection difficulty:** Rich examples make validation easier and can look
harmless until the source owner or participant context is reviewed.

**Structural solution:** Keep fixtures synthetic or product-neutral and invoke
the Privacy Reviewer before adding publication-facing examples or packets.

**Evidence:** `.roles/stakeholders/privacy-reviewer.md`, `CLAUDE.md`, and
`docs/proof-surface.md`.

**Test:** `tests/pitfall_policy.rs`
