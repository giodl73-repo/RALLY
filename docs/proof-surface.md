# RALLY proof surface

RALLY keeps a minimal accepted/failure fixture pair for its neutral validation
report contract:

| Fixture | Expected result |
|---|---|
| `fixtures/proof/accepted-report.json` | A finding-free report with `status: pass`. |
| `fixtures/proof/rejected-report.json` | An error finding with stable code, location, message, and `status: error`. |

Run the focused proof from the repository root:

```powershell
cargo test proof_fixtures_record_pass_and_structured_failure
```

The test constructs reports through the public API and compares their complete
machine-readable output with the retained fixtures. A status, field, ordering,
escaping, or structured-failure change must update the fixture intentionally.

The fixtures are product-neutral. Consumer repositories retain ownership of
their scenarios, acceptance thresholds, creative policy, and private playtest
evidence.
