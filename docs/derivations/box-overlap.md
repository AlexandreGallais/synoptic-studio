# DERIV-box-overlap — Whether a contour's painted area touches an upright box, or lies within it

Used by: EN-144 (F37), step 10 since AUD-039, the shapes a selection rectangle takes (`interaction.md` §2). Sources: `REF-WIKI-INTERVAL` (a closed interval), `REF-WIKI-BOUNDING-BOX` (a box as the product of two intervals), `REF-SUNDAY-INCLUSION` (the winding number counted along a ray of any direction), `REF-WIKI-IVT`, `REF-WIKI-BEZIER` (a cubic under a linear map of its control points), `REF-MATHWORLD-CIRCLE`, `REF-WIKI-DE-CASTELJAU` (the point at a fraction of a segment), `REF-MATHWORLD-PERP-DOT`; `DERIV-painted-area`, `DERIV-arc-bounding-box`, `DERIV-cubic-sides`.

No single source tells whether a contour of segments, fillet arcs and cubic sides meets a rectangle: the sources count a ray's crossings, give a curve's box from its extremes, and find a curve's crossings with a line. This page joins them, beside `DERIV-painted-area`.

## Notation

Frame of the SVG: x to the right, y down.

| Symbol | Definition |
| --- | --- |
| `R` | the box asked, `[x₀, x₁] × [y₀, y₁]`, `x₀ ≤ x₁`, `y₀ ≤ y₁`, its sides included |
| `K` | its corner `(x₀, y₀)` |
| `wn(P)` | the contour's winding number about `P` (`DERIV-painted-area`, step 1) |
| `S, E` | the start and the end of a piece of the contour |
| `c, [a, b]` | a horizontal side of `R`: the ordinate `c`, its abscissas from `a` to `b` |
| `C, r, σ` | a fillet arc's centre and radius, `σ = +1` when it turns in the positive direction of angles (`positive`), `−1` otherwise |
| `u × v` | the perp dot product `uₓ v_y − u_y vₓ` (`REF-MATHWORLD-PERP-DOT`) |

## Step 1 — A point in a box

A closed interval `[a, b]` is `{x | a ≤ x ≤ b}`, both ends included (`REF-WIKI-INTERVAL`); an upright box is the product of an interval on each axis (`REF-WIKI-BOUNDING-BOX`). So `P` is in `R` when `x₀ ≤ pₓ ≤ x₁` and `y₀ ≤ p_y ≤ y₁`. A number between two others given in either order is in `[min(a, b), max(a, b)]`.

## Step 2 — Enclosing: the contour's box within the box

`R` encloses the contour when every point of every piece is in `R`. The contour's straight box is the smallest upright box around the outline as drawn, arcs and cubic sides included, from their ends and their extremes on each axis (`DERIV-arc-bounding-box` step 4; `DERIV-cubic-sides` step 3, the roots of the derivative from `cubicAxisTurns`). `R` is an upright box: when it holds the outline, it holds the smallest one around it; when it holds that box, it holds the outline in it. So

`R` encloses the contour ⇔ `x₀ ≤ minX`, `maxX ≤ x₁`, `y₀ ≤ minY` and `maxY ≤ y₁`, of the contour's box.

The box is computed: a fillet's axis point lands within a rounding error of the side it is tangent to, the left of a rounded rectangle 100 × 100 at (0, 0) at about `−3.6 × 10⁻¹⁵`. Each comparison is therefore made within `EPSILON` (ADR-0003), so that a rectangle drawn along a side, on integers, encloses the shape.

The painted area lies within the same box: from a point right of it, the ray toward growing x meets no piece, so `wn = 0`; above, below and left of it, a ray of another direction meets none, and the count is the same along a ray of any direction (`REF-SUNDAY-INCLUSION`: "Pick any point Q on S1", "take an infinite ray R starting at P and extending in the direction of the vector Q").

## Step 3 — Touching: a piece meets the box, or its corner is painted

`R` touches the painted area when they share a point, the outline counted with the area it bounds, as the SVG fills up to it. Then:

