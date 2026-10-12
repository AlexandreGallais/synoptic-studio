# DERIV-turning-angle — Turning angle at a contour vertex

Used by: EN-003, `DERIV-local-radius-clamp` (notation `τᵢ`). Sources: `REF-MATHWORLD-VECTOR-ADDITION`, `REF-MATHWORLD-VECTOR-DIFFERENCE`, `REF-MATHWORLD-DOT`, `REF-MATHWORLD-PERP-DOT`, `REF-MATHWORLD-ROTATION-MATRIX`, `REF-MDN-ATAN2`.

## Notation

Cyclic contour `v₀ … vₙ₋₁`; indices are taken modulo `n` (Q11): the vertex before `v₀` is `vₙ₋₁`, the vertex after `vₙ₋₁` is `v₀`.

| Symbol          | Definition                                   |
| --------------- | -------------------------------------------- |
| `a = vᵢ − vᵢ₋₁` | incoming edge at `vᵢ`                        |
| `b = vᵢ₊₁ − vᵢ` | outgoing edge at `vᵢ`                        |
| `τᵢ`            | turning angle at `vᵢ`: angle from `a` to `b` |

## Step 1 — Edge vectors

A vector difference is the sum with the second vector reversed, `A − B = A + (−B)` (`REF-MATHWORLD-VECTOR-DIFFERENCE`), and vectors add component-wise (`REF-MATHWORLD-VECTOR-ADDITION`). Hence `a = (aₓ, a_y) = (xᵢ − xᵢ₋₁, yᵢ − yᵢ₋₁)`.

## Step 2 — Cosine and sine of the turn

- Dot product: `a · b = aₓbₓ + a_y b_y = |a| |b| cos τ` (`REF-MATHWORLD-DOT`).
- Perp dot product: `a⊥ · b = |a| |b| sin τ`, `τ` measured from `a` to `b`, with `a⊥` = `a` rotated a quarter turn to the left (`REF-MATHWORLD-PERP-DOT`). The rotation matrix with θ = π/2 gives its components, `a⊥ = (−a_y, aₓ)` (`REF-MATHWORLD-ROTATION-MATRIX`), hence `a⊥ · b = aₓ b_y − a_y bₓ` (dot product in components, `REF-MATHWORLD-DOT`).

## Step 3 — Angle

`atan2(y, x)` returns the angle, in `[−π, π]`, between the positive x-axis and the point `(x, y)`, counter-clockwise (`REF-MDN-ATAN2`). It only depends on the direction of `(x, y)`, so for `|a| |b| > 0`:

`τ = atan2(a⊥ · b, a · b) = atan2(|a||b| sin τ, |a||b| cos τ)`.

## Frame

The formulas hold in the mathematical frame (y up). In the SVG frame (y down) the same numbers describe the mirrored drawing: **a positive `τ` is a clockwise turn on screen**. A contour listed clockwise on screen (Q11) therefore has positive turning angles at its convex vertices. The fillet computations only use `|τ|` (`DERIV-local-radius-clamp`).

## Checks (test cases)

| Vertices `vᵢ₋₁, vᵢ, vᵢ₊₁`                                                  | `a`, `b`           | `a⊥·b`, `a·b` | `τ`                                  |
| -------------------------------------------------------------------------- | ------------------ | ------------- | ------------------------------------ |
| (0, 0), (100, 0), (100, 50)                                                | (100, 0), (0, 50)  | 5000, 0       | `π/2` (clockwise on screen)          |
| (0, 50), (0, 0), (100, 0) — first vertex of the rectangle, previous = last | (0, −50), (100, 0) | 5000, 0       | `π/2`                                |
| (0, 0), (5, 0), (10, 0)                                                    | (5, 0), (5, 0)     | 0, 25         | `0` (aligned)                        |
| (0, 0), (10, 0), (10, −10)                                                 | (10, 0), (0, −10)  | −100, 0       | `−π/2` (counter-clockwise on screen) |
| (0, 0), (10, 0), (0, 0)                                                    | (10, 0), (−10, 0)  | 0, −100       | `π` (back-turn)                      |

## Limits

- A zero-length edge (repeated vertex, e.g. a rectangle of width 0, Q15) has no direction: both products are 0 and τ is taken as 0 — no turn, hence no fillet. This is stated explicitly, not left to `atan2(0, 0)`: with signed zeros, `atan2(+0, −0) = π` in IEEE 754 (found by the EN-005 audit, e.g. a = (−10, −10), b = (0, 0)). The sign of that zero is not meaningful.
