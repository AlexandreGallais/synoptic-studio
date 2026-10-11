---
id: VAL-005
epic: E01
feature: F08
title: Validate F08 on the reference cases
status: done
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

- [x] T1 — One test per criterion `[F08.AC1]`…`[F08.AC4]` (1 h)
- [x] T2 — Guided test in the playground: the steps of the US-014, US-015 and US-016 cards (1.5 h)
- [x] T3 — Demo page in `docs/guide/` in plain language (1 h)
- [x] T4 — Product Owner runs the guided test and validates; feedback recorded; feature pull request into `main` (0.5 h)

## Product Owner's account (2026-10-11)

Tested freely in the playground, before the guided test existed:

- Moving a point: « Putain, c'est génial, ça marche de fou. J'adore, j'adore ça. »
- Deleting: « Je peux delete le point. Oh putain, c'est fait un triangle. » — neighbours joined, confirmed (Q28).
- Adding a point in the middle and moving it: « J'adore, c'est trop bien. »
- Moving a point after a stretch: « ça va mettre à jour tous les points. C'est génial, c'est exactement ça que je voulais. »
- Decimals kept on the coordinate not typed: « Il reste à la virgule. C'est trop de la balle. \[…\] C'est non destructif. » — Q28 settled, ADR-0034.

Routed:

| Remark                                                                                                                  | Where                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Move several selected points at once, relatively (`+10`)                                                                | `docs/domain/interaction.md` §3, for later (editor applications)                         |
| Basic arithmetic in number fields, with parentheses                                                                     | `docs/domain/interaction.md` §3, for later                                               |
| A radius field with the points, the shape's radius for the whole shape, shown in both modes                             | done in this story: a points' Radius field, the shape's field always shown               |
| A field losing the focus at each key, so that typing or holding an arrow key failed                                     | fixed in this story: the field is no longer moved while typed in; a test keeps the focus |
| Free shape, then polygon, then free shape again                                                                         | fixed in this story: the free shape comes back with its own size                         |
| Degenerate shapes: points on one slanted line, two points at the same place, crossing edges; a cap on typed coordinates | Q29 settled: allowed, never changed on their own, flagged in the applications; no cap    |
| A clickable zone laid over a symbol; a shared status zone on every symbol; the saved position of a symbol (Q30)         | `docs/domain/symbols-and-views.md` §1, for later                                         |
| A rotation mode with a movable center, snapping to points and centers, 90° buttons, an angle field                      | F04 feature file, for its refinement                                                     |
