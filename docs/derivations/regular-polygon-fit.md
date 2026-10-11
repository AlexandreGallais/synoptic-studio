# DERIV-regular-polygon-fit — Regular polygon fitted uniformly in a box

Used by: `docs/domain/shapes.md` §3 (Q1: uniform mode), EN-008, US-005, US-006, EN-009, US-009. Sources read in SP-002 (note 0004) and SP-003 (note 0005).

## Inputs (model, integers)

`n ≥ 3` corners, width `w ≥ 0`, height `h ≥ 0`; a horizontal and a vertical alignment, each `min`, `mid` or `max` (F07, step 5).

## Step 1 — Unit polygon

The nth roots of unity `e^(2πik/n)` form a regular polygon with n sides, every vertex on the unit circle (`REF-MATHWORLD-ROOT-OF-UNITY`). By Euler's formula `e^(ix) = cos x + i sin x` (`REF-MATHWORLD-EULER-FORMULA`), the vertex at central angle `α` is `(cos α, sin α)` (`REF-OPENSTAX-UNIT-CIRCLE`), in the mathematical frame (y upwards). Turning every vertex by the same angle `α₀` keeps them on the unit circle, `2π/n` apart:

`pₖ = (cos αₖ, sin αₖ)`, `αₖ = α₀ + 2πk / n`.

Two consecutive vertices are a chord `a = 2 sin(π/n)` apart (`REF-MATHWORLD-REGULAR-POLYGON` (2), `R = 1`).

Default orientation: **flat base** (a horizontal bottom edge). The bottom edge joins the angles `−π/2 ± π/n`: symmetric about `−π/2`, they have the same sine, hence the same y. So `α₀ = −π/2 + π/n`, and

`αₖ = −π/2 + (2k + 1) π / n`.

Consequence: `n=3` triangle pointing up, `n=4` square (not a diamond), `n=6` hexagon with flat bases.

## Step 2 — Order and starting vertex (Q11, Q18)

Increasing angles turn counter-clockwise in the mathematical frame; the picture keeps its orientation when drawn in the SVG frame (step 5), so the vertices are listed by **decreasing** angle to run clockwise on screen.

The topmost vertex has the largest sine: its angle is the closest to `π/2`. With `(2k + 1) π / n − π/2 = π/2`, i.e. `2k + 1 = n`:

- `n` odd: `k = (n − 1)/2` gives exactly `π/2`, a single topmost vertex (the apex);
- `n` even: no `k` does; `k = n/2 − 1` and `k = n/2` give `π/2 ∓ π/n`, a flat top edge; the leftmost of the two has the smaller cosine, at `π/2 + π/n` (cosine negative), i.e. `k = n/2`.

Both cases give `k = ⌊n/2⌋`. The `j`-th vertex in drawing order (`j = 0 … n−1`) has the angle

`βⱼ = −π/2 + (2⌊n/2⌋ + 1 − 2j) π / n`.

The start is chosen by its index, never by comparing floating coordinates.

The code (`unitRegularPolygon`, EN-008) hands the unit polygon over **already flipped** to the SVG frame: `(cos βⱼ, −sin βⱼ)`. Step 5 then uses the flipped ordinates `y′ = −y` (below).

## Step 3 — Unit bounding box

`Wᵤ = max xₖ − min xₖ`, `Hᵤ = max yₖ − min yₖ`.

## Step 4 — Uniform scale

`s = min(w / Wᵤ, h / Hᵤ)`.
At least one dimension is reached (`w` or `h`, or both), exactly in theory; in floating point within a rounding error relative to the box (about 1e-16 × its size: 1e-13 for a box of 1000, 3.7e-9 for 1e7, but a few units near the largest safe integer: 2 for a square of side 2⁵³ − 1, AUD-003). `Wᵤ` and `Hᵤ` are positive for `n ≥ 3`; a size of 0 gives `s = 0`: every vertex is at the anchored place of the box, its center for `mid` on both axes (Q15, step 5).

