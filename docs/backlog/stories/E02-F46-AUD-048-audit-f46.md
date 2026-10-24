---
id: AUD-048
epic: E02
feature: F46
title: Audit F46
status: ready
points: 1
---

# E02 · F46 · AUD-048 — Audit F46

Audit F46 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F46's criteria, when the audit ends, then each is traced to a test titled `[F46.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-048.T1`.

## Findings

The `auditor` alone reviewed F46 read-only (ADR-0086) and found no wrong result in its code. F46's criteria are traced to the `[F46.AC1]` tests of `landed-nudge-shift.test.ts`, `field-arrow-shift.test.ts`, `nudge-shapes.test.ts`, `edited-fields.test.ts` and `several-arrow-steps.test.ts`, the `[F46.AC2]` tests of `press-left-open.test.ts` and the `[F46.AC3]` tests of `canvas-size.test.ts`.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | `interaction.md` §2 (The arrows) and US-089's first criterion said the axis moved goes "less than" 1 or 10 from a fraction, where round(a + n) − a lies within half a unit of it, above as below: 6.65064 to 8 goes by 1.34936 | fixed: "within half a unit of 1 or 10 from a fraction, never 0 nor backward", in both |
| 2 | US-090's cause rests on an order of keys inferred, Space, Cmd, Space released, never observed, where `REF-WEBKIT-CMD-KEYUP` and `REF-MOZILLA-CMD-KEYUP` describe Cmd pressed first | fixed: US-090 and `interaction.md` §7 (A drag that pans) say the order was inferred and that Cmd's release letting Space go covers both orders; the Product Owner's test step 2 kept |
| 3 | `nudgeShift`'s TSDoc named `landedShift` for the landing, which is `landedNudgeShift` | fixed |
| 4 | Several shapes' X and Y added the step to each shape's own value, a fraction kept, where one shape's land on integers (US-089): `listenToSeveralFields` dropped `isStep` | the main thread's decision, built: an arrow in several shapes' X or Y lands each shape's own straight box's top-left on integers as one shape's (`applySeveralValue`, `readSteppedKept`, `fieldArrowShift`), X then Y set by their shifts as relative values, the plain step for a shape without a straight box; the drawing's arrows still land several shapes together by their dashed box. Reproduced before the fix: A′ turned at (4.19873, −1.65064) and A at (10, 20), Up in X gave (5.19873, −1.65064). Tests: Up in X (5, −2) and (11, 20), Down in X (3, −2) and (9, 20), Up in Y (4, −1) and (10, 21), Shift + Up next (15, −2) and (21, 20), `+1` typed keeping the fraction, a text never measured left as it is, Up held 30 keys one step of the history undone by Ctrl + Z. `interaction.md` §2 (The arrows, An arrow in one shape's X or Y), Q56, the guide pages of F38 and F39 |

Mutations (A4), `mutate --all`, the work staged first, on `apply-several-value.ts` and `listen-to-several-fields.ts` against `several-arrow-steps.test.ts` and `write-several-values.test.ts`: 10 caught, 0 survived; `listen-to-several-fields.ts` has no mutation site. `isStep` dropped by hand, `+1` typed then landing: the new `+1` test and 3 `[F38.AC2]` tests fail.

By hand, each criterion once, against the tests of `edited-fields`, `selection-keys`, `selection`, `selection-drag`, `view` and `src/model/moving`: criterion 1, `landedNudgeShift` returning the nudge unlanded, 28 `[F46.AC1]` tests fail and 1 `[F39.AC4]`; several shapes' fields given the plain step, the 6 `[F46.AC1]` tests of `several-arrow-steps.test.ts` but the `+1` one fail; criterion 2, Cmd's release no longer letting Space go, 1 `[F46.AC2]` test fails, and a drag whose release was never heard kept at the next press, 1 other; criterion 3, the resized canvas's redraw left to the next frame, 2 `[F46.AC3]` tests fail.

## For the main thread

- For the done commit's footer: the rule docs changed, `interaction.md` §2 (The arrows, An arrow in one shape's X or Y) and §7 (A drag that pans), the Q row Q56 of `docs/domain/README.md`, and the guide pages of F38 and F39; F46 has no guide page of its own and VAL-047 is dropped.