`R` touches ⇔ some piece meets `R`, or `wn(K) ≠ 0`.

If a piece meets `R`, they share a point of the outline. If `wn(K) ≠ 0`, `K` is painted. Conversely, let `Q` in `R` be painted, and no piece meet `R`. For `P` and `P′` on one horizontal line in `R`, `P` left of `P′`, the ray from `P` toward growing x is the segment `P P′` followed by the ray from `P′`; no piece crosses `P P′`, which lies in `R`, so both rays count the same crossings and `wn(P) = wn(P′)`. A vertical ray counts the same winding number (`REF-SUNDAY-INCLUSION`, step 2), so along a vertical line in `R` it does not change either. Going from `Q` to `(x₀, q_y)`, then to `K`, inside `R`, gives `wn(K) = wn(Q) ≠ 0`.

## Step 4 — A piece meets the box when its start is in it, or it reaches a side

Let a piece run from `S` outside `R` to a point `Q` in `R`, its points `γ(t)`, `t` from 0 to 1, continuous. The function `g(t) = max(x₀ − x(t), x(t) − x₁, y₀ − y(t), y(t) − y₁)` is continuous, positive outside `R`, at most 0 in it, and 0 exactly on its sides (step 1). `g` is positive at `S` and not at `Q`, so it takes the value 0 between them (`REF-WIKI-IVT`): the piece reaches a side. So

a piece meets `R` ⇔ `S` is in `R`, or the piece reaches one of `R`'s four sides.

The end `E` needs no test of its own: in `R` with `S` outside, the piece reaches a side on its way.

## Step 5 — A segment reaching a horizontal side

The segment's points are `S + t (E − S)`, `t` from 0 to 1 (`REF-WIKI-DE-CASTELJAU`, `pointBetween`). When `S_y ≠ E_y` and `c` is between them, it meets the line `y = c` at `t = (c − S_y) / (E_y − S_y)`, and reaches the side when the x there is between `a` and `b`. A segment with `S_y = E_y` lies on a horizontal line; it is left out, and step 8 shows that its start and the vertical sides tell.

## Step 6 — A cubic side reaching a horizontal side

A cubic side is cut at the roots of `y′` into parts monotone in y (`DERIV-painted-area` step 4, `cubicYBreaks`). On a part `[t₁, t₂]` with `y(t₁) ≠ y(t₂)`, `y` takes every value between its ends (`REF-WIKI-IVT`), and each once, being monotone: the part meets the line `y = c` when `c` is between `y(t₁)` and `y(t₂)`, at the `t*` the bisection finds (`cubicLevelParameter`), and reaches the side when `x(t*)` is between `a` and `b`. A part with `y(t₁) = y(t₂)` is constant in y, monotone; it is left out as a flat segment (step 8). The cubic reaches the side when one of its parts does.

## Step 7 — A fillet arc reaching a horizontal side

The arc lies on the circle of the points at distance `r` from `C` (`REF-MATHWORLD-CIRCLE`). On the line `y = c`, `(x − Cₓ)² + (c − C_y)² = r²` (`REF-MATHWORLD-DISTANCE`) gives, with `h = c − C_y`,

`x = Cₓ ± √(r² − h²)` when `|h| ≤ r`, none otherwise.

The arc sweeps less than half a turn (`DERIV-painted-area` step 3), and lies on the side of its chord away from the centre, the centre's side being `σ ((E − S) × (C − S)) > 0` (`DERIV-painted-area` step 3). A point `Q` of the circle is on the arc when

`σ ((E − S) × (Q − S)) ≤ 0`,

the line of the chord meeting the circle at `S` and `E` only. The arc reaches the side when one of the two points is on it with its x between `a` and `b`. An arc never lies along a line.

## Step 8 — The vertical sides, by swapping the axes

