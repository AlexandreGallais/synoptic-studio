# DERIV-round-shape — A circle or a capsule as a rectangle rounded by half its smaller side

Used by: ADR-0036, F10. Sources: `DERIV-local-radius-clamp` (setback and factor per edge), `DERIV-fillet-setback` (`s = r · tan(|τ|/2)`), `REF-MATHWORLD-STADIUM` (a rectangle capped by two half circles), `REF-SVG2-SHAPES` §10.2 (as a contrast: SVG caps a `rect`'s `rx` at half its width and `ry` at half its height separately, which gives elliptical corners, not a capsule).

## Notation

| Symbol | Definition                                              |
| ------ | ------------------------------------------------------- |
| `w, h` | width and height of the round shape's box, integers ≥ 0 |
| `m`    | the smaller side, `min(w, h)`                           |
| `r`    | the radius given to each of the four corners            |

## Step 1 — Every corner rounded by half the smaller side

The box is a rectangle: four right angles, `|τ| = π/2`, so each setback is `s = r · tan(π/4) = r` (`DERIV-fillet-setback`). With `r = m/2`:

- on a side of length `m`, the two setbacks add up to `m`: the factor of that side is 1 (`DERIV-local-radius-clamp` step 2), the two arcs meet in its middle and the side is a half circle;
- on a side of length `M = max(w, h) ≥ m`, they add up to `m ≤ M`: the factor is 1 too, and a straight part of length `M − m` remains.

No radius is reduced: the drawing is exactly the rectangle capped by two half circles of radius `m/2`, a stadium (`REF-MATHWORLD-STADIUM`); when `w = h` the straight parts have no length and the four quarter arcs make the circle of diameter `w` (as F01: a square of side `c` with radius `c/2`). In floating point `tan(π/4)` is `0.9999999999999999`: the arcs stop about `1e-16 · m` short of the middle (`1.4e-14` for `m = 100`), a piece without length dropped from the contour while it stays below `EPSILON` (`hasLength`), that is for sides up to a few million units, as for any rectangle at its maximal radius (Q20).

## Step 2 — Converted to a free shape

A free shape stores integer radii (ADR-0029, ADR-0033). The conversion is the rectangle tool's free shape for the box (the unit square stretched to `w × h`, ADR-0032, valid at a size of 0), each of its 4 corners getting `⌈m/2⌉`. For an even `m` it is `m/2`, step 1. For an odd `m` it is `(m + 1)/2`: on a side of length `m` the setbacks add up to `m + 1 > m`, the factor is `m/(m + 1)`, and the effective radius is `(m + 1)/2 · m/(m + 1) = m/2` (`DERIV-local-radius-clamp` step 3): the same drawing. On the longer side, `m + 1 ≤ M` unless `w = h`, where both sides give the same factor.

## Checks (test cases)

| Case               | Computation                               | Expected                                                             |
| ------------------ | ----------------------------------------- | -------------------------------------------------------------------- |
| 100 × 100          | `r = 50`                                  | the circle of diameter 100: four quarter arcs of radius 50           |
| 120 × 80           | `r = 40`; top side 120: straight part 40  | a capsule: half circles of radius 40, straight sides from x 40 to 80 |
| 80 × 120           | `r = 40`; left side 120: straight part 40 | a standing capsule                                                   |
| 81 × 81 converted  | radii 41; factor 81/82                    | effective radius 40.5: the same circle                               |
| 120 × 80 converted | radii 40                                  | the same capsule                                                     |
| 0 × 50             | `r = 0`                                   | a vertical line, as a rectangle of width 0 (Q15)                     |
