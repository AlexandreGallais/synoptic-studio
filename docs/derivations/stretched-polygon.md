# DERIV-stretched-polygon — A polygon stretched to its box, sides equal while its corners stay sound

Used by: ADR-0037, F06 (rule C chosen by the Product Owner, research note 0011). Sources: `REF-CRANE-DISCRETE-CURVES` (a plane polygon is fixed by its side lengths and turning angles), `REF-CARROLL-2013` (the turning angle), `REF-OPENSTAX-UNIT-CIRCLE`, `REF-MATHWORLD-VECTOR-ADDITION`, `REF-MATHWORLD-AFFINE`, `REF-MDN-ATAN2`, `REF-MDN-ACOS`, `REF-MATHWORLD-BISECTION`, `REF-SVG2-COORDS`; `DERIV-regular-polygon-fit` steps 1–2, `DERIV-turning-angle`, `DERIV-free-shape-stretch`.

## Inputs

`n` corners, `3 ≤ n ≤ 12` (Q19); the box `w × h`, integers ≥ 0; its ratio `r = h / w`.

## Notation

| Symbol | Definition                                                                                         |
| ------ | -------------------------------------------------------------------------------------------------- |
| `m`    | sides of the right half after the half base: `(n − 1)/2` for an odd `n`, `n/2 − 1` for an even `n` |
| `θⱼ`   | direction of side `j` of the regular polygon, `2πj/n`, `j = 1 … m`                                 |
| `λ`    | vertical scale of those directions, `λ > 0`; `λ = 1` is the regular polygon                        |
| `φⱼ`   | direction of side `j` of the stretched polygon                                                     |
| `τ`    | turning angle at a corner                                                                          |
| `A(λ)` | aspect of the unit polygon: its height over its width                                              |

## Step 1 — A symmetric polygon of sides 1, built from directions

Mathematical frame (y up). A plane polygon is fixed, up to its place, by its side lengths and turning angles (`REF-CRANE-DISCRETE-CURVES`); so a polygon of sides 1 is fixed by the directions of its sides. A side of length 1 and direction `φ` is the vector `(cos φ, sin φ)` (`REF-OPENSTAX-UNIT-CIRCLE`), and the vertices are the running sums of the sides (`REF-MATHWORLD-VECTOR-ADDITION`).

The polygon is symmetric about the vertical axis `x = 0`, its base on `y = 0` (F06). Only its right half is built: from `R₀ = (1/2, 0)`, the right end of the half base, `Rⱼ = Rⱼ₋₁ + (cos φⱼ, sin φⱼ)` for `j = 1 … m`, with `0 < φ₁ < … < φₘ < π`, so that `y` grows along the half. For an odd `n`, `Rₘ` is the apex, on the axis: `1/2 + Σ cos φⱼ = 0`. For an even `n`, `Rₘ` is the right end of the top side, horizontal, which goes back to the axis: `Σ cos φⱼ = 0`, so `Rₘ = (1/2, H)`. The left half is the mirror image `(x, y) ↦ (−x, y)`; the apex of an odd `n` is its own image.

## Step 2 — The λ family: the regular directions scaled vertically

In the regular polygon of `DERIV-regular-polygon-fit` step 1, the vertices are `2π/n` apart on the circle; turning the polygon by `2π/n` maps each side onto the next, so the directions of the sides grow by `2π/n`: from the base (direction 0), `θⱼ = 2πj/n`.

The stretched polygon keeps these directions scaled vertically by `λ`, the axis scaling `(x, y) ↦ (x, λy)`, an affine map, which does not keep angles nor lengths (`REF-MATHWORLD-AFFINE`); the order of the directions is enforced by the corner guard of step 3, a negative turn failing it: the side of direction `θⱼ`, `(cos θⱼ, sin θⱼ)`, becomes `(cos θⱼ, λ sin θⱼ)`, of direction

`φⱼ = atan2(λ sin θⱼ, cos θⱼ)` (`REF-MDN-ATAN2`: the angle in `[−π, π]` of the point `(x, y)`; here `sin θⱼ > 0`, so `φⱼ ∈ (0, π)`), and every side is given length 1 again.

- **Even `n`**: `θⱼ` and `θ_{n/2 − j} = π − θⱼ` have opposite cosines and equal sines, so `φ_{n/2−j} = π − φⱼ`: their cosines cancel, and `θ_{n/4} = π/2` gives `φ = π/2`. The closure `Σ cos φⱼ = 0` of step 1 holds for every `λ`.
- **Odd `n`**: the last side closes the half on the axis: `cos φₘ = −1/2 − Σ_{j<m} cos φⱼ`, `φₘ = acos(…)` in `[0, π]` (`REF-MDN-ACOS`); outside `[−1, 1]` no side closes it, and `acos` gives `NaN`: such a `λ` has no polygon.