The map `(x, y) ↦ (y, x)` takes `R` to the box `[y₀, y₁] × [x₀, x₁]`, its vertical sides to horizontal ones, a segment to the segment of its swapped ends, a cubic to the cubic of its swapped control points (a linear map of the plane is applied to a Bézier curve by applying it to its control points, `REF-WIKI-BEZIER`), and a circle to the circle of the swapped centre, of the same radius. It reverses every turn: `(u_y, uₓ) × (v_y, vₓ) = u_y vₓ − uₓ v_y = −(u × v)`, so a swapped arc has `−σ`, which the side test of step 7 needs. A piece reaches a vertical side of `R` when the swapped piece reaches a horizontal side of the swapped box.

A piece along a horizontal line `y = c` with `y₀ ≤ c ≤ y₁`, left out in steps 5 and 6, meets `R` when its x range meets `[x₀, x₁]`: its start is then in `R`, or the piece goes from outside `[x₀, x₁]` into it and passes `x = x₀` or `x = x₁` at `y = c`, a point of a vertical side, where, swapped, it is not flat. The same holds swapped for a piece along a vertical line. So steps 4 to 8 together find every piece meeting `R`.

## Step 9 — A shape without area

A box of width or height 0 paints nothing (`DERIV-painted-area` step 6): its outline, the closed polygon its corners join, is taken instead. `R` touches it when one of its sides, as segments, meets `R` (step 4); it is enclosed when its corners' box is within `R` (step 2, the polygon's box being its corners').

## Step 10 — Touching within `EPSILON`

The contour is computed: a side of a Polygon-tool square, 4 corners of radius 10, 100 × 100 at `(0, 0)`, lands at `x ≈ 1.1 × 10⁻¹⁵`, the cosines of its corners' angles not exact, and the bottom of a stretched decagon of that box at `y ≈ 100 − 1.4 × 10⁻¹⁴`. Step 2 takes such a side as on `R` within `EPSILON`; touching, exact, would then leave out a shape that `R` encloses, `[0, 100]²` enclosing the square while `[−10, 0] × [40, 60]` along its left side misses it. So steps 3 to 9 are asked of `R` widened by `EPSILON` on every side,

`R_ε = [x₀ − ε, x₁ + ε] × [y₀ − ε, y₁ + ε]`, `ε = EPSILON` (ADR-0003),

the product of its intervals each widened at both ends (`REF-WIKI-BOUNDING-BOX`, `widenedBox`): a side of the contour within `EPSILON` of a side of `R` touches it, as it is enclosed by it, and one beyond by more does not. A contour's rounding errors lie far below `EPSILON` (ADR-0003) and `R` is drawn on integers, so no shape a whole unit away is taken.

## Cross-check

For an upright rectangle as the contour, `[p₀, p₁] × [q₀, q₁]`, the boxes share a point when their intervals overlap on both axes, `p₀ ≤ x₁`, `x₀ ≤ p₁`, `q₀ ≤ y₁` and `y₀ ≤ q₁`, the intersection of two products being the product of the intersections (`REF-WIKI-BOUNDING-BOX`, `REF-WIKI-INTERVAL`). Steps 3 to 8 give the same answer, which the tests' properties draw on integers.

## Checks (test cases)

The arch of `DERIV-painted-area`: `(0, 0)`, `(0, −40)`, `(100, −40)`, `(100, 0)`, `y = −120 t (1 − t)`, `x = 300 t² − 200 t³`, its breaks 0, ½, 1. The quarter arc: from `(0, 50)` to `(50, 0)` about `C = (50, 50)`, `r = 50`, `σ = +1`.

