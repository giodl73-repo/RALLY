# Reproduce the seeded mechanics workbench

The unpublished standalone web/adapter consumer calls existing rally-core APIs.
Core mechanics, manifest, lockfile, RUNE pins, descriptors and consumer canaries
are unchanged from integrated baseline 411b5c7f. A focused native serializer fix
escapes previously invalid C0 strings; see docs/json-control-escaping.md. Existing
valid output remains unchanged. No product game rules, consumer
migration readiness or deletion score is added. Fixtures are authored neutral
software test data; no private room, campaign, player, vendor or playtest content
is copied. Software and ordinary software documentation follow MIT; original
non-software content retains the repository's CC BY-NC 4.0 scope. LICENSE and
upstream notices remain intact; no blanket relicensing.

Install Rust 1.95.0 with rustfmt, Clippy and wasm32-unknown-unknown, Python 3.12,
Node 22 and wasm-bindgen-cli 0.2.127. Then:

```sh
cargo +1.95.0 test --locked
cargo +1.95.0 clippy --locked --all-targets -- -D warnings
cargo +1.95.0 test --locked --manifest-path web/adapter/Cargo.toml --all-features
npm ci
npx playwright install chromium
python -B tools/build-pages.py
npm run test:pages
python -B tools/serve-pages.py
```

Open http://127.0.0.1:4177/RALLY/. The builder explicitly selects Rust 1.95.0;
CI pins and checksums bindings and uses the npm lockfile. Metadata resolves the
actual adapter dependency closure, including proc macros and unchanged pinned
Git RUNE crates. Their repository-level LICENSE is bundled even where package
manifests omit SPDX metadata. Rust library notices accompany the compiled WASM.
Clean source.zip and provenance identify the exact commit; dirty previews are
labelled and include staged files, not release evidence. The source archive,
per-file hashes, native response fixtures and browser artifacts enable custody
checks. Only the tested main artifact deploys; TRACKER records full live hashes
and a fresh committed browser replay afterward.

## Request and operations

Supply seed, subject, ordered seats/card IDs, token counts, action_budget and
named steps. Steps are roll (expression/keep/extra), shuffle, draw (count),
discard (card), recycle, spend/gain (token/amount), advance_turn, spend_action
(amount), refresh_actions. Unknown fields/operations and invalid admission fail
before execution. Every operation is explicit: no automatic recycling, budget
refresh or game phase policy. Discard requires the card in the workbench hand.
Native recycle appends discards and shuffles the entire remaining draw pile.
Blocked spends/exhausted draws generate warnings; a missing-hand discard is a
structured error. The transcript continues, exposing its actual resulting state.

Bounds: request 128 KiB UTF-8; seed 256 bytes, IDs/subject 128 bytes; 1–16 unique
seats, up to 64 unique cards, 32 named token pools (initial 0–1,000) and 1–128
unique steps. Dice count 1–32, sides 1–1,000, extra 0–16, modifier magnitude
10,000. Token changes max 1,000 and action changes/budget max 100. These bounds
keep core integer arithmetic and loops portable; they are companion policy,
not native API restrictions or a browser heap quota.

Deck and DiceRoller have separate native RNG objects initialized from the same
native run ID. Native roll position counts completed roll calls, not individual
random draws. Native modulo RNG remains deterministic/non-cryptographic; this
tool makes no unbiased/fair-dice or statistical-significance claim. Run ID is
an adapter/subject/seed label, not a digest of the full script. The complete
submitted input is retained in the workbench report. Native manifest paths are
references, not integrity or authenticity proof. Mechanics pass/review/error
does not approve a game, migration or private-content publication.

## Downloads and recovery

Original input preserves valid opened UTF-8/CRLF until editing, successful
replacement or example/reset. Submitted request is the exact editor text passed
to Rust. Native event JSONL, validation JSON and manifest JSON download as their
unchanged core strings; the complete report includes all step outcomes/states.
Manifest paths match the submitted-request/event/validation/report download names.
Strings are rendered inertly. No user input is uploaded or persisted.

Worker startup/compute deadlines are 12/10 seconds; file reads recover after five
seconds. Edit/reset/retry epochs fence stale completions. Failure retains drafts
and independent input/source downloads. Browser qualification uses real WASM,
native response fixtures including a 64-seed sweep, own files/inputs, exact
exports, blocked actions, control/Unicode strings, recovery, keyboard, no-JS and
desktop/mobile views. Provider canaries, SCENARIUM compatibility and old privacy/
boundary evidence retain their original authority and are regression checks.
