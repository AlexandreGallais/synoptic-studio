---
id: US-008
epic: E01
feature: F02
title: Cap the sizes typed in the playground at 100 000
status: done
points: 1
---

# E01 · F02 · US-008 — Cap the sizes typed in the playground at 100 000

As a symbol designer, I want a size typed above 100 000 to become 100 000 so that the interface stays within what a screen can show, while the calculations keep no upper limit (Q20).

Asked by the Product Owner on 2026-10-09, when settling Q20: one user unit is one screen pixel; even a wall of 8K screens stays far below 100 000 pixels; the limit belongs to the interface, never to the calculations.

## Acceptance criteria

- Given the playground, when a width, height or radius above 100 000 is typed, then the input shows 100 000 and the shape is drawn with 100 000.
- Given a value of 100 000 or less, when it is typed, then it is kept exactly.
- Given the library, when a shape of any size ≥ 0 is evaluated, then no upper limit applies (Q20).

## Product Owner test

| Step | Do                            | You should see                                                           |
| ---- | ----------------------------- | ------------------------------------------------------------------------ |
| 1    | Type width 250000             | the input shows 100000; the shape is drawn 100 000 wide (off the canvas) |
| 2    | Type width 100000, then 99999 | each value is kept as typed                                              |
| 3    | Type radius 500000            | the input shows 100000                                                   |

## Tasks

One task = one commit, referenced as `US-008.Tn`.

- [x] T1 — Cap width, height and radius at 100 000 in the playground, with happy-dom tests (1 h)
- [x] T2 — Q20 settled in the domain, feature criterion, help text (0.5 h)
