# DERIV-painted-area — Whether a point is in a contour's painted area

Used by: EN-143 (F36), the shape under the pointer (`interaction.md` §2). Sources: `REF-SVG2-FILL-RULE` (the nonzero rule), `REF-SUNDAY-INCLUSION` and `REF-BOURKE-POINT-IN-POLYGON` (a horizontal ray, the half-open test of an edge's ends, the winding number of any closed curve), `REF-MATHWORLD-PERP-DOT`, `REF-POMAX-BEZIER` (a curve's crossings with a line as the roots of its y), `REF-WIKI-QUADRATIC`, `REF-MATHWORLD-BISECTION`, `REF-SUNDAY-SEGMENT-DISTANCE` and `REF-BOURKE-POINT-LINE` (a point's distance to a segment); `DERIV-fillet-arc`, `DERIV-cubic-sides`.

No single source counts a ray's crossings with a contour of segments, fillet arcs and cubic sides: the sources count them on a polygon's edges, and find a curve's crossings with a line as roots. This page joins them.

## Notation

Frame of the SVG: x to the right, y down.

| Symbol | Definition |
| --- | --- |
| `P = (pₓ, p_y)` | the point asked |
| `S, E` | the start and the end of a piece of the contour |
| `u × v` | the perp dot product `uₓ v_y − u_y vₓ` (`REF-MATHWORLD-PERP-DOT`) |
| `d` | a piece's direction across the ray: `+1` when its y grows, `−1` when it falls |
| `C, r, σ` | a fillet arc's centre and radius, `σ = +1` when it turns in the positive direction of angles (`positive`), `−1` otherwise |
| `y(t), x(t)` | a cubic side's coordinates at `t` from 0 to 1 |

## Step 1 — The nonzero rule

A point is painted when the signed count of the contour's crossings with a ray from it is not zero: "add one each time a path segment crosses the ray from left to right and subtract one each time a path segment crosses the ray from right to left" (`REF-SVG2-FILL-RULE`). The ray is taken horizontal, from `P` toward growing x, as both `REF-SUNDAY-INCLUSION` and `REF-BOURKE-POINT-IN-POLYGON` take it; a crossing counts `+1` when the curve crosses it with y growing, `−1` with y falling (`REF-SUNDAY-INCLUSION`: "from below to above, the crossing is positive (+1)", below being the smaller y). The count of a closed curve is its winding number about `P`, whatever the curve (`REF-SUNDAY-INCLUSION`), and it is the sum of its pieces' counts. With y down the picture is mirrored, every count changes sign together, and the rule only asks whether the sum is 0: the frame does not matter.

## Step 2 — A segment

The edge crossing rules (`REF-SUNDAY-INCLUSION`): "an upward edge includes its starting endpoint, and excludes its final endpoint; a downward edge excludes its starting endpoint, and includes its final endpoint; horizontal edges are excluded". On both directions the segment meets the ray's line when

`min(S_y, E_y) ≤ p_y < max(S_y, E_y)`,

the test Franklin's `pnpoly` writes (`REF-BOURKE-POINT-IN-POLYGON`). A vertex on the ray's line then belongs to exactly one of its two edges when the contour goes on across the line, and to none or both when it turns back, so it is counted once or not at all: two segments meeting at `(0, 50)`, one from `(0, 0)` and one on to `(0, 100)`, count the vertex in the second only; `(0, 0)` to `(0, 50)` and back up to `(10, 0)` count it in neither.

The crossing must be "strictly right of the point P": an edge going up crosses there when `P` is strictly on its left, `(E − S) × (P − S) > 0`, one going down when strictly on its right, `< 0` (`REF-SUNDAY-INCLUSION`, `isLeft`). So, with `d = sign(E_y − S_y)`:

`count(S, E) = d` when the segment meets the ray's line and `sign((E − S) × (P − S)) = d`; otherwise 0.

The rule is the same read backwards: `count(E, S) = −count(S, E)`, the range of y and the side test both symmetric.

## Step 3 — A fillet arc, by its chord

A fillet sweeps less than half a turn (`DERIV-fillet-arc`, step 4). Going along the arc from `S` to `E`, then back along its chord from `E` to `S`, is a closed curve, so by step 1 and step 2's symmetry

`count(arc) = count(S, E) + count(loop)`.

The loop bounds the circular segment between the arc and its chord, a convex region, travelled once in the arc's direction: a closed curve winds once about a point inside it and not at all about a point outside (`REF-SUNDAY-INCLUSION`: the winding number counts how many times the curve wraps around `P`, positive in the direction of growing angles), so `count(loop) = σ` inside and 0 outside.

The circular segment is the part of the disc on the far side of the chord from the centre. With `s = S − C`, `e = E − C`, `s × e = r² sin(σθ)` for an arc of angle `θ` in direction `σ` (`REF-MATHWORLD-PERP-DOT`), so `σ (s × e) > 0` as `0 < θ < π`; and `(E − S) × (C − S) = (e − s) × (−s) = s × e`. The centre is therefore on the side where `σ ((E − S) × (C − S)) > 0`, and `P` is inside the circular segment when

`|P − C| < r` and `σ ((E − S) × (P − S)) < 0`,

and `count(arc) = count(S, E) + σ` then, `count(S, E)` otherwise. No crossing with the circle is computed, and the chord's ends are the arc's, so step 2's rule of the ends holds where the arc meets its neighbours.

## Step 4 — A cubic side, by its parts monotone in y

A cubic side crosses the line `y = p_y` at the roots of `y(t) − p_y` (`REF-POMAX-BEZIER`: a curve's crossings with a line are roots once the line is the x-axis, here a horizontal line moved by `p_y`). Between the ends and the roots of `y′` with `0 < t < 1` (`DERIV-cubic-sides`, steps 2 and 3, the roots taken without cancellation, `REF-WIKI-QUADRATIC`), `y′` keeps one sign, so `y` is monotone on each part `[a, b]`: the part crosses the line at most once, as a segment does.

Each part is counted as step 2 counts a segment, from its ends `y(a)` and `y(b)`: it crosses when `min(y(a), y(b)) ≤ p_y < max(y(a), y(b))`, the parts joining end to end at the same computed points, so the rule of the ends holds between them as between edges; a part whose ends have the same y, a flat side or the part between a double root, never crosses. A part that crosses has `y(t) − p_y` of opposite signs at its ends, which brackets the root (`REF-MATHWORLD-BISECTION`): the bisection finds its `t*`, and the part counts `d = sign(y(b) − y(a))` when `x(t*) > pₓ`, 0 otherwise.

## Step 5 — A contour

The count of a contour is the sum of its pieces' counts (step 1), and `P` is painted when it is not 0 (`REF-SVG2-FILL-RULE`). The contour's pieces meet exactly where a segment or a cubic takes an arc's ends, and within `EPSILON` where a piece without length was dropped (`DERIV-fillet-arc`, step 7): a ray passing in that gap, less than `EPSILON` tall, may count one crossing too many or too few, far below a pixel. A point on the contour itself is left to the rounding of the side test, as the sources leave it.

## Step 6 — An outline without area

A box of width or height 0 paints nothing (step 5 gives 0 everywhere). Its outline is measured instead: the distance from `P` to the segment `S E` is `|P − S|` when `w · v ≤ 0`, `|P − E|` when `v · v ≤ w · v`, and `|P − (S + b v)|` otherwise, with `v = E − S`, `w = P − S`, `b = (w · v) / (v · v)` (`REF-SUNDAY-SEGMENT-DISTANCE`; `REF-BOURKE-POINT-LINE`, the same `u` tested between 0 and 1). A segment of length 0 has `v · v = 0 = w · v`, and the first test gives `|P − S|`. The distance to the closed outline its corners join is the smallest over its sides.

## Checks (test cases)

| Case | Computation | Expected |
| --- | --- | --- |
| segment `(100, 0)` to `(100, 100)`, `P = (50, 50)` | going down the screen, `d = +1`; `(0, 100) × (−50, 50) = 0 · 50 − 100 · (−50) = 5000 > 0` | `+1`; read backwards `−1` |
| the same segment, `P = (150, 50)` | `(0, 100) × (50, 50) = −5000`, not of the sign of `d` | 0 |
| the same segment, `P = (50, 0)` and `P = (50, 100)` | `0 ≤ 0 < 100` holds; `0 ≤ 100 < 100` fails | `+1` and 0 |
| segment `(0, 0)` to `(100, 100)`, `P = (40, 50)` and `(60, 50)` | crossing at x = 50: right of 40, left of 60 | `+1` and 0 |
| fillet of the top-left corner of 100 × 100, radius 20: `S = (0, 20)`, `E = (20, 0)`, `C = (20, 20)`, `σ = +1` |  |  |
| — `P = (8, 8)` | chord: `d = −1`, `(20, −20) × (8, −12) = −240 + 160 = −80 < 0`, count `−1`; `\|P − C\| = 12√2 ≈ 16.97 < 20` and `−80 < 0`: inside | `−1 + 1 = 0`: the ray meets the arc at x = 4 only, left of 8 |
| — `P = (2, 8)` | chord: `(20, −20) × (2, −12) = −200`, count `−1`; `\|P − C\| = √468 ≈ 21.63 > 20` | `−1`: the arc crosses at x = 4, right of 2, y falling |
| — `P = (15, 15)` | inside the disc, `(20, −20) × (15, −5) = 200 > 0`: the centre's side | chord only |
| arch `(0, 0)`, `(0, −40)`, `(100, −40)`, `(100, 0)`: `y = −120 t (1 − t)`, `x = 300 t² − 200 t³` | `y′ = 240 t − 120`, root ½: parts `[0, ½]` (y from 0 to −30) and `[½, 1]` (−30 to 0) | breaks 0, ½, 1 |
| — `P = (50, −15)` | `t (1 − t) = ⅛`, `t = (1 ± √½) / 2 ≈ 0.14645, 0.85355`; `x ≈ 5.80583, 94.19417` | first part left of 50, second `+1`: `+1` |
| — `P = (0, −15)` and `(50, −35)` | both crossings right of 0: `−1 + 1`; −35 above the arch | 0 and 0 |
| arch closed by `(100, 0)` to `(0, 0)` | the segment is flat | `(50, −15)` painted, `(50, 5)` not |
| segment `(0, 0)` to `(100, 0)` | `P = (50, 3)`: `b = ½`; `(−3, 4)`: `w · v = −300 ≤ 0`; `(103, 4)`: `v · v = 10 000 ≤ 10 300` | 3, 5, 5 |
| segment `(5, 5)` to `(5, 5)`, `P = (8, 9)` | `w · v = 0 ≤ 0` | 5 |
