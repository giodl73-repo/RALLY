# Seeded mechanics workbench

Integrated baseline: 411b5c7f0c7d754a6d13bee7b286e205125c86fd, preserving tracked
3c0dbfa6fee16daa9e876ace7dee4b9cc320e022 boundaries alongside public main
4d84433d8d113d5433fd2c97ef4d7e7aed55fbb0.

Reader task: edit an explicit seed and neutral mechanics script; replay dice,
shuffle/draw/discard/recycle, token spend/gain, turns and action budgets; inspect
ordered native BoardEvent JSONL and structured ValidationReport findings; export
the submitted input, report, JSONL, manifest, source/WASM/build/legal artifacts.
One unpublished standalone first-party consumer uses existing APIs. Core mechanics,
normal dependencies/features, root lockfile, RUNE pins, descriptors and consumer
canaries stay unchanged. No consumer migration/adoption or neutral-code deletion
claim. No game, campaign, room, puzzle, player, vendor or playtest fixture is copied.

| Pulse | Scope | Status |
|---|---|---|
| 01 | Bounded standalone consumer, UI, native/worker parity and delivery | implemented; exact release qualification pending |

Companion policy: one to sixteen unique seat IDs, up to 64 unique card IDs,
32 token types with initial counts 0–1,000; initial action budget 0–100; 1–128
uniquely named steps. IDs up to 128 UTF-8 bytes, seed up to 256 bytes, whole
request 128 KiB. Dice count 1–32, sides 1–1,000, extra 0–16, modifier magnitude
at most 10,000. Token/action changes non-negative and bounded; execution cannot
overflow native integer arithmetic. Input fields and operations are strict.
Discard requires the card in the workbench hand, preserving card identity and
conservation; this adapter rule is not new game policy in rally-core.
Spending failure, exhausted draw and missing-hand discard remain structured
findings, not silently successful actions. Continue the transcript after blocked
operations so the exact resulting state is inspectable. No implicit turn/budget
or deck-recycle rules: those operations are explicit in the script.

Dice and deck use separate RNG objects initialized from the same native run ID.
Native dice position counts completed roll calls, not individual random draws.
Run ID is a namespace label from adapter/subject/seed, not a digest of the whole
script. Manifest paths are references, not integrity/authenticity proof. Native
modulo-based randomness remains deterministic and non-cryptographic; this page
does not claim unbiased/fair dice or statistical significance. Validation status
covers only this script's mechanics, not game quality, balance or adoption.

Review in declared order (producer simulations, not independent signoffs):
Privacy Reviewer — authored neutral software fixtures only; no private consumer
content; existing mixed software/content licensing preserved. Simulation Auditor
and Evidence Packet Reviewer — same inputs reproduce exact results and native
JSONL/report strings; blocked cases and complete request remain inspectable.
Harness Boundary Engineer — adapter owns protocol, bounds, hand tracking and
presentation; core and product policy stay unchanged. Consumer Advocate — no
breaking API, adoption gain or consumer readiness claim; existing six canaries
and SCENARIUM compatibility remain regression gates.

Validate core fmt/tests/strict Clippy and release build; companion native tests,
fmt/host+WASM Clippy, boundary and state-conservation tests; exact native/WASM
response fixtures and seed sweep; real browser own files/inputs, exports,
UTF-8/CRLF/inert strings, worker/file deadlines and stale events, keyboard/mobile/
no-JS. Clean pinned Pages artifact, full Git source ZIP and complete deployed-file
hash custody plus fresh live replay precede TRACKER pointer intake. Startup/
compute deadlines: 12/10 seconds; file recovery: five seconds.

Boundary test finding: native escape_json left C0 characters other than newline
literal, producing invalid event/report/manifest JSON. The dedicated serializer
repair and its full-C0 round-trip test are described in
docs/json-control-escaping.md. Existing valid encodings and all compatibility
canaries pass unchanged. This source exception fixes native evidence rather than
rejecting useful string inputs or repairing output only in JavaScript. No RNG,
game mechanic, field, dependency or descriptor changes.

Local qualification: 29 core and seven companion tests pass; core and companion
fmt/strict host+WASM Clippy, native release build/docs and PROOF (77 documents,
zero errors/warnings including installed dependency docs) pass. All 21 browser
cases pass without skips/retries, including 91 exact native/WASM responses and
the 64-seed sweep, own inputs/exports, Unicode/control strings, failed actions,
source/license scope, deadlines/stale events, keyboard/mobile/no-JS. Mobile
results were visually inspected. Clean exact-main hosted/live custody remains
the release gate before TRACKER intake.
