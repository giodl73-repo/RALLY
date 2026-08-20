# RALLY consumer compatibility

RALLY has six source-level consumers in the portfolio. The supported boundary is
the neutral Rust primitives each consumer actually imports, not the consumer's
game, simulation, economic, creative, or review policy.

## Retained consumer matrix

| Consumer | RALLY-owned surface | Consumer-owned semantics |
|---|---|---|
| AMAZE | Seeded sampling and percentage helpers | Room behavior, timing, safety, and scoring |
| QUEST | Deterministic dice parsing and outcomes | Campaign state, narrative, and mechanical policy |
| HUNT | Simulation runs, metrics, findings, and reports | Puzzle graph, hints, solver, and publish readiness |
| TIGRIS | Turn order, scores, token pools, and neutral reports | Parliament axes, stakes, personas, and board policy |
| BANISH | Seeded runs, actor traces, metrics, comparisons, and reports | Settlement rules, economics, and scenario claims |
| CERES | Runs, events, comparisons, reports, and packet manifests | Market, cooperative, civic, and evidence policy |

The machine-readable matrix is
[`contracts/consumer-surfaces.json`](../contracts/consumer-surfaces.json). Run
its provider-side rehearsal with:

```powershell
cargo test --test consumer_contracts
```

The test exercises each consumer's imported public surface and compares the
result with a retained representative projection. The rows are compatibility
canaries, not exhaustive inventories of every imported symbol. The existing
proof fixture test separately protects accepted and structured-error validation
reports.

## Compatibility and lifecycle

RALLY is pre-1.0. Consumers should pin a tested revision. Existing constructors,
field meanings, deterministic sequences, status values, and JSON field names in
the retained matrix must not change silently.

A breaking change requires:

1. a new consumer-surface contract version or an explicit changed projection;
2. identification of every affected recorded consumer;
3. a passing provider rehearsal plus the affected consumer's own tests;
4. migration and rollback instructions; and
5. owner approval from both RALLY and the affected consumer.

A surface may be deprecated only with a replacement or removal reason and an
explicit removal condition. Consumers may remain on the last passing RALLY
revision while migrating. Rollback restores that revision and its retained
projection; it must not move consumer policy into RALLY or rewrite historical
evidence to conceal drift.

## Review findings

- **Shared substrate owner:** accepted; one provider rehearsal protects the
  neutral primitives already used by six repositories.
- **Consumer operator:** accepted; each row names the consumer-owned semantics
  and requires consumer tests before migration.
- **Scope and simplicity reviewer:** accepted; no adapter framework, runtime
  discovery, policy schema, or automatic upgrade mechanism was added.
