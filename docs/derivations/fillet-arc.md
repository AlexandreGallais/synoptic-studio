# DERIV-fillet-arc — Tangent points, center and SVG flags of a corner fillet

Used by: EN-006 (geometry), EN-007 (path data). Builds on `DERIV-fillet-setback` and `DERIV-turning-angle`.

## Notation

Screen frame of SVG: x to the right, **y down** (`REF-SVG2-COORDS` §8.4).

| Symbol          | Definition                                                                                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------ |
| `P`, `V`, `N`   | previous, current and next vertices of the contour                                                           |
| `u_in`, `u_out` | unit directions of the incoming edge `V − P` and the outgoing edge `N − V`                                   |
| `τ`             | turning angle at `V` (`DERIV-turning-angle`), in ]−π, π[; `τ > 0` when the contour turns clockwise on screen |
| `r`             | effective radius at `V` (after `DERIV-local-radius-clamp`), `r ≥ 0`                                          |
| `s`             | setback, `s = r · tan(\|τ\| / 2)` (`DERIV-fillet-setback`)                                                   |
| `w⊥`            | `w` rotated by a quarter turn: `(x, y)⊥ = (−y, x)`                                                           |

## Step 1 — Unit directions

`u = w / ‖w‖` with `‖w‖ = √(x² + y²)` (`REF-MATHWORLD-UNIT-VECTOR`, `REF-MATHWORLD-VECTOR-NORM`). A zero-length edge (a size of 0, Q15) has no direction: its unit vector is taken as `(0, 0)`. Its turning angle is then taken as 0 explicitly (`DERIV-turning-angle`: not left to `atan2`, which gives π for `atan2(+0, −0)`), so `s = 0` and the vertex keeps no arc.

## Step 2 — Tangent points

The fillet touches each edge at distance `s` from `V` (`DERIV-fillet-setback`, steps 1–4: the two tangent lengths are equal); a point at distance `s` along a unit direction is `V + s · u` (scalar multiplication and addition in components, `REF-OPENSTAX-CALC3-VECTORS`):

- on the incoming edge: `T_in = V − s · u_in`;
- on the outgoing edge: `T_out = V + s · u_out`.

## Step 3 — Center

The radius to the point of contact is perpendicular to the tangent (Euclid III.18, `REF-EUCLID-III18`): the center `C` lies on the perpendicular to the incoming edge through `T_in`, at distance `r`. The quarter turn of a vector is `(x, y) ↦ (−y, x)`, the rotation matrix with angle π/2 (`REF-MATHWORLD-ROTATION-MATRIX`).

The center is on the side of the angle's bisector (Euclid IV.4, `REF-EUCLID-IV4`), the side towards which the contour turns. Since `perpDot(u_in, u_out) = u_in⊥ · u_out = sin τ` (`REF-MATHWORLD-PERP-DOT`), the outgoing edge leaves on the `u_in⊥` side when `τ > 0` and on the other side when `τ < 0`:

`C = T_in + sign(τ) · r · u_in⊥`

## Step 4 — Angle of the arc and large-arc flag

In the quadrilateral `V T_in C T_out`, the angles at `T_in` and `T_out` are right (step 3) and the angle at `V` is `θ = π − |τ|`. The angles of a quadrilateral sum to four right angles (two triangles, Euclid I.32, `REF-EUCLID-I32`), so the angle at `C` is `2π − π − θ = |τ|`. As `|τ| < π`, the fillet is the smaller arc: **large-arc-flag = 0** (`REF-SVG2-IMPLNOTE` B.2.1: `fA = 1` only for an arc of more than 180°).

## Step 5 — Direction of the arc and sweep flag

Along the arc from `T_in` to `T_out`, the tangent turns from `u_in` to `u_out`, by `τ`. The radius stays perpendicular to the tangent (Euclid III.18), so it turns by the same signed angle: the angle around `C` increases when `τ > 0`. SVG draws the arc "in a positive-angle direction" when the sweep flag is 1 (`REF-SVG2-PATHS` §9.3.8, `REF-SVG2-IMPLNOTE` B.2.1). Hence **sweep-flag = 1 when `τ > 0`, 0 when `τ < 0`**. With y down, a positive angle turns clockwise on screen: the convex corners of a clockwise contour (Q11) have `τ > 0` and sweep 1.

## Step 6 — Radius 0

`r = 0` gives `s = 0`, `T_in = T_out = V`: no arc. SVG would also draw an arc of zero radius as a straight line (`REF-SVG2-IMPLNOTE` B.2.5, step 1); the library never writes one.

## Step 7 — Starting point of a rounded contour

A contour starts at its top-left vertex (Q11). Once rounded, that vertex is no longer on the path; the path starts at `T_out` of the first vertex, so that its commands follow the vertices in order — edge to the second vertex first — and the first vertex's arc closes the path. This is a convention of the output, chosen for this order; starting at `T_in` would draw the same shape.

Pieces without length are dropped: a segment between two tangent points that meet (two fillets covering a whole edge), or the arc of a corner that keeps no fillet (`T_in = T_out`). A piece is kept when the distance between its ends is at least `EPSILON` (Q10). Consecutive pieces therefore meet within `EPSILON`, not exactly.

When the contour is written as path data, a final segment is left to `closepath`, which draws a straight line back to the start (`REF-SVG2-PATHS` §9.3.4): with every radius 0 and edges of positive length, the result is exactly the sharp path data `M p₀ L p₁ … Z`. Zero-length edges (a size of 0, Q15) are dropped, so a 0 × 50 rectangle is written `M0 0 L0 50 Z`: the same drawing with fewer commands.

## Checks (test cases)

Rectangle 100 × 50 from the top-left vertex, clockwise: `(0, 0)`, `(100, 0)`, `(100, 50)`, `(0, 50)`, effective radius 10 everywhere. Every corner has `τ = π/2`, so `s = 10 · tan(π/4) = 10`.

| Vertex      | `u_in`    | `u_out`   | `T_in`      | `T_out`     | `u_in⊥`   | `C`        | flags |
| ----------- | --------- | --------- | ----------- | ----------- | --------- | ---------- | ----- |
| `(0, 0)`    | `(0, −1)` | `(1, 0)`  | `(0, 10)`   | `(10, 0)`   | `(1, 0)`  | `(10, 10)` | `0 1` |
| `(100, 0)`  | `(1, 0)`  | `(0, 1)`  | `(90, 0)`   | `(100, 10)` | `(0, 1)`  | `(90, 10)` | `0 1` |
| `(100, 50)` | `(0, 1)`  | `(−1, 0)` | `(100, 40)` | `(90, 50)`  | `(−1, 0)` | `(90, 40)` | `0 1` |
| `(0, 50)`   | `(−1, 0)` | `(0, −1)` | `(10, 50)`  | `(0, 40)`   | `(0, −1)` | `(10, 40)` | `0 1` |

Other cases:

| Case                                                                       | Computation                                         | Expected                                                                                               |
| -------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Concave corner: `u_in = (1, 0)`, `u_out = (0, −1)`, `V = (10, 0)`, `r = 5` | `perpDot = 1·(−1) − 0·0 = −1` → `τ = −π/2`, `s = 5` | `T_in = (5, 0)`, `T_out = (10, −5)`, `C = (5, 0) − 5·(0, 1) = (5, −5)`, flags `0 0`                    |
| Aligned vertex, `r = 30`                                                   | `τ = 0` → `s = 0`                                   | `T_in = T_out = V`, no arc                                                                             |
| Square 100 × 100, `r = 50`                                                 | `s = 50` on every corner                            | arcs meet: `T_out` of `(100, 0)` = `(100, 50)` = `T_in` of `(100, 100)`; no segment of positive length |
| Zero-length edge (rectangle 0 × 50)                                        | unit vector `(0, 0)`, `τ = 0`                       | `s = 0`, no arc, no `NaN`                                                                              |

Each center is at distance `r` from both tangent points, e.g. `‖(90, 10) − (100, 10)‖ = 10`.

## Limits

- The turning angle must be in ]−π, π[. At a vertex where the contour turns back on itself (`τ = ±π`, a spike), `tan(|τ|/2)` is huge: the clamp then reduces the radius to almost 0 while the setback covers the whole edge, so the spike is consumed. This is the intended behavior (Q16, settled by the Product Owner); the sign of `τ` there also depends on the sign of a zero.
- At `τ = 0`, `sign(τ) = 0` gives `C = V`; there is no arc, so no center is used.
- The angle between `C − T_in` and `C − T_out` equals `|τ|`; the library never needs the center to write the path (only radius, flags and end point), but keeps it for later uses (hit-testing, snapping).
