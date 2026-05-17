# Mille Miglia Role Review

Reviewed after wiring AMAZE to RALLY bounded run primitives.

| Role | Finding | Resolution |
|---|---|---|
| Harness Boundary Engineer | Numeric seeds and bounded sampling are product-neutral; room semantics are not. | RALLY owns `RunSeed`, bounded sampling, percent chance, and percent-of helpers. AMAZE keeps room/team/build policy. |
| Simulation Auditor | AMAZE simulation should remain deterministic after extraction. | AMAZE `sim.rs` now wraps RALLY primitives and its deterministic RNG tests still pass. |
| Consumer Advocate | The first consumer should not lose commands during adoption. | AMAZE command surface is unchanged; validation uses existing `simulate` and `score` commands. |
| Evidence Packet Reviewer | Packet manifests are in RALLY, but AMAZE packet contents remain adapter-owned. | No packet file names moved in this pulse. |
| Privacy Reviewer | Shared infra must not copy private room content. | No room content moved into RALLY; active-room smoke tests run in AMAZE only. |

## Follow-up

The next Mille Miglia wave should extract validation-report row conversion or
packet manifest writing once AMAZE has a second call site ready.
