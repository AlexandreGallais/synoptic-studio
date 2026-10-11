---
id: US-011
epic: E01
feature: F03
title: Radius field for every corner, Mixed
status: ready
points: 2
---

# E01 · F03 · US-011 — Radius field for every corner, Mixed

As a symbol designer, I want the radius field of a shape to round every corner at once, and to say "Mixed" when the corners differ, so that I see at a glance whether the shape is rounded evenly.

## Acceptance criteria

- Given a shape whose vertices have the same radius, when it is shown, then the radius field shows that radius (F03.AC2).
- Given a shape whose vertices have different radii, when it is shown, then the radius field is empty and shows "Mixed"; typing a value gives it to every vertex (F03.AC2).
- Given a hexagon with radii 10, 0, 0, 0, 0, 0, when it gets 5 corners, then its radii are 10, 0, 0, 0, 0; with 8 corners, 10 and seven 0 (F03.AC3).

## Product Owner test

Written with the story.

## Tasks

One task = one commit, referenced as `US-011.Tn`.

- [ ] T1 — Radii kept by vertex number when the number of corners changes; the common radius of a shape or none (Mixed, `REF-FIGMA-CR`); tests (1.5 h)
- [ ] T2 — Playground: radius field showing Mixed, typing gives every vertex the value; Product Owner test card (1 h)
