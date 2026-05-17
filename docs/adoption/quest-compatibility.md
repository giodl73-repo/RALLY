# QUEST Compatibility Plan

## Decision

QUEST should move deterministic mechanics toward RALLY gradually. The current
Python engine remains the source of behavior until Rust parity is proven with
fixtures.

## Compatibility surfaces

| Surface | QUEST current role | RALLY target |
|---|---|---|
| Dice | Seeded rolls with JSONL audit entries. | Seeded roll stream and logged mechanical outcomes. |
| Event log | Session event JSONL for spells, saves, reactions, conditions, and near-death events. | Product-neutral event rows with QUEST-owned event taxonomy. |
| Checkpoint | Re-entrant session state before LLM-required beats. | Schema validation and stable scenario/scene/beat references. |
| Mechanical beat | Python resolves mechanical beats while narrative remains LLM-owned. | Adapter validates whether a beat is mechanically resolvable and logs the outcome. |

## Parity fixtures

The first QUEST adapter wave should create fixture-driven checks before replacing
behavior:

1. Same seed and roll expression produce the same result sequence.
2. Existing event-log rows can be mapped into RALLY event rows without losing
   session, scene, beat, event type, or message.
3. A checkpoint with missing scene, beat, or party state fields yields validation
   findings instead of silent success.
4. A known mechanically resolvable beat produces the same state delta in Python
   and the Rust adapter.

## Migration order

1. Add read-only RALLY validation over QUEST logs/checkpoints.
2. Add dice parity fixtures.
3. Add mechanical beat validation fixtures.
4. Replace Python internals only when parity tests cover the behavior.

## Non-goals

- Do not rewrite QUEST narrative generation.
- Do not move Dragonlance canon, party state, or campaign policy into RALLY core.
- Do not retroactively rescore old sessions.
