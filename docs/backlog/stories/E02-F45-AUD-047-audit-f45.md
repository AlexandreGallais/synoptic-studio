---
id: AUD-047
epic: E02
feature: F45
title: Audit F45
status: ready
points: 1
---

# E02 · F45 · AUD-047 — Audit F45

Audit F45 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F45's criterion, when the audit ends, then it is traced to a test titled `[F45.AC1] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criterion traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-047.T1`.

## Findings

The `auditor` alone reviewed F45 read-only (ADR-0086); the cause written in US-088 holds, and its fix covers every `change` the page sends a field of its own, `applySteppedValue`'s alone (`listen-to-number-fields.ts`). Each finding below reproduced before it was fixed. F45's criterion is traced to the `[F45.AC1]` tests of `playground/handles/handles-stepped-by-arrows.test.ts`.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | The side method's tests and the point's X and Y test were titled `[F45.AC2]`, a criterion F45 lacks, the page method's `[F45.AC1] [F45.AC2]` | fixed: every one titled `[F45.AC1]`, the four handle fields, both methods and every mode being F45's one criterion |
| 2 | Letter spacing and Word spacing, from −100 000 to 100 000, doubled at each arrow as Before y did, untested | reproduced with US-088's fix undone: −5 and Up gave −9. Tests: −5, Up −4, Shift + Up 6, in both fields (`own-number-fields.test.ts`) |
| 3 | A field of its own showing a fraction applied nothing at an arrow or a relative value: rounded only for a fraction typed (`wholeValueWithTyped`), 11.83476 + 1 gave 12.83476, which the handles' and the points' listeners refuse (`Number.isSafeInteger`), the field showing it and the handle unmoved; reached by Mirror angle, the side method and a converted shape's point | the main thread's choice, from the Product Owner's rule of 2026-10-24 for every number field: a field of its own holding whole numbers rounds its result whatever was typed (`readComputedValue`, `nearestInteger`), the shapes' X, Y, W and H keeping a shape's own fraction for an integer typed (Q27); the listeners' guards kept, always passing for these fields. Tests: Mirror angle, After x 11.83476 and Up 13, Before (−11.1649, −4.62007); from the side, After 22.61986° and Up 24, Before 247.38014° and `+1` 248; the hexagon's point 1 converted at (75, 93.30127), Up or `+1` in Y 94; `readOwnValue` 33.54102 + 1 35, 0.5 − 1 0. `interaction.md` §2 (Every number field, Moving a vertex), Q56, the guide pages of F08 and F38 |

Mutations (A4), `npm run mutate -- --all`, the work staged first: the 3 modules F45 and this audit changed, `listen-to-number-fields.ts`, `stepped-changes.ts` and `read-own-value.ts`, against `handles-stepped-by-arrows.test.ts`, `own-number-fields.test.ts` and `free-shape-editing.test.ts`: 18 caught and 3 survivors, all equivalent, on lines AUD-046 wrote: `readValueInRange`'s `value - least` and `most - least` turned `+`, every range that goes round starting at 0, the Angle's and the hue's, and `readOwnValue`'s 0 to start a fixed value from turned 1, a fixed value reading no number, as AUD-046 found. `stepped-changes.ts` holds a constant alone, no mutation in it.

By hand, the criterion once: `applyTypedValue`'s check of `STEPPED_CHANGES` dropped, 11 `[F45.AC1]` tests fail, each with a negative value shown (Before y's steps, Before x and Before y on the page in every mode, the point's X and Y), the side method's, showing positive values, passing; the 2 spacing tests and the Mirror angle rounding fail with it. The rounding put back to a fraction typed alone: the 8 new tests of finding 3 fail.

## For the main thread

- For the done commit's footer: the rule docs changed, `interaction.md` §2 (Every number field, Moving a vertex), the Q row Q56 of `docs/domain/README.md`, and the guide pages of F08 and F38; F45 has no guide page of its own and VAL-046 is dropped.
