---
id: SP-001
epic: E01
feature: F01
title: Research for the remaining stories of F01
status: done
points: 2
---

# E01 · F01 · SP-001 — Research for the remaining stories of F01

Find, read and record the sources of EN-005, EN-006, EN-007, US-003 and VAL-001 before implementing them (ADR-0020, added after F01 was refined).

## Acceptance criteria

- Given EN-005 to VAL-001, when the spike ends, then every formula and behavior they need has a verified `REF-*`, a `DERIV-*` or an open research request.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and research note 0003 records what was found and what remains open.

## Questions

Inventory (T1): what each remaining story needs, and where it comes from.

| Story   | Needs                                                                                                                                        | Source                                                                                                          |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| EN-005  | edge length `L = ‖e‖`                                                                                                                        | Euclidean norm (`REF-MATHWORLD-VECTOR-NORM`)                                                                    |
| EN-005  | factor per edge `f = 1` if `S ≤ L`, else `L / S` (tested before dividing: `L = S = 0` on a zero-length edge), per vertex `g = min(fᵢ₋₁, fᵢ)` | `DERIV-local-radius-clamp`, principle of `REF-CSS-BR` §4.5                                                      |
| EN-005  | setback at a vertex                                                                                                                          | `DERIV-fillet-setback` (EN-004)                                                                                 |
| EN-006  | unit direction of an edge; zero-length edges (size 0, Q15)                                                                                   | `REF-MATHWORLD-UNIT-VECTOR`; zero vector handled explicitly                                                     |
| EN-006  | tangent points, arc center, turning direction of the arc                                                                                     | to derive: `DERIV-fillet-arc` (Euclid III.18, IV.4; rotation by a quarter turn `REF-MATHWORLD-ROTATION-MATRIX`) |
| EN-006  | segment of zero length after clamping (circle)                                                                                               | `EPSILON = 1e-9` (Q10, ADR-0003)                                                                                |
| EN-007  | `A rx ry x-axis-rotation large-arc-flag sweep-flag x y`, flag grammar                                                                        | `REF-SVG2-PATHS` §9.3.8, §9.3.9                                                                                 |
| EN-007  | meaning of the flags; zero radius; radii too small                                                                                           | `REF-SVG2-IMPLNOTE` B.2.1, B.2.5                                                                                |
| EN-007  | "positive-angle" direction on screen                                                                                                         | `REF-SVG2-COORDS` §8.4 (y down)                                                                                 |
| US-003  | requested vs effective radius, display                                                                                                       | `docs/domain/shapes.md` §2 (Q8, settled) — no open business point                                               |
| VAL-001 | replays the feature's criteria                                                                                                               | no new source: the criteria and the check tables above                                                          |

## Tasks

One task = one commit, referenced as `SP-001.Tn`.

- [x] T1 — Inventory the formulas and behaviors of EN-005 to VAL-001 (0.5 h)
- [x] T2 — Find, read and record the sources; write the derivations (3 h)
- [x] T3 — Write research note 0003 and update the stories (1 h)
