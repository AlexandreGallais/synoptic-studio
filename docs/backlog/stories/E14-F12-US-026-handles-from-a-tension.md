---
id: US-026
epic: E14
feature: F12
title: "Smooth a point by a tension, its handles computed"
status: ready
points: 3
---

# E14 · F12 · US-026 — Smooth a point by a tension, its handles computed

As a symbol designer, I smooth a point by typing one tension, its handles computed from the points before and after it, as Inkscape's auto-smooth nodes and the cardinal spline do (`REF-INKSCAPE-NODE-TYPES`, `REF-WIKI-HERMITE`), so that the same tension gives the same kind of curve at every point.

## Acceptance criteria

- Given a point A (100, 0) between (0, 0) and (200, 100), when the user types tension 0 %, then A's handles are ±(next − previous) / 6 = ±(33.33333, 16.66667) (Catmull–Rom, a derivation from `m = (next − previous) / 2` and handles `m / 3`).
- Given tension 100 %, then both handles are (0, 0): the sides are straight at A.
- Given a tension outside 0 to 100, or not an integer, then it is refused.
- Given the neighbours move, when the point is in tension mode, then its handles follow them, as an auto-smooth node does.

## Tasks

One task = one commit, referenced as `US-026.Tn`.

- [ ] T1 — Derivation and function of handles from a tension
- [ ] T2 — The tension field in the panel; handles kept up to date; tests
