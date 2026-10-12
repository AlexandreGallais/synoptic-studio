---
id: VAL-002
epic: E01
feature: F02
title: Validate F02 on the reference cases
status: done
points: 2
---

# E01 · F02 · VAL-002 — Validate F02 on the reference cases

Replay every acceptance criterion of F02 in the playground and against the derivation's check table.

## Acceptance criteria

- Given the playground, when each reference case of F02 is entered, then the drawing and the effective radius match the expected values.
- Given the check table of `DERIV-regular-polygon-fit`, when each case is run, then the results match.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F02 is `done`.

## Tasks

One task = one commit, referenced as `VAL-002.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F02.AC1]`…`[F02.AC5]`, the playground ones in `playground/mount-playground.test.ts` (1.5 h)
- [x] T2 — Guided test in the playground: the steps of the US-007 card, then a capped size (US-008 card, F02.AC5), one at a time (2 h)
- [x] T3 — Demo page in `docs/guide/` in plain language (fit in a box, inscribed circle explained) (1.5 h)
- [x] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Tests per criterion (2026-10-09)

Written with the stories and the audit; counted at the validation (`[F02.ACn]` in test titles).

| Criterion                                                  | Tests | Where                                                                                     |
| ---------------------------------------------------------- | ----- | ----------------------------------------------------------------------------------------- |
| F02.AC1 — uniform fit, flat base, refused values           | 12    | `unit-regular-polygon`, `regular-polygon-contour`, `is-valid-regular-polygon`, playground |
| F02.AC2 — global radius, incircle at most                  | 6     | `regular-polygon-corners`, playground                                                     |
| F02.AC3 — 5 decimals, clockwise, starting vertex           | 7     | `unit-regular-polygon`, `regular-polygon-contour`, playground                             |
| F02.AC4 — shape selector                                   | 6     | playground                                                                                |
| F02.AC5 — sizes capped at 100 000, no limit in the library | 3     | playground, `is-valid-rectangle`                                                          |

## Validation (2026-10-09)

The Product Owner ran the playground of the feature branch themselves (`npm run dev`), before the guided test was written: it works as they meant, and they closed the feature.

Compared with what the feature meant to show: no misunderstanding reported. The guided test of F02 (steps 10–20 of the playground) was added afterwards, for the published playground and the next readers. Remarks raised during the validation, none blocking F02:

| Remark                                                                                                           | Where it goes        |
| ---------------------------------------------------------------------------------------------------------------- | -------------------- |
| Largest size of a shape: one unit is one screen pixel, no limit in the calculations, interface capped at 100 000 | Q20 settled, US-008  |
| A polygon stretched to fill its whole box, as a second kind next to the fixed one                                | draft feature F06    |
| The View Editor could ask for the target screen                                                                  | improvement log, E08 |
| Oversized SVG imports accepted without loss, flagged, simplified when possible, refused when too complex         | E11, improvement log |
