# DERIV-turned-shape — A shape turned about the center of its box, and kept in place when edited

Used by: ADR-0035, F04. Sources: `REF-SVG11-TRANSFORM` §7.4 (the rotation matrix) and §7.6 (a rotation about a point is translate, rotate, translate back), `REF-CSS-TRANSFORMS` §9.1 (a positive angle appears clockwise), `REF-SVG2-COORDS` §8.4 (y pointing down), `REF-MDN-REMAINDER` (sign of `%`), `REF-MATHWORLD-ROTATION-MATRIX` (the quarter turn).

## Notation

| Symbol          | Definition                                                                          |
| --------------- | ----------------------------------------------------------------------------------- |
| `a`             | the angle typed, an integer number of degrees                                       |
| `α`             | the angle kept, an integer from 0 to 359                                            |
| `b = (bₓ, b_y)` | top-left of the shape's box before it turns, in the SVG (its position, ADR-0035)    |
| `w, h`          | width and height of the box; `k = (w/2, h/2)` its half size; `c = b + k` its center |
| `l`             | a point of the straight shape, relative to the top-left of its box (local)          |
| `R`             | the rotation by `α`: `R(x, y) = (x cos α − y sin α, x sin α + y cos α)`             |
| `D(l)`          | the point drawn                                                                     |

## Step 1 — The angle kept from 0 to 359

`%` in ECMAScript keeps the sign of the dividend (`REF-MDN-REMAINDER`): `−90 % 360` is `−90`. The modulo is `((a % 360) + 360) % 360` (same source): `370 ↦ 10`, `−90 ↦ 270`, `360 ↦ 0`, `−360 ↦ 0` (`−0 + 360 = 360`, `360 % 360 = 0`, a positive zero). Every operand is an integer below 2⁵³: exact.

## Step 2 — Clockwise on screen

SVG's rotation about the origin is the matrix `[cos a, sin a, −sin a, cos a, 0, 0]` (`REF-SVG11-TRANSFORM` §7.4), so `x' = x cos a − y sin a`, `y' = x sin a + y cos a`: the formula of `R`, which MathWorld gives as counterclockwise with y up (`REF-MATHWORLD-ROTATION-MATRIX`). With y pointing down (`REF-SVG2-COORDS` §8.4) the same formula appears clockwise, as CSS says of `rotate(90deg)` (`REF-CSS-TRANSFORMS` §9.1). Check: `R` at 90° maps `(1, 0)` (right) to `(0, 1)` (down): a quarter turn clockwise on screen.

## Step 3 — Exact quarter turns

`cos(π/2)` in floating point is `6.1e-17`, not 0: a square turned by 90° would leave rounding dust. Write `α = 90q + r` with `q ∈ {0, 1, 2, 3}` and `0 ≤ r < 90`. Then `R_α = R_90^q ∘ R_r`, and the quarter turn is `(x, y) ↦ (−y, x)` (`REF-MATHWORLD-ROTATION-MATRIX`), exact in floating point. So `(cos α, sin α)` is `(cos r, sin r)` turned `q` times: at 0°, 90°, 180°, 270° the pair is exactly `(1, 0)`, `(0, 1)`, `(−1, 0)`, `(0, −1)`.

## Step 4 — About the center of the box

A rotation about a point is translate, rotate, translate back (`REF-SVG11-TRANSFORM` §7.6). About `c`:

`D(l) = c + R(b + l − c) = b + k + R(l − k)`

The center maps to itself (`l = k`), so turning never moves the center of the box (Product Owner, F04). At `α = 0` the straight shape is drawn as it is, without computing (`(x − c) + c` may differ from `x` by an ulp, and every earlier test expects the exact straight drawing). The same holds for the position: a free shape whose position `b` is the top-left `o` of its stored points is drawn by the earlier formula, as it is (`(p − o) + o` may also differ from `p` by an ulp); only `b ≠ o` adds `b − o`.

## Step 5 — Kept in place when the box changes

Editing a turned shape changes its box (`w, h` typed, a point moved). To keep a local point `l'` of the new box `(w', h')` at a point `P` of the SVG, solve step 4 for the top-left:

`b' = P − k' − R(l' − k')`

- **A width or height typed**: the turned top-left corner stays in place (Product Owner, F04, as Figma). It is `l = (0, 0)` before and after: `P = b + k − R(k)`, then `b' = P − k' + R(k')`.
- **A point typed**: un-turn the point typed, `u = c + R⁻¹(T − c)` (`R⁻¹` turns by `−α`), edit the straight shape as at `α = 0` (ADR-0033), then place the new box so that a point not moved keeps its drawn place `P`. The stored `u` is fractional, and the drawn point is `T` within a rounding error (`R⁻¹` then `R` in floating point): the integer typed holds in the drawing, written at 5 decimals (Q10), not in the model (ADR-0035).
- **A turned polygon converted**: its free shape has the box of its points, not the polygon's box; placed so that its first point keeps its drawn place.

The shape is rigid under `R` (a rotation keeps distances): once one point is in place, every point not moved is too.

## Checks (test cases)

| Case                                                 | Computation                                             | Expected                                               |
| ---------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------ |
| Angles typed 370, −90, 360, −360, 45                 | `((a % 360) + 360) % 360`                               | 10, 270, 0, 0, 45                                      |
| Rectangle 100 × 50 at `b = (0, 0)`, 90°              | `c = (50, 25)`; `(−50, −25) ↦ (25, −50)`                | points (75, −25), (75, 75), (25, 75), (25, −25)        |
| Same, straight box                                   | x from 25 to 75, y from −25 to 75                       | x 25, y −25, 50 × 100                                  |
| Square 100 × 100 at `(0, 0)`, 45°                    | `(−50, −50) ↦ (0, −70.71068)`                           | point 1 at (50, −20.71068); straight box 141.42136     |
| Rectangle 100 × 50, 90°, width typed 200             | `P = (75, −25)`, `k' = (100, 25)`, `R(k') = (−25, 100)` | `b' = (−50, 50)`; point 1 still at (75, −25)           |
| Any shape, 0°                                        | not computed                                            | the straight drawing, bit for bit                      |
| Rectangle 100 × 50 at 90°, 180°, 270°                | quarter turns exact                                     | every coordinate exact, no rounding dust (`Object.is`) |
| The 90° drawing map applied four times to the points | four exact quarter turns about `c`                      | the straight points exactly                            |
