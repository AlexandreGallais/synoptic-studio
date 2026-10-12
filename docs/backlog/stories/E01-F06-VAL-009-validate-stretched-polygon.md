---
id: VAL-009
epic: E01
feature: F06
title: Validate F06 on the reference cases
status: ready
points: 2
---

# E01 · F06 · VAL-009 — Validate F06 on the reference cases

Replay every acceptance criterion of F06 in the playground.

## Acceptance criteria

- Given the playground, when each reference case of F06 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F06 is `done`.

## Tasks

One task = one commit, referenced as `VAL-009.Tn`. This story is not merged automatically (ADR-0028).

- [x] T1 — One test per criterion `[F06.AC1]`…`[F06.AC5]`: written with the stories (AC1 in 4 tests, AC2 in 12, AC3 in 3, AC4 in 2, AC5 in 5) (0.5 h)
- [x] T2 — Guided test in the playground: the steps of the US-021 card, steps 58 to 66 (1 h)
- [x] T3 — Demo page in `docs/guide/`, with rules C and B side by side, as the Product Owner asked (0.5 h)
