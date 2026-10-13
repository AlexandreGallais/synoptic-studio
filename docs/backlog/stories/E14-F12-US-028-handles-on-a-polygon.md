---
id: US-028
epic: E14
feature: F12
title: Give handles to a point of a regular polygon
status: ready
points: 1
---

# E14 · F12 · US-028 — Give handles to a point of a regular polygon

As a designer, I want to switch a point of a regular polygon to handles, so that I can bend a hexagon's side without converting it by hand first.

A regular polygon's points are computed from its corner count and its box: a point with handles would no longer be regular. As for deleting a point (F08), the polygon is converted to a free shape first, without moving (Product Owner, VAL-011, 2026-10-13).

## Acceptance criteria

- Given a hexagon 100 × 100 in Points mode and its point 1 selected, when **Handles** is pressed, then the shape becomes a free shape of 6 points, the drawing unchanged, and point 1 is in handles mode, Mirror angle and length, its handles (0, 0).
- Given the same, when the radius of the polygon was 10, then every corner keeps its radius 10 and point 1's radius is kept under its handles.

## Tasks

One task = one commit, referenced as `US-028.Tn`.

- [ ] T1 — The switch converts a polygon first; playground test and test card (1 h)
