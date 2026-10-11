---
id: VAL-003
epic: E01
feature: F07
title: Validate F07 on the reference cases
status: ready
points: 2
---

# E01 · F07 · VAL-003 — Validate F07 on the reference cases

Replay every acceptance criterion of F07 in the playground and against the derivation's check table.

## Acceptance criteria

- Given the playground, when each reference case of F07 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F07 is `done`.

## Tasks

One task = one commit, referenced as `VAL-003.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F07.AC1]`…`[F07.AC5]`, the playground ones in `playground/mount-playground.test.ts` (1 h)
- [x] T2 — Guided test in the playground: the steps of the US-010 card (1.5 h)
- [ ] T3 — Demo page in `docs/guide/` in plain language (1 h)
- [ ] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Tests per criterion (2026-10-11)

Written with the stories and the audit; counted at the validation (`[F07.ACn]` in test titles).

| Criterion                                           | Tests | Where                                                                                |
| --------------------------------------------------- | ----- | ------------------------------------------------------------------------------------ |
| F07.AC1 — alignments stored, center by default      | 6     | `is-anchor`, `is-valid-regular-polygon`, `regular-polygon-contour`, playground       |
| F07.AC2 — placement on the anchored edges           | 8     | `fit-in-box`, `regular-polygon-contour`, playground                                  |
| F07.AC3 — no move on the filled axis                | 4     | `fit-in-box`, `regular-polygon-contour`, playground                                  |
| F07.AC4 — the radius does not move the polygon      | 4     | `regular-polygon-corners`, playground; the flat-base property of the audit (AUD-003) |
| F07.AC5 — 3 × 3 anchor grid, keyboard, center first | 3     | playground                                                                           |
