---
id: VAL-005
epic: E01
feature: F08
title: Validate F08 on the reference cases
status: ready
points: 2
---

# E01 · F08 · VAL-005 — Validate F08 on the reference cases

Replay every acceptance criterion of F08 in the playground.

## Acceptance criteria

- Given the playground, when each reference case of F08 is entered, then the drawing matches the expected values.
- Given the demo page, when the Product Owner follows its guided test, then each step is ticked or commented, and misunderstandings become backlog items (ADR-0023).
- Given the review, when the Product Owner validates, then F08 is `done`.

## Tasks

One task = one commit, referenced as `VAL-005.Tn`. This story is not merged automatically (ADR-0028).

- [ ] T1 — One test per criterion `[F08.AC1]`…`[F08.AC4]` (1 h)
- [ ] T2 — Guided test in the playground: the steps of the US-014, US-015 and US-016 cards (1.5 h)
- [ ] T3 — Demo page in `docs/guide/` in plain language (1 h)
- [ ] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)
      | A clickable zone laid over a symbol; a shared status zone on every symbol; the saved position of a symbol (Q30) | `docs/domain/symbols-and-views.md` §1, for later |
      | A rotation mode with a movable center, snapping to points and centers, 90° buttons, an angle field | F04 feature file, for its refinement |
