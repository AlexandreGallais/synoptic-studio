---
id: US-020
epic: E01
feature: F10
title: Draw and convert a round shape in the playground
status: ready
points: 2
---

# E01 · F10 · US-020 — Draw and convert a round shape in the playground

As a symbol designer, I want to choose a round shape and type its size, so that I get a circle or a capsule that stays round whatever its size, without a radius to keep at its maximum.

## Acceptance criteria

- Given the shape selector, when « Round » is chosen, then a circle or a capsule fills the box typed, and the radius field and the anchor grid are hidden (F10.AC4, F10.AC1).
- Given a round shape, when its angle or its position is typed, then it turns and moves as every shape (F10.AC2).
- Given a round shape in Points mode, when « Convert to free shape » is clicked, then it becomes a free shape of 4 points with their radii, drawn the same (F10.AC3, F10.AC4).

## Product Owner test

Launch the playground with `npm run dev`, then open `http://localhost:5173/?step=1` (a 120 × 80 box). A **round shape** always fills its box: a **circle** when the box is square, a **capsule** otherwise — two half circles joined by straight sides.

| Step | Do                                                | You should see                                                                                                                                                             |
| ---- | ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | Choose « Round » in the shape selector            | A capsule fills the 120 × 80 box: half circles of radius 40 on the left and the right, straight sides on top and bottom; no Radius field, no anchor grid, no Corners field |
| 2    | Type 100 in Width and 100 in Height               | A circle of diameter 100                                                                                                                                                   |
| 3    | Type 300 in Width and 300 in Height               | Still a circle, of diameter 300: it stays round whatever its size, without any radius to type                                                                              |
| 4    | Type 120 in Width, 80 in Height, then 90 in Angle | The capsule stands up about its center, as every shape (F04)                                                                                                               |
| 5    | Type 0 in Angle, click Points                     | The 4 corners of its box listed, each with radius 40; no point can be typed, Delete is greyed; the « Convert to free shape » button shows                                  |
| 6    | Click « Convert to free shape »                   | Nothing moves; the selector reads « Free shape »; the Radius field comes back; the points can now be moved, added or deleted (F08)                                         |
| 7    | Type 200 in Width                                 | The free shape stretches: the half circles stay of radius 40, the straight sides grow — it is now a rounded rectangle, no longer a round shape                             |

Look closely: before the conversion nothing about radii can be typed; after it, the shape is an ordinary free shape, its radii those it had.

## Tasks

One task = one commit, referenced as `US-020.Tn`.

- [x] T1 — « Round » in the selector, fields shown and hidden, conversion; the tests of `"anchor" in model` replaced by the shape's kind (note 0010); tests (1.5 h)
- [x] T2 — Product Owner test card (0.5 h)
