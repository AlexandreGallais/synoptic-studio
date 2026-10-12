---
id: EN-003
epic: E01
feature: F01
title: Turning angle at a contour vertex
status: done
points: 3
---

# E01 · F01 · EN-003 — Turning angle at a contour vertex

Compute, at each vertex of a cyclic contour, the turning angle between the incoming and the outgoing edge (`DERIV-local-radius-clamp`, notation `τᵢ`), for the fillet computations.

## Acceptance criteria

- Given the clockwise rectangle of US-001, when the turning angles are computed, then each one is a quarter turn in absolute value.
- Given three aligned vertices, when the angle at the middle one is computed, then it is 0.
- Given the first vertex of a contour, when its angle is computed, then the incoming edge is the last edge (cyclic contour, Q11).

## Tasks

One task = one commit, referenced as `EN-003.Tn`.

- [x] T1 — Verify a source for the turning angle formula (atan2 of cross and dot products) and record it in `docs/references.md`; otherwise write a derivation (1 h) — `DERIV-turning-angle`, MathWorld and MDN verified
- [x] T2 — Write the tests of the cases above (1 h)
- [x] T3 — Implement vector subtraction, dot and cross products (`math`, one file each) (2 h) — `subtract`, `dot`, `perpDot`
- [x] T4 — Implement the turning angle at a vertex (`geometry`) (1.5 h) — `turningAngle`, `turningAngles`, `cyclicVertex`