At `λ = 1` every `φⱼ = θⱼ` (the odd closure gives `θₘ` back, the regular polygon closing by symmetry): the regular polygon. For `n = 5` and 6 (`m = 2`: the closure and the aspect fix both directions) this family gives the only equal-sided polygon filling the box; from 7 on, the octagon included (`m = 3`, one freedom left), it picks one of several (note 0011).

## Step 3 — Turning angles and the corner guard

The turning angle at a corner is the angle between the directions of the two sides meeting there (`REF-CARROLL-2013`; `DERIV-turning-angle`): `φ₁` at each base corner (from the base, direction 0), `φⱼ₊₁ − φⱼ` at the next corners, `2(π − φₘ)` at the apex of an odd `n` (the side after it is the mirror of side `m` run downwards, `(cos φₘ, −sin φₘ)`, of direction `2π − φₘ`), and `π − φₘ` at each top corner of an even `n` (towards the top side, direction `π`). The regular polygon turns `2π/n` at every corner.

**Corner guard** (Product Owner, 2026-10-12): `λ` is **sound** when every corner turns at least `π/n`, half of its regular turn. A `λ` without polygon (step 2) is not sound. The sound `λ` form one interval `[λ_lo, λ_hi]` containing 1; this is checked numerically, not proved (step 5).

In the code the turns are those of the whole polygon in the SVG frame (`turningAngles`, `DERIV-turning-angle`, clockwise positive on screen): the same values, and a `NaN` coordinate gives a `NaN` turn, which fails the guard.

## Step 4 — The aspect

The unit polygon spans `2W` in width, `W` the largest `x` of the right half, and `H` in height, the largest `y` (the apex or the top side, `y` growing along the half): `A(λ) = H / (2W)`. `A` grows with `λ` for `n ≥ 5` wherever the polygon closes (checked, step 5). The triangle (`m = 1`, `φ₁ = 2π/3` by closure) and the square (`φ₁ = π/2`) do not depend on `λ`: one shape each, `A = √3/2` and 1, always sound.

## Step 5 — Choosing λ (rule C)

Bisection keeps a bracket `[a, b]` around the solution and halves it at each step; after `k` steps the error is at most `2⁻ᵏ (b − a)` (`REF-MATHWORLD-BISECTION`). It runs on `ln λ`, in `[−3, 3]`, which contains every sound `λ` of `n = 5 … 12`: from `λ_lo = 1/3` (hexagon) to `λ_hi = 6.74` (pentagon, `ln λ = 1.908`), found by the sweep below and confirmed by the review of SP-009; 60 steps: an error below `3 · 2⁻⁶⁰ ≈ 2.6e-18` in exact arithmetic; in doubles the bracket stops shrinking at one unit in the last place of `ln λ` (about 4e-16, after some 54 steps), still far under `EPSILON`. A bound found this way may lie on the unsound side by that much: a clamped polygon's guard corner turns `π/n` within about 1e-16.

1. `λ_lo`: the bisection in `[−3, 0]` on « not sound » (true at `−3`, false at 0; for the triangle and the square, sound everywhere, it ends at `−3`, which gives their only shape).
2. `λ_hi`: the bisection in `[0, 3]` on « sound ».
3. The bisection in `[ln λ_lo, ln λ_hi]` on `A(λ) < r`. If `r ≤ A(λ_lo)`, the predicate is false everywhere and the bisection ends at `λ_lo`; if `r ≥ A(λ_hi)`, true everywhere, it ends at `λ_hi` (the box is too wide or too tall for sound equal sides): the clamp of rule C needs no step of its own.

Each bisection needs its predicate to change once in its bracket: « sound » on each side of 1, and `A(λ) < r` between `λ_lo` and `λ_hi`. Neither is proved (note 0011); both are checked by the tests over `ln λ` from −3 to 3 in steps of 0.001 (6 001 values): « sound » changes at most twice and holds at `λ = 1` for every `n` from 3 to 12 (never changes for 3 and 4); `A` never decreases where the polygon closes (`n = 5` and 9 do not close below `ln λ ≈ −0.58` and `−0.71`), constant for 3 and 4. The triangle and the square, independent of `λ`, give their only shape whatever the branch.