## Step 5 — Placement in the SVG frame

The scaled polygon spans `s·Wᵤ × s·Hᵤ` and leaves the room `Δx = w − s·Wᵤ ≥ 0` and `Δy = h − s·Hᵤ ≥ 0` (step 4), zero on the axis it fills. It is placed in that room by a horizontal and a vertical **alignment**, each `min`, `mid` or `max` (F07). This is SVG's `preserveAspectRatio` with `meet` (`REF-SVG2-COORDS` §8.2, §8.7): the scale is the smaller of the two ratios (§8.2 step 7), and the translation adds nothing for `xMin`, `Δx / 2` for `xMid` (step 11), `Δx` for `xMax` (step 12), the same for `YMin`, `YMid`, `YMax` (steps 13–14); `min` aligns the smallest coordinate with the box's, `max` the largest (§8.7). With the y axis flipped (y downwards, `REF-SVG2-COORDS`), `min` is the left or the top, `max` the right or the bottom.

`x = tₓ + s · (xₖ − min xₖ)`, `y = t_y + s · (max yₖ − yₖ)`, with `tₓ ∈ {0, Δx / 2, Δx}` and `t_y ∈ {0, Δy / 2, Δy}`.

The highest vertex of the mathematical frame becomes the one with the smallest SVG y: the picture is unchanged, only its coordinates are.

With the flipped unit polygon of step 2 (`y′ₖ = −yₖ`, so `max yₖ − yₖ = y′ₖ − min y′ₖ`), the same placement reads `y = t_y + s · (y′ₖ − min y′ₖ)`: no second flip.

**On the anchored side.** For `min`, the extreme vertex is at `x = 0 + s · 0 = 0`; for `max`, at `Δx + s · Wᵤ = w`. In floating point these hold within a rounding error relative to the box, like step 4: the unit vertices of one edge come from `cos` and `sin` and may differ in their last bit (the base of the unit triangle at y = 0.5 and 0.5000000000000003), so a base anchored at the bottom of a 100 box may be computed at 99.99999999999999. The output writes 5 decimals (Q10): every vertex of that edge is written `100`, the integer edge of the box. For `mid`, the placement of F02 is kept unchanged: `x = Δx / 2 + s · (xₖ − min xₖ)`.

On the axis the polygon fills, `Δ = 0` in theory and the three alignments give the same points; in floating point they differ by a rounding error relative to the box (about 1e-16 × its size, step 4), far below what the output writes (5 decimals, Q10): the written drawing is the same, unless a coordinate falls on a rounding boundary of the 5th decimal (none in boxes from 0 to 1000 for n = 3 … 12, review of EN-009).

The default is `mid` on both axes: F02's centering. In the code, steps 3–5 are `boundingBox` and `fitInBox` (its share of the room: `alignmentFactor`, EN-009), and the whole fit is `regularPolygonContour` (US-005, US-009).

## Step 6 — Maximal rounding: the incircle

At the maximal radius, every edge carries two equal fillets, clamped to meet at its middle (`DERIV-local-radius-clamp`): setback `a/2`. With the interior angle `θ = π − 2π/n`, the radius is `ρ = (a/2) · tan(θ/2)` (`DERIV-fillet-setback` step 4), and `tan(π/2 − π/n) = cot(π/n)`:

`ρ = ½ a cot(π/n)`, the inradius `r` of the polygon (`REF-MATHWORLD-REGULAR-POLYGON` (3)).

