# 0014 — Sources of F12: a corner fillet beside a curved side

Spike SP-012, 2026-10-13, after the Product Owner asked at VAL-011 that a rounded corner stay rounded when a neighbouring point gets handles. Sources in [references](../../references.md#curves-and-handles); formulas in `DERIV-fillet-beside-curve`; decision in ADR-0047.

## What tools do

| Tool                                 | Beside a curved segment                                                                                                                       | Source                     |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| Astute Graphics, Illustrator plug-in | rounds points next to curves "without distorting them"; a radius that does not fit is "made as large as possible", never past the next anchor | `REF-ASTUTE-CORNERS`       |
| Inkscape, fillet and chamfer         | needs "a numerical solution"; when several, the one closest to the corner along the curve                                                     | `REF-INKSCAPE-FILLET-SPEC` |
| Figma                                | a radius is clamped to half a straight edge; nothing found on curved edges                                                                    | `REF-FIGMA-CR`             |

## How it is built

- The fillet's centre is at distance `r` from both sides: on the straight side's parallel line and on the curve's parallel curve (`REF-WIKI-PARALLEL-CURVE`). The parallel of a cubic is no cubic (`REF-WIKI-BEZIER`, offsets), so the meeting point is found numerically.
- Along the curve, the radius a circle reaches at parameter `t` has a closed form, `r(t)`; it grows while the curve moves away from the other side and bends less than the circle (`REF-WIKI-CURVATURE`). Candidates are found in a coarse table along the curve then by bisection (`DERIV-fillet-beside-curve` step 7), the first whose circle crosses neither side drawn.
- The cubic is cut there by de Casteljau's split (`REF-WIKI-DE-CASTELJAU`): the piece kept lies exactly on the curve.
- A straight side written as a cubic gives the same fillet as a segment (tangent half-angle, `REF-WIKI-TAN-HALF-ANGLE`).

Running example, the Product Owner's: a rectangle 120 × 80, radius 20, point 3 in handles mode with a handle of length 1 along the diagonal. Points 2 and 4 keep their radius 20; corner 2's centre is at (100.15277, 20), its arc touching the curve at (120.15232, 19.86518).

## A radius too large (decided 2026-10-13)

A cubic's other end always has its handles on, so it is sharp: only one fillet ever uses a curve.

| Question                                     | Options                                                                                                                                                                   | Decision                                                                             |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| how far along the curve                      | the whole curve (rectangle above with a handle of 40, radius 60 at point 2 → 48.6); half of it (→ 37.2, and a straight cubic would reduce a radius 20 that fits to 18.75) | the whole curve, as a single rounded corner may use a whole straight side (ADR-0007) |
| a curve bending more tightly than the radius | the largest circle still tangent (handle 80, radius 60 → 36.3); a sharp corner                                                                                            | the largest circle, as Astute Graphics                                               |

As built (US-027), the circle is kept clear of the whole curve, not only up to where it touches: the examples asked become 47.7 instead of 48.6 (the larger circle crossed the curve further on) and 46.9 instead of 36.3 (a larger circle touches the curve at point 3 without crossing it). The rule chosen is unchanged — the whole curve, the largest circle, never sharp — but the numbers differ from those the Product Owner chose from: shown at VAL-011.

## Impact

- Geometry: a general `splitCubic`, the tangent at a corner whose handle is off, the fillet between a side and a cubic found along the cubic's parallel curve (folders `fillet-side`, `curve-fillet`, `curve-contour`).
- Domain: `shapes.md` "Points with handles"; ADR-0047 replaces ADR-0046's sharp corner.