A box of size 0 gives `r = 0`, `+∞` or `NaN` (`0/0`): every comparison with `NaN` is false, so the last bisection ends at `λ_lo`; any `λ` is right, the polygon being then flattened to a line or a point (step 6).

## Step 6 — Placing it in the box

The full polygon in drawing order (`DERIV-regular-polygon-fit` step 2, clockwise on screen from the right end of the base): `R₀`, its mirror `L₀`, `L₁ … Lₘ₋₁`, then the apex `Rₘ` (odd `n`) or `Lₘ, Rₘ` (even `n`), then `Rₘ₋₁ … R₁`; `n` points. It goes to the SVG frame with its box at the origin: `x = W + X`, `y = H − Y` (y down, `REF-SVG2-COORDS` §8.4). Then each axis is scaled to the box, the top-left fixed (`stretchToSize`, `DERIV-free-shape-stretch`): every side of the box is touched, the far ones within one unit in the last place, since the span of the unit polygon is irrational (a base at `y = 6.000000000000001` in a box of height 6), far below the 5 decimals written (Q10). When `r` lies within `[A(λ_lo), A(λ_hi)]`, `A(λ) = r` and both factors are equal: the sides stay equal; past the guard, the limit polygon is stretched on each axis (rule C). The triangle becomes isosceles and the square the rectangle of the box: the plain stretch, as they have no freedom.

## Checks (test cases)

Values of note 0011, recomputed by the agent's own implementation of these steps (5 decimals, SVG frame, drawing order).

| Case                              | Expected                                                                                                                     |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| any `n`, `r` = the regular aspect | `λ = 1`: the polygon of F02 filling its box                                                                                  |
| triangle 100 × 100                | (100, 100), (0, 100), (50, 0)                                                                                                |
| square 300 × 50                   | (300, 50), (0, 50), (0, 0), (300, 0)                                                                                         |
| pentagon 100 × 100                | (81.75623, 100), (18.24377, 100), (0, 39.16418), (50, 0), (100, 39.16418); every side 63.51246                               |
| pentagon 300 × 50                 | (228.85967, 50), (71.14033, 50), (0, 12.85950), (150, 0), (300, 12.85950)                                                    |
| hexagon 100 × 100                 | (77.42919, 100), (22.57081, 100), (0, 50), (22.57081, 0), (77.42919, 0), (100, 50); every side 54.85838                      |
| hexagon 300 × 50                  | (204.90381, 50), (95.09619, 50), (0, 25), (95.09619, 0), (204.90381, 0), (300, 25)                                           |
| hexagon 50 × 300                  | (41.47297, 300), (8.52703, 300), (0, 150), (8.52703, 0), (41.47297, 0), (50, 150)                                            |
| octagon 100 × 50                  | (67.55767, 50), (32.44233, 50), (0, 39.16136), (0, 10.83864), (32.44233, 0), (67.55767, 0), (100, 10.83864), (100, 39.16136) |
| heptagon 100 × 100                | (72.54712, 100), (27.45288, 100), (0, 64.22529), (9.67353, 20.18085), (50, 0), (90.32647, 20.18085), (100, 64.22529)         |

By hand, without the harmonic identity of note 0011: hexagon 100 × 100. With `φ₂ = π − φ₁` (step 2), `A = 2 sin φ₁ / (1 + 2 cos φ₁)`; at `φ₁ = 65.7048°`, `2 sin φ₁ = 1.82288` and `1 + 2 cos φ₁ = 1.82288`: `A = 1 = r`. The half width `1/2 + cos φ₁ = 0.91144`, the scale `50 / 0.91144 = 54.858` is the side, and the base ends at `x = 50 ± 27.429`. Hexagon 300 × 50, at its guard: `φ₁ = 30°` turns exactly `π/6`; width `1 + 2 cos 30° = 2.73205`, height `2 sin 30° = 1`, `A = 0.36603 > 50/300`: `λ = λ_lo`, x scale `300 / 2.73205 = 109.808` (the base), base end `x = 150 + 54.904 = 204.904`.

## Limits

- The two monotonic properties of step 5 are checked, not proved; a proof would remove the sweeps from the tests.
- `λ_lo` and `λ_hi` depend on `n` only: computed at each call (three bisections of 60 steps on `n` points, negligible), not tabulated, so that no table needs a derivation of its own.
- In very elongated boxes, every corner of the limit polygon is stretched: sides and corners become uneven, less than with the plain stretch of the regular polygon (note 0011).
