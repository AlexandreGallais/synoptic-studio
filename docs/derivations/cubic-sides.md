# DERIV-cubic-sides — A cubic side: its extremes, its split in half, its handles turned

Used by: ADR-0046, F12 (EN-025). Sources: `REF-WIKI-BEZIER` (the cubic, its derivative, affine maps on the control points), `REF-POMAX-BEZIER` (bounding box from the roots of the derivative; splitting by de Casteljau), `REF-WIKI-QUADRATIC` (roots without cancellation), `REF-MATHWORLD-ROTATION-MATRIX`.

## Notation

| Symbol           | Definition                                                                               |
| ---------------- | ---------------------------------------------------------------------------------------- |
| `P₀, P₁, P₂, P₃` | a cubic side: from point A = P₀ to point B = P₃, `P₁ = A + A.after`, `P₂ = B + B.before` |
| `B(t)`           | the point of the curve at `t` from 0 to 1                                                |
| `w₀ … w₃`        | the values of one axis, x or y, of the four points                                       |

## Step 1 — The curve and its derivative

`B(t) = (1−t)³P₀ + 3(1−t)²t P₁ + 3(1−t)t² P₂ + t³P₃`, and `B′(t) = 3(1−t)²(P₁−P₀) + 6(1−t)t(P₂−P₁) + 3t²(P₃−P₂)` (`REF-WIKI-BEZIER`).

## Step 2 — The derivative as a quadratic in t

Expanding step 1 on one axis (the Bernstein weights `3(w₁−w₀)`, `3(w₂−w₁)`, `3(w₃−w₂)` of the derivative, `REF-POMAX-BEZIER`): `B′(t) = a t² + b t + c` with

`a = 3(−w₀ + 3w₁ − 3w₂ + w₃)`, `b = 6(w₀ − 2w₁ + w₂)`, `c = 3(w₁ − w₀)`.

Check at t = 0: `B′(0) = c = 3(w₁ − w₀)`, as step 1 gives.

## Step 3 — The extremes, without cancellation

The curve's box on an axis is the smallest and largest of `w₀`, `w₃` and `B(t)` at each root of `a t² + b t + c = 0` with `0 < t < 1` (`REF-POMAX-BEZIER`). The roots are taken as `q = −(b + sign(b) √D) / 2`, `D = b² − 4ac`, `t₁ = q / a`, `t₂ = c / q`, `sign(0) = +1` (`REF-WIKI-QUADRATIC`: `t₁ t₂ = c / a`). A division by zero gives an infinite or NaN `t`, which fails `0 < t < 1`: `a = 0` leaves the linear root `−c / b`; `a = b = 0` leaves none; `D < 0` none.

## Step 4 — Split in half (de Casteljau)

At `t = ½`: `P₀₁ = (P₀+P₁)/2`, `P₁₂ = (P₁+P₂)/2`, `P₂₃ = (P₂+P₃)/2`, `P₀₁₂ = (P₀₁+P₁₂)/2`, `P₁₂₃ = (P₁₂+P₂₃)/2`, `M = (P₀₁₂+P₁₂₃)/2 = B(½)`; the two halves `P₀, P₀₁, P₀₁₂, M` and `M, P₁₂₃, P₂₃, P₃` lie exactly on the curve (`REF-POMAX-BEZIER`, splitting curves). The new point M gets the handles `before = P₀₁₂ − M`, `after = P₁₂₃ − M`: opposite and of the same length, since M is their midpoint (mode angle and length); A's handle after becomes `P₀₁ − P₀`, half of it, and B's handle before `P₂₃ − P₃`.

## Step 5 — Handles turned

Turning a shape about a center is an affine map; applied to the control points, it maps the curve (`REF-WIKI-BEZIER`). A handle is a difference of points, so it takes the rotation only: `h′ = R(θ) h`, with `R(θ)` turning clockwise on screen as `turnDirection` (`REF-MATHWORLD-ROTATION-MATRIX`).

## Checks (test cases)

| Case                                                 | Expected                                                                                                    |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| (100, 0), (130, −15), (200, 0), (200, 0)             | y: a = −135, b = 180, c = −45, root t = ⅓; box (100, −20/3) – (200, 0)                                      |
| (0, 0), (0, −40), (100, −40), (100, 0): `a = 0` on y | root t = ½; box (0, −30) – (100, 0)                                                                         |
| (0, 0), (30, −30), (70, 30), (100, 0)                | roots (3 ∓ √3)/6; box (0, −5√3) – (100, 5√3)                                                                |
| (0, 0), (0, 0), (150, 0), (100, 0)                   | x root 6/7; largest x 5400/49 ≈ 110.20408                                                                   |
| four equal points (5, 5)                             | no root; box (5, 5) – (5, 5)                                                                                |
| (0, 0), (0, −40), (100, −40), (100, 0) split in half | M = (50, −30); halves (0, 0), (0, −20), (25, −30), (50, −30) and (50, −30), (75, −30), (100, −20), (100, 0) |
| handle (30, −15) turned by 90°                       | (15, 30)                                                                                                    |
