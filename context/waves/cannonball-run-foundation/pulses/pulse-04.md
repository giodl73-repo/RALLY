# Pulse 04: PITFALL adoption

## Goal

Add a repo-local PITFALL doctrine layer so RALLY's neutral harness boundary,
deterministic proof surface, consumer compatibility, RUNE metadata, and privacy
risks are visible to portfolio tooling.

## Change

- Added `.pitfall/PITFALL.md` as the local doctrine index.
- Added `.pitfall/PRINCIPLES.md` with five RALLY decision principles.
- Added `.pitfall/INVARIANTS.md` with five neutral-core, fixture,
  compatibility, RUNE, and privacy invariants.
- Added `.pitfall/PITFALLS.md` with five recurring failure patterns.

## Findings

PITFALL adoption confirmed one existing mitigation:

- `RALLY-PF-03`: accepted and rejected validation-report fixtures already
  mitigate success-only proof drift.

PITFALL adoption surfaced four open repo-local issues:

- `RALLY-PF-01`: shared harness helpers can absorb product policy.
- `RALLY-PF-02`: green provider canaries can be mistaken for consumer
  readiness.
- `RALLY-PF-04`: RUNE metadata can be overread as product contract authority.
- `RALLY-PF-05`: shared evidence packets can leak private playtest detail.

## Validation

```powershell
C:\Users\giodl\.cargo\bin\cargo.exe run --manifest-path C:\src\TRACKER\repos\standards-protocols\pitfall\Cargo.toml -q -p pitfall-cli -- validate C:\src\TRACKER\repos\games-design\rally --format json
python C:\src\TRACKER\repos\standards-protocols\pitfall\tools\check_pitfall.py C:\src\TRACKER\repos\games-design\rally
C:\Users\giodl\.cargo\bin\cargo.exe fmt --check
C:\Users\giodl\.cargo\bin\cargo.exe test proof_fixtures_record_pass_and_structured_failure
C:\Users\giodl\.cargo\bin\cargo.exe test --test consumer_contracts
git diff --check
```