| Case | Computation | Expected |
| --- | --- | --- |
| `(5, 5)` in `[0, 10] × [0, 10]`; `(10, 0)`; `(11, 5)` | `0 ≤ 5 ≤ 10` twice; ends included; `11 > 10` | in, in, out |
| box `[0, 100] × [−30, 0]` within `[−1, 101] × [−31, 1]`; within `[0, 100] × [−29, 0]` | `−1 ≤ 0`, `100 ≤ 101`, `−31 ≤ −30`, `0 ≤ 1`; `−29 > −30` | yes; no |
| box `[−3.6 × 10⁻¹⁵, 100]²` within `[0, 100]²`; `[−10⁻⁶, 100]²` | `0 − 10⁻⁹ ≤ −3.6 × 10⁻¹⁵`; `−10⁻⁶ < −10⁻⁹` | yes; no |
| segment `(0, 0)` to `(100, 100)`, side `y = 50`, `[40, 60]`; `[60, 80]` | `t = 50 / 100 = ½`, x = 50 | reaches; not |
| the same segment, side `y = 100`, `[90, 110]`; side `y = 150` | `t = 1`, x = 100; 150 beyond 100 | reaches; not |
| segment `(0, 0)` to `(100, 0)`, side `y = 0`, `[0, 100]` | flat: left to its start and the vertical sides | not |
| arch, side `y = −15`, `[0, 10]`, `[10, 90]`, `[90, 100]` | `t (1 − t) = ⅛`, `x ≈ 5.80583`, `94.19417` | reaches, not, reaches |
| arch, side `y = −30`, `[40, 60]`; side `y = −35` | the double root `t = ½`: x = `75 − 25 = 50`; −35 above the top | reaches; not |
| arch, part `[0, ½]` and `[½, 1]`, side `y = −15`, `[0, 10]` | x ≈ 5.80583 on the first part, 94.19417 on the second | reaches; not |
| circle about `(50, 50)`, `r = 50`, level 25; level −10 | `h = −25`, `√(2500 − 625) = √1875 ≈ 43.30127`: x ≈ 6.69873, 93.30127; `\|h\| = 60 > 50` | two; none |
| quarter arc, `Q = (6.69873, 25)` and `(93.30127, 25)` | `E − S = (50, −50)`; `50 · (−25) − (−50) · 6.69873 ≈ −915.06 ≤ 0`; `−1250 + 4665.06 = 3415.06 > 0` | on; off |
| quarter arc, side `y = 25`, `[0, 10]`; `[90, 100]`; side `y = 60`, `[0, 10]` | x ≈ 6.69873 on it; 93.30127 off it; `h = 10`, x ≈ 1.01021: `50 · 10 + 50 · 1.01021 > 0`, off | reaches; not; not |
| quarter arc swapped | from `(50, 0)` to `(0, 50)` about `(50, 50)`, from the top to the left: `σ = −1` | negative |
| arch and box `[45, 55] × [−35, −25]` | the top `(50, −30)` is in the box, the ends outside | meets |
| arch and box `[0, 10] × [−30, −20]`, the corner of its box | on `y = −20`, `t (1 − t) = ⅙`, `t ≈ 0.21132`, x ≈ 11.51 > 10; on `x = 10`, `t ≈ 0.1955`, y ≈ −18.87, below −20 | does not meet |
| the arch closed by `(100, 0)` to `(0, 0)`, box `[40, 60] × [−20, −10]` | no piece meets it (above it the arch, below it y = 0); `K = (40, −20)` under the arch, `wn = +1` | touches |
| the same contour, box `[0, 10] × [−30, −20]` | no piece meets it; `K = (0, −30)` above the arch, `wn = 0` | does not touch |
| square `[0, 100]²`, box `[100, 200] × [50, 60]` | the square's side from `(100, 0)` to `(100, 100)` reaches `y = 50` at `t = ½`, x = 100, between 100 and 200: the sides share a point | touches |
| `[0, 100] × [−30, 0]` widened by 2 | `[0 − 2, 100 + 2] × [−30 − 2, 0 + 2]` | `[−2, 102] × [−32, 2]` |
| the Polygon-tool square of radius 10, box `[−10, 0] × [40, 60]`; `[−10, −1] × [40, 60]` | its left side at `x ≈ 1.1 × 10⁻¹⁵ ≤ 0 + 10⁻⁹`; `−1 + 10⁻⁹ < 1.1 × 10⁻¹⁵` | touches; does not touch |
| the stretched decagon of the box `[0, 100]²`, box `[0, 100] × [100, 110]` | its bottom side at `y ≈ 100 − 1.4 × 10⁻¹⁴ ≥ 100 − 10⁻⁹` | touches |