The fillet's center `D` is the point of the bisector at distance `ρ` from both edges of the corner `B` (`DERIV-fillet-setback` step 1). The incircle is tangent to every side (`REF-MATHWORLD-INCIRCLE`): its center `O` is at distance `r = ρ` from both edges, the feet `E` and `F` of its perpendiculars being the points of contact (Euclid III.18, `REF-EUCLID-III18`). The right triangles `BEO` and `BFO` share the hypotenuse `BO` and have `OE = OF`, so `BE = BF` (Euclid I.47, `REF-EUCLID-I47`); their three sides being equal, their angles at `B` are equal (Euclid I.8, `REF-EUCLID-I8`): `O` lies on the bisector. On the bisector, the distance to the edges grows with the distance to `B`, so one point only is at distance `ρ`: `O = D`. Every fillet is therefore an arc of the incircle, and the fillets meet at the middles of the edges: the rounded polygon **is** its incircle, of radius `s · cos(π/n)` once scaled (`REF-MATHWORLD-REGULAR-POLYGON` (4), `R = s`). In the code nothing is specific to polygons: `regularPolygonCorners` gives every vertex the requested radius and the clamp of F01 finds the incircle (US-006).

## Checks

| `n` | Box       | `Wᵤ × Hᵤ`         | Result                 | Vertices in drawing order (5 decimals)                                                | Maximal radius |
| --- | --------- | ----------------- | ---------------------- | ------------------------------------------------------------------------------------- | -------------- |
| 3   | 100 × 100 | √3 × 1.5          | 100 × 86.60254 (width) | (50, 6.69873), (100, 93.30127), (0, 93.30127)                                         | 28.86751       |
| 4   | 100 × 50  | √2 × √2           | 50 × 50 (height)       | (25, 0), (75, 0), (75, 50), (25, 50)                                                  | 25             |
| 5   | 100 × 100 | 1.90211 × 1.80902 | 100 × 95.10565 (width) | (50, 2.44717), (100, 38.7743), (80.9017, 97.55283), (19.0983, 97.55283), (0, 38.7743) | 42.53254       |
| 6   | 100 × 100 | 2 × √3            | 100 × 86.60254 (width) | (25, 6.69873), (75, 6.69873), (100, 50), (75, 93.30127), (25, 93.30127), (0, 50)      | 43.30127       |
| 8   | 100 × 100 | 1.84776 × 1.84776 | 100 × 100 (both)       | (29.28932, 0), (70.71068, 0), (100, 29.28932), …, (0, 29.28932)                       | 50             |

Anchored (F07): the room is all before (`max`) or all after (`min`) the shape instead of split in two.

| `n` | Box       | Alignment (horizontal, vertical) | Vertices in drawing order (5 decimals)                                               |
| --- | --------- | -------------------------------- | ------------------------------------------------------------------------------------ |
| 3   | 100 × 100 | any, `max` (bottom)              | (50, 13.39746), (100, 100), (0, 100)                                                 |
| 3   | 100 × 100 | any, `min` (top)                 | (50, 0), (100, 86.60254), (0, 86.60254)                                              |
| 4   | 100 × 50  | `min` (left), any                | (0, 0), (50, 0), (50, 50), (0, 50)                                                   |
| 4   | 100 × 50  | `max` (right), any               | (50, 0), (100, 0), (100, 50), (50, 50)                                               |
| 6   | 100 × 100 | any, `max` (bottom)              | (25, 13.39746), (75, 13.39746), (100, 56.69873), (75, 100), (25, 100), (0, 56.69873) |

By hand: the triangle's room is `Δy = 100 − 86.60254 = 13.39746`; at the bottom the apex is at `y = 13.39746` and the base at `100`; at the top the apex is at `0` and the base at `86.60254`. The square of side 50 leaves `Δx = 50`: left from `x = 0`, right from `x = 50`. The hexagon is shifted down by `13.39746 − 6.69873 = 6.69873` from its centered place.

By hand: `n = 3`, `s = 100/√3 = 57.73503`, height `1.5 s = 86.60254`, top margin `(100 − 86.60254)/2 = 6.69873`; maximal radius `s cos(π/3) = 28.86751`. `n = 6`, `s = 50`, maximal radius `50 cos(π/6) = 43.30127`, a circle of diameter 86.60254.

## Note

The fit is computed on the **sharp-cornered** polygon. A corner radius then rounds inwards: a vertex that touched the box no longer does, a flat edge that touched it still does; at the maximal radius only the incircle remains (accepted by the Product Owner, 2026-10-09).
