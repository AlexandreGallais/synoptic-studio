---
id: US-011
epic: E01
feature: F03
title: Radius field for every corner, Mixed
status: done
points: 2
---

# E01 · F03 · US-011 — Radius field for every corner, Mixed

As a symbol designer, I want the radius field of a shape to round every corner at once, and to say "Mixed" when the corners differ, so that I see at a glance whether the shape is rounded evenly.

## Acceptance criteria

- Given a shape whose vertices have the same radius, when it is shown, then the radius field shows that radius (F03.AC2).
- Given a shape whose vertices have different radii, when it is shown, then the radius field is empty and shows "Mixed"; typing a value gives it to every vertex; the effective radii are then shown per vertex in Points mode, not in one line (F03.AC2).
- Given a hexagon with radii 10, 0, 0, 0, 0, 0, when it gets 5 corners, then its radii are 10, 0, 0, 0, 0; with 8 corners, 10 and seven 0 (F03.AC3).

## Product Owner test

Nothing new to see yet: corners get different radii only in Points mode, which comes with US-012 and US-013. "Mixed" and the radii kept when the number of corners changes are tried in US-013's card. Here, check that nothing changed.

Run `npm run dev` and open `http://localhost:5173/?guide=epic&step=10`. The **radius** is how much each corner is rounded (the arrondi).

| Step | Do | You should see |
| --- | --- | --- |
| 1 | On the hexagon of step 10, type radius 10 | every corner rounded alike; "Effective radius: 10 (as requested)" |
| 2 | Type 5 corners, then 8 | the same rounding on every corner of the pentagon, then the octagon |
| 3 | Open "Words" under the guided test | a sentence explains "Mixed" |

## Tasks

One task = one commit, referenced as `US-011.Tn`.

- [x] T1 — Radii kept by vertex number when the number of corners changes; the common radius of a shape or none (Mixed, `REF-FIGMA-CR`); tests (1.5 h)
- [x] T2 — Playground: radius field showing Mixed, typing gives every vertex the value; Product Owner test card (1 h)
