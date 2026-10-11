---
id: US-017
epic: E01
feature: F09
title: The rectangle as a free shape, stretched by its size
status: ready
points: 2
---

# E01 · F09 · US-017 — The rectangle as a free shape, stretched by its size

As a symbol designer, I want the rectangle to be a free shape that I resize by its width and height, so that every shape is resized and reshaped the same way.

## Acceptance criteria

- Given the playground on the rectangle, when width, height and radius are typed, then the drawing is that of F01 and F03 (F09.AC4).
- Given the rectangle, when its width is typed 0 then 100, then it comes back the same (F09.AC3).
- Given the rectangle, when the model is shown, then it is a free shape: its stored points, its width and height, its radii (F09.AC4); the list of points shows the drawn points.
- Given Points mode on the rectangle, when points are selected and rounded, then it works as in F03 (F09.AC4).

## Product Owner test

Run `npm run dev` and open `http://localhost:5173/?step=1` (a 120 × 80 rectangle, radius 10). A **free shape** is a shape kept as its points in a box of their own; its **width** and **height** stretch that box. The rectangle is now one.

| Step | Do                                              | You should see                                                                                                                                      |
| ---- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Look at the drawing and "Model (integers)"      | the same rectangle as before; the model now shows `points` (0, 0), (1, 0), (1, 1), (0, 1) — a unit square — with width 120, height 80 and the radii |
| 2    | Type width 0, then 120                          | the rectangle disappears (no width), then comes back exactly as it was                                                                              |
| 3    | Type height 0, then 80                          | the same, in height                                                                                                                                 |
| 4    | Click **Points**, click point 3, type radius 30 | only the bottom-right corner gets a bigger rounding, as in F03                                                                                      |

Look closely at: whether anything about the rectangle behaves differently from before — it should not.

## Tasks

One task = one commit, referenced as `US-017.Tn`.

- [ ] T1 — Playground: the rectangle read and shown as a free shape; tests (1.5 h)
- [ ] T2 — Product Owner test card; help text (0.5 h)
