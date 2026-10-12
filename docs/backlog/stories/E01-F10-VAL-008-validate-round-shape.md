---
id: VAL-008
epic: E01
feature: F10
title: Validate F10 on the reference cases
status: ready
points: 1
---

# E01 · F10 · VAL-008 — Validate F10 on the reference cases

Replay every acceptance criterion of F10 in the playground.

## Acceptance criteria

- Given the playground, when each reference case of F10 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F10 is `done`.

## Tasks

One task = one commit, referenced as `VAL-008.Tn`. This story is not merged automatically (ADR-0028).

- [ ] T1 — One test per criterion `[F10.AC1]`…`[F10.AC4]` (0.5 h)
- [ ] T2 — Guided test in the playground: the steps of the US-020 card (1 h)
- [ ] T3 — Demo page in `docs/guide/` (0.5 h)
