---
id: VAL-004
epic: E01
feature: F03
title: Validate F03 on the reference cases
status: done
points: 2
---

# E01 · F03 · VAL-004 — Validate F03 on the reference cases

Replay every acceptance criterion of F03 in the playground and against the derivation's check table.

## Acceptance criteria

- Given the playground, when each reference case of F03 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F03 is `done`.

## Tasks

One task = one commit, referenced as `VAL-004.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F03.AC1]`…`[F03.AC5]`, the playground ones in `playground/mount-playground.test.ts` (1 h)
- [x] T2 — Guided test in the playground: the steps of the US-011, US-012 and US-013 cards (1.5 h)
- [x] T3 — Demo page in `docs/guide/` in plain language (1 h)
- [x] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Tests per criterion (2026-10-11)

Written with the stories and the audit; counted at the validation (`[F03.ACn]` in test titles).

| Criterion | Tests | Where |
| --- | --- | --- |
| F03.AC1 — a requested radius per vertex, clamped locally | 13 | `are-valid-radii`, `corners-with-radii`, `is-valid-rectangle`, `is-valid-regular-polygon`, `rectangle-corners`, `regular-polygon-corners` (properties) |
| F03.AC2 — radius field for every vertex, Mixed | 7 | `common-radius`, playground (`radius-field`) |
| F03.AC3 — radii kept by vertex number | 6 | `resize-radii`, playground (`radius-field`, `points-list`) |
| F03.AC4 — tool bar, handles, point 1 and 2, list | 6 | playground (`points-mode`, `points-list`) |
| F03.AC5 — selection, radius on the selection | 14 | `radii-with-selection`, `selection`, playground (`points-list`) |

## Validation (2026-10-11)

The Product Owner ran the playground of the feature branch from the test cards, before the guided test was written: point 1 and the direction of the outline read as intended, nothing is missing, and they asked for the feature pull request.

Compared with what the feature meant to show: point 1 and point 2 read as the start and the direction of the outline, as meant. No misunderstanding. The guided test of F03 (steps 29 to 33) was added afterwards, for the published playground and the next readers. Remarks:

| Remark | Where it goes |
| --- | --- |
| A polygon could remember the radii of corners that went away (12 → 3 → 12), against F03.AC3 as decided | open question Q26, improvement log |
| No free contour yet, x and y not editable in the list | confirmed in F08 (node editing), next feature |
