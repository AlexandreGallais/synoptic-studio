---
id: VAL-007
epic: E01
feature: F04
title: Validate F04 on the reference cases
status: done
points: 2
---

# E01 · F04 · VAL-007 — Validate F04 on the reference cases

Replay every acceptance criterion of F04 in the playground.

## Acceptance criteria

- Given the playground, when each reference case of F04 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F04 is `done`.

## Tasks

One task = one commit, referenced as `VAL-007.Tn`. This story is not merged automatically (ADR-0028).

- [x] T0 — The view stays put: the Product Owner's first try saw a turned shape slide (the view followed it into negative coordinates) (0.5 h)
- [x] T1 — One test per criterion `[F04.AC1]`…`[F04.AC5]`: written with the stories (AC1 in 7 test files, AC2 in 2, AC3 in 12, AC4 in 6, AC5 in the playground) (1 h)
- [x] T2 — Guided test in the playground: the steps of the US-018 and US-019 cards, steps 47 to 57 (1.5 h)
- [x] T3 — Demo page in `docs/guide/` (1 h)

## Validation (2026-10-12)

First try, before the guided test:

- Moving and resizing a turned shape, and the width of a turned shape, work as the Product Owner meant.
- All the features of E01 work together on one shape.
- The two boxes read as meant: the straight box dashed, the turned box in blue.
- A defect: the shape did not seem to turn about its center, because the view followed a turned shape into negative coordinates; fixed at T0, the view stays put, and a shape may leave it.

Guided test, steps 47 to 57: everything behaved as expected.

Routed:

- The rotation mode, its movable center and the handles on both boxes: E02 (ideas of the F04 refinement).
- The Product Owner's requests on the speed of the work (live view of scripts, the CI only once, fewer waits): improvement log, for the end of E01.
- The round shape: F10, next in E01.
