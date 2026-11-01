# Native JSON control-character repair

The mechanics workbench's boundary tests found that the shared escape_json
helper escaped quotes, backslashes and newline but left other U+0000–U+001F
characters literal. Such event, finding, report and manifest strings produced
invalid JSON, including ordinary tab and carriage-return inputs.

The helper now escapes all remaining C0 characters with JSON Unicode escapes.
Existing encodings for quote, backslash and newline and all other valid string
content stay byte-identical. No fields, API, RNG sequence, descriptor, dependency
or consumer rule changes. This repairs previously invalid serialization rather
than restricting accepted IDs or replacing native output in the browser.

Regression: tests/json_control_characters.rs round-trips every C0 character in
native BoardEvent JSONL, validation and manifest fields and checks one physical
event line. Existing proof/consumer/SCENARIUM/RUNE/PITFALL canaries remain gates;
their valid output fixtures must remain unchanged. A browser fixture exercises
control characters and inert Unicode/HTML strings through real worker WASM.

Producer review in the declared order: Privacy Reviewer confirms authored
synthetic strings only; Simulation Auditor confirms unchanged seeds/mechanics;
Evidence Packet Reviewer requires valid lossless native JSON rather than a JS
repair; Harness Boundary Engineer keeps the fix in the shared serializer;
Consumer Advocate requires unchanged valid canaries and records that this does
not approve any consumer migration. No independent reviewer signoff is claimed.
