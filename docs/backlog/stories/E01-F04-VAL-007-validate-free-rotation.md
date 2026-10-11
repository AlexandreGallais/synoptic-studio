---
id: VAL-007
epic: E01
feature: F04
title: Validate F04 on the reference cases
status: ready
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
