# RALLY Invariants

## RALLY-I-01: Proof Fixtures Cover Pass And Structured Failure

**Status:** VERIFIED

**Invariant:** RALLY retains both an accepted validation report fixture and a
structured failure fixture.

**Why it matters:** A validation system that only proves success can lose error
shape, status, location, code, or message compatibility.

**Evidence:** `docs/proof-surface.md`, `fixtures/proof/accepted-report.json`,
and `fixtures/proof/rejected-report.json`.

**Test:** `cargo test proof_fixtures_record_pass_and_structured_failure`.

## RALLY-I-02: Consumer Canaries Stay Product-Neutral

**Status:** VERIFIED

**Invariant:** The retained consumer compatibility matrix names RALLY-owned
neutral surfaces separately from consumer-owned semantics.

**Why it matters:** Shared infrastructure adoption should not silently move
creative, economic, or release authority into RALLY.

**Evidence:** `docs/consumer-compatibility.md` and
`contracts/consumer-surfaces.json`.

**Test:** `cargo test --test consumer_contracts`.

## RALLY-I-03: Breaking Changes Require Migration And Rollback Evidence

**Status:** ENFORCED

**Invariant:** Existing constructors, field meanings, deterministic sequences,
status values, and JSON field names in retained projections do not change
silently.

**Why it matters:** Consumers need predictable migration paths and the ability
to stay on the last passing revision.

**Enforcement:** `docs/consumer-compatibility.md` requires version or projection
changes, affected-consumer identification, provider and consumer tests,
migration notes, rollback instructions, and owner approval.

**Evidence:** `.roles/stakeholders/consumer-advocate.md`.

## RALLY-I-04: RUNE Contract Fixtures Match Registered Types

**Status:** VERIFIED

**Invariant:** RALLY's RUNE descriptor collection covers the neutral
simulation, metric, comparison, finding, and validation-report spine.

**Why it matters:** Descriptor drift would let generated contract metadata
appear current after Rust fields or report semantics changed.

**Evidence:** `docs/rune/simulation_contracts.json` and `src/lib.rs`.

**Test:** `cargo test rune_contract_registry_matches_retained_fixture`.

## RALLY-I-05: Private Content Does Not Enter Shared Fixtures

**Status:** ENFORCED

**Invariant:** RALLY fixtures remain product-neutral and do not contain private
room, campaign, vendor, player, or playtest evidence.

**Why it matters:** Shared infrastructure should not become an accidental
publication channel for private consumer material.

**Enforcement:** `.roles/stakeholders/privacy-reviewer.md` blocks unsafe
fixtures, examples, packets, or publication artifacts.

**Evidence:** `CLAUDE.md`, `README.md`, and `docs/proof-surface.md`.

## RALLY-I-06: Shared Simulation Boundaries Stay Machine-Readable

**Status:** VERIFIED

**Invariant:** RALLY keeps a machine-readable boundary manifest for neutral
simulation primitives, consumer-owned semantics, provider canaries, migration
readiness, private playtest evidence, and RUNE metadata authority.

**Why it matters:** Shared infrastructure earns reuse by being generic, but
generic tests and examples can look like product policy, migration approval, or
permission to publish private playtest detail.

**Enforcement:** `tests/pitfall_policy.rs` parses the boundary manifest and
requires blocked claims for shared harness policy, provider canary readiness,
and private evidence packets.

**Evidence:** `docs/shared-simulation-boundaries.v1.json` and
`tests/pitfall_policy.rs`.
