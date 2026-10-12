---
id: EN-006
epic: E01
feature: F01
title: "Rounded contour geometry: segments and arcs"
status: done
points: 5
---

# E01 · F01 · EN-006 — Rounded contour geometry: segments and arcs

Evaluate a contour with effective radii into a closed sequence of segments and circular arcs (ADR-0001, ADR-0005 stage 3).

## Acceptance criteria

Sources: `DERIV-fillet-arc` (check table), `EPSILON` of Q10 introduced by EN-005 (SP-001).

- Given a 100 × 100 square with effective radius 50 everywhere, when evaluated, then the result is 4 quarter arcs and no segment of positive length.
- Given a 100 × 50 rectangle with effective radius 10, when evaluated, then it has 4 segments and 4 quarter arcs, with the tangent points and centers of the `DERIV-fillet-arc` table (e.g. corner `(100, 0)`: from `(90, 0)` to `(100, 10)`, center `(90, 10)`, positive direction).
- Given a concave corner at `V = (10, 0)` (`u_in = (1, 0)`, `u_out = (0, −1)`, `r = 5`), when evaluated, then its arc goes from `(5, 0)` to `(10, −5)` around `(5, −5)` in the negative direction.
- Given radius 0 everywhere, when evaluated, then the result is the sharp contour.
- Given a rectangle of size 0 × 50, when evaluated, then no `NaN` appears.

## Tasks

One task = one commit, referenced as `EN-006.Tn`.

- [x] T1 — Vector helpers `add`, `scale`, `unit` (zero vector → `(0, 0)`), `quarterTurn` (`math`; `REF-MATHWORLD-VECTOR-ADDITION`, `REF-OPENSTAX-CALC3-VECTORS`, `REF-MATHWORLD-UNIT-VECTOR`, `REF-MATHWORLD-ROTATION-MATRIX`) with tests and properties (1.5 h)
- [x] T2 — Segment and arc types (start, end, center, radius, direction) (0.5 h)
- [x] T3 — Fillet of a corner: tangent points, center, direction (`geometry`, `DERIV-fillet-arc`) with the check table (2 h)
- [x] T4 — Closed sequence of segments and arcs; segments shorter than `EPSILON` dropped (`geometry`) (1.5 h)
- [x] T5 — Audit findings: spike limit documented and raised as Q16; `Arc` radius contract; chaining within `EPSILON` and the drop rule stated in the derivation; citations of `unit`, `add` and the references aligned (0.5 h)
