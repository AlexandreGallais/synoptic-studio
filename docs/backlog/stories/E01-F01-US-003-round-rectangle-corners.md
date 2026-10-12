---
id: US-003
epic: E01
feature: F01
title: Round the rectangle's corners in the playground
status: done
points: 2
---

# E01 · F01 · US-003 — Round the rectangle's corners in the playground

As a symbol designer, I want to type a corner radius and see the corners rounded so that I can shape the rectangle exactly.

## Acceptance criteria

- Given a 100 × 50 rectangle, when I type radius 10, then the four corners are rounded with radius 10.
- Given a 100 × 25 rectangle, when I type radius 100, then the effective radius 12.5 is shown next to the requested 100.
- Given any radius, when I change the width, then the rounding follows, up to the requested value (Q8).

## Product Owner test

Run at the validation of F01 (VAL-001), on the local playground the agent starts for you.

Words: the **radius** is how much a corner is rounded — the curve starts that many pixels before the corner; the **requested radius** is the number you type, kept as is; the **effective radius** is the one actually drawn, smaller when the rectangle is too small for the request.

| Step | Do                                     | You should see                                                                                                         |
| ---- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 1    | Open the playground                    | a 120 × 80 rectangle with four rounded corners; under the inputs: "Effective radius: 10 (as requested)"                |
| 2    | Type width 100, height 50, radius 10   | four corners rounded the same way; each curve starts 10 pixels before its corner                                       |
| 3    | Type height 25, keep radius 100        | the short sides become half circles (a "pill"); the text says "Effective radius: 12.5 (requested 100, reduced to fit)" |
| 4    | Type width 100, height 100, radius 50  | a circle                                                                                                               |
| 5    | Type radius 1000                       | the same circle; the text says the radius was reduced to fit, to 50                                                    |
| 6    | Type radius 100, width 100, height 300 | a vertical pill; effective radius 50                                                                                   |
| 7    | Type width 300                         | a square with rounded corners; "Effective radius: 100 (as requested)" — the rounding grew back to what you asked       |
| 8    | Type radius 0                          | sharp corners, as before this feature                                                                                  |
| 9    | Type radius −3, then 2.5               | the inputs are outlined in red and a message says the values must be integers ≥ 0                                      |

Look closely at: whether the rounding behaves as you expect when the rectangle is too small (steps 3, 5, 6), and whether the text explains it clearly.

## Open points

- Where no arc can exist — a rectangle of size 0, an aligned vertex — the effective radius stays the requested one, so the page says "as requested" while nothing is rounded: question Q17, settled by the Product Owner on 2026-10-09: kept as is, the radius applies when the shape grows.

## Tasks

One task = one commit, referenced as `US-003.Tn`.

- [x] T1 — Global requested radius in the rectangle model, its validation, the rectangle's corners and its effective radius (`domain`), tests `[F01.AC2]` (1.5 h)
- [x] T2 — Radius input, effective radius shown next to the requested one, rounded path in the playground (`procedure`) (2 h)
- [x] T3 — Product Owner test card; demonstrated at VAL-001 (0.5 h)
