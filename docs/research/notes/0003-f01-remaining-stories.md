# Note 0003 — Sources of the remaining F01 stories

**Date**: 2026-10-08 — research spike SP-001, done by Claude Code (WebFetch), during the autonomous run of F01.
**Resulting documents**: `DERIV-fillet-arc`; references verified or added; EN-005, EN-006, EN-007 and US-003 updated.

## Per story

| Story  | Found                                                                                                                                                                                              | Source                                                                              |
| ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| EN-005 | CSS reduces every radius of a box by one factor `f = min(Lᵢ/Sᵢ)`; ADR-0007 keeps the factor but applies it per edge (`f = 1` if `S ≤ L`, else `L / S`, which never divides by zero) and per vertex | `REF-CSS-BR` §4.5 (quoted), `DERIV-local-radius-clamp`                              |
| EN-005 | edge length is the L2-norm                                                                                                                                                                         | `REF-MATHWORLD-VECTOR-NORM`                                                         |
| EN-006 | unit vector `v / ‖v‖`, defined for a nonzero vector only: the zero vector (size 0) needs an explicit rule                                                                                          | `REF-MATHWORLD-UNIT-VECTOR`                                                         |
| EN-006 | quarter turn `(x, y) ↦ (−y, x)` from the rotation matrix with θ = π/2                                                                                                                              | `REF-MATHWORLD-ROTATION-MATRIX` (MathWorld's perp-dot page gives no component form) |
| EN-006 | tangent points, center, angle and direction of the fillet                                                                                                                                          | derived: `DERIV-fillet-arc` (Euclid III.18, IV.4, I.32)                             |
| EN-007 | `A rx ry x-axis-rotation large-arc-flag sweep-flag x y`; flags are single digits `0` or `1`                                                                                                        | `REF-SVG2-PATHS` §9.3.8, §9.3.9                                                     |
| EN-007 | `fA = 1` only above 180°; `fS = 1` for increasing angles; zero radius drawn as a line                                                                                                              | `REF-SVG2-IMPLNOTE` B.2.1, B.2.5 (now verified)                                     |
| EN-007 | y points down, so increasing angles turn clockwise on screen                                                                                                                                       | `REF-SVG2-COORDS` §8.4                                                              |
| US-003 | requested radius stored, effective radius derived and signaled when different                                                                                                                      | `docs/domain/shapes.md` §2 (Q8)                                                     |

## Remaining open

- Display for a zero-size rectangle (no corner to round): question for the Product Owner, recorded in US-003; it blocks nothing.
- No source needed beyond these for F01; `REF-GG-FILLET` stays unverified and uncited.

## For the audit (AUD-001)

- `cyclicVertex` relied on JavaScript `%` and `.at` with negative indexes (ADR-0025): done in EN-005 — generalized into `cyclicItem` with a non-negative modulo stated in its TSDoc.
- The center of the fillet is computed but not needed by the path data; keep it tested for later uses.
