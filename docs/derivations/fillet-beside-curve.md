# DERIV-fillet-beside-curve — A corner fillet beside a cubic side

Used by: F12 (SP-012, US-027). A corner in radius mode keeps its rounding when one or both of its sides are cubic (ADR-0047). The circle is tangent to both sides; its centre is where their offsets meet.

## Notation

| Symbol | Definition                                                             |
| ------ | ---------------------------------------------------------------------- |
| `V`    | the corner point                                                       |
| `a`    | unit direction of a straight side, from `V` towards its other point    |
| `B(t)` | the cubic side written from `V`: `B(0) = V`, `t` in `[0, 1]`           |
| `b`    | unit tangent of `B` at `V`                                             |
| `⊥`    | the quarter turn `(x, y) ↦ (−y, x)`                                    |
| `σ`    | `sign(a⊥ · b)`: on which side of the straight side the curve leaves    |
| `nL`   | `σ a⊥`, unit normal of the straight side, towards the curve            |
| `n(t)` | `−σ unit(B′(t))⊥`, unit normal of the curve, towards the straight side |
| `r`    | the fillet radius, `r ≥ 0`                                             |

## Step 0 — A curve written from the corner

A cubic with its control points reversed traces the same points backwards, `B_rev(t) = B(1 − t)`: changing `t` into `1 − t` swaps the Bernstein weights `(1−t)³, 3(1−t)²t, 3(1−t)t², t³` in pairs (`REF-WIKI-BEZIER`). A side coming into `V` is reversed, so that every curve starts at `V`.

## Step 1 — Tangent at a corner whose handle is off

`B′(t) = 3(1−t)²(P₁−P₀) + 6(1−t)t(P₂−P₁) + 3t²(P₃−P₂)` (`REF-WIKI-BEZIER`); the tangent is its direction (`REF-POMAX-BEZIER`, "Tangents and normals"). The corner is in radius mode, its handle off, so `P₁ = P₀ = V` and `B′(t) = 3t[2(1−t)(P₂−P₀) + t(P₃−P₂)]`, whose direction tends to `P₂ − P₀` as `t → 0`; with `P₂ = P₀` too, `B′ = 3t²(P₃ − P₀)`. So `b` is the unit vector of the first non-zero among `P₁ − P₀`, `P₂ − P₀`, `P₃ − P₀`. New step.

## Step 2 — Side of the fillet

The centre lies inside the angle at `V` (Euclid IV.4, `REF-EUCLID-IV4`, as `DERIV-fillet-setback` step 1). `nL = σa⊥` points from the straight side towards `b`, and `n(0) = −σb⊥` satisfies `n(0) · a = |a⊥ · b| > 0`: it points from the curve towards the straight side (`REF-MATHWORLD-PERP-DOT`). With `σ = 0` the curve leaves along the straight side (no corner, or a spike): no fillet.

## Step 3 — The centre

A circle of radius `r` touches a line when its centre is at distance `r` from it, the radius perpendicular at the contact point (Euclid III.16 porism and III.18, `REF-EUCLID-III16`, `REF-EUCLID-III18`): `(D − V) · nL = r`, the parallel line. It touches the curve at `B(t)` when `D = B(t) + r n(t)`, its radius on the curve's normal (same propositions): the parallel curve `x_d(t) = x(t) + d n(t)` (`REF-WIKI-PARALLEL-CURVE`; `REF-POMAX-BEZIER`, "Curve offsetting"). The centre is where the two offsets meet.

## Step 4 — The radius reached at `t`

Putting step 3's curve condition into the line condition: `(B(t) − V) · nL + r n(t) · nL = r`, so

`r(t) = (B(t) − V) · nL / (1 − n(t) · nL)`.

New step (algebra). For unit vectors `1 − u · v = ‖u − v‖² / 2` (`REF-MATHWORLD-DOT`: `‖u‖² = u · u`): the denominator is computed so, without cancellation when the corner is nearly flat.

## Step 5 — A straight side written as a cubic gives today's fillet

If `B(t) − V = λb`, then `(λb) · nL = λ sin θ` and `n · nL = −cos θ`, so `r = λ sin θ / (1 + cos θ) = λ tan(θ/2)` (`REF-WIKI-TAN-HALF-ANGLE`, with η = 0): `λ = r / tan(θ/2)`, the setback of `DERIV-fillet-setback`.

## Step 6 — Where the radius stops growing

With the signed curvature `k = det(B′, B″) / ‖B′‖³` and the normal `N` turning by `N′ = −k B′` (`REF-WIKI-CURVATURE`, signed curvature and Frenet–Serret in the plane), let `κ = −σk` be the curvature towards the fillet. The quotient rule gives

`r′(t) = ‖B′‖ (T · nL) (1 − r(t) κ(t)) / (1 − n · nL)`.

So `r(t)` grows while (i) the curve moves away from the straight side, `T · nL > 0`, and (ii) its radius of curvature on the fillet's side is larger than the radius reached, `r(t) κ < 1` — an offset has a cusp where the distance equals the radius of curvature (`REF-WIKI-PARALLEL-CURVE`). Near a corner whose handle is off the curvature is unbounded (step 1: `B′` vanishes faster than `det(B′, B″)`), while `r(t)` tends to 0: (ii) holds there. New step. The code does not evaluate the curvature: steps 7 and 11 test the circle itself.

## Step 7 — Solving for `t`

`r(0⁺) = 0`: the centre starts inside the other side's distance, `distance − r = r(cos τ − 1) < 0` (the numerator tends to 0, the denominator to `1 + cos θ > 0`). The centre runs along the parallel curve of the curved side — measured from the straight side when there is one, so that its distance needs no search — and every parameter where it reaches the distance `r` from the other side is a candidate (`REF-WIKI-IVT`, version I): a coarse table of 32 samples finds the brackets, a bisection inside each the point (`REF-MATHWORLD-BISECTION`, `REF-WIKI-BISECTION`) — Pomax's coarse lookup table followed by a binary search ("Projecting a point onto a Bézier curve"). The candidate drawn is the one closest to the corner whose circle is valid (step 11), as Inkscape's fillet spec selects the solution "closest … to the cusp along the length of bezier" (`REF-INKSCAPE-FILLET-SPEC`). The foot on a curved side is found the same way, the first place where the point stops lying ahead along the tangent.

## Step 8 — Tangent points and arc

`Tc = B(t*)`, `D = Tc + r n(t*)`, `Tl = D − r nL`, and the setback on the straight side `s = (D − V) · a` (`REF-MATHWORLD-DOT`). The arc turns from `a` to the curve's tangent at `t*`, by less than π while (i) holds: `large-arc = 0`, the sweep as in `DERIV-fillet-arc` step 5.

## Step 9 — The curve cut at the tangent point

De Casteljau's recurrence `βᵢ⁽ʲ⁾ = βᵢ⁽ʲ⁻¹⁾(1 − t₀) + βᵢ₊₁⁽ʲ⁻¹⁾ t₀` splits a curve at `t₀` into two with control points `β₀⁽⁰⁾ … β₀⁽ⁿ⁾` and `β₀⁽ⁿ⁾, β₁⁽ⁿ⁻¹⁾, …, βₙ⁽⁰⁾` (`REF-WIKI-DE-CASTELJAU`), which "can be overlaid exactly on the original curve" (`REF-POMAX-BEZIER`, "Splitting curves"). The piece kept is the second one from `Tc`; for a curve coming into `V`, the original is split at `1 − t*` and the first piece kept. `splitCubic` at `t₀ = ½` halves the curve (`DERIV-cubic-sides` step 4).

## Step 10 — Between two cubic sides

Both sides written from `V` as `A` and `B`; `σ = sign(T_A(0)⊥ · T_B(0))`, `n_A = σT_A⊥`, `n_B = −σT_B⊥`. The centre is `D = B(t_b) + r n_B(t_b)` at distance `r` from `A`, measured at the foot where `(D − A(t_a)) · A′(t_a) = 0` — the radius perpendicular to the tangent (Euclid III.18) — found by binary search (`REF-POMAX-BEZIER`, projecting a point). An outer bisection on `t_b` finds where `(D − A(t_a)) · n_A(t_a) = r`. With `A` straight it is steps 3 to 8. New step. Today a cubic has at most one end in radius mode, so this case arises only at a corner between two curved sides.

## Step 11 — A radius too large (ADR-0047)

A cubic has at most one filleted end: the point at its other end has handles on, and is drawn sharp. So a fillet may use the whole curve, as a single rounded corner may use a whole straight side (ADR-0007). A radius fits when its tangent points lie within both sides' limits and its circle stays clear of both sides — no sample of either side inside it, before or after the tangent point — so that the arc never crosses the curve. A radius that does not fit is reduced to the largest that does: a table of 16 radii descending from the one asked finds the first that fits, then a bisection between it and the one above keeps the last radius seen to fit, so that the radius drawn always has its arc (`REF-ASTUTE-CORNERS`: "its radius will be made as large as possible. Corners cannot extend past the next anchor point"). On a straight side shared with a reduced factor, the fillet may reach that share of its setback (ADR-0007); otherwise the whole side.

On the rectangle 120 × 80 with point 3's handles along the diagonal, a radius 60 at point 2 becomes 47.74907 with handles of 40, and 46.86292 with handles of 80, where its circle touches the curve at point 3 itself.

## Checks (test cases)

| Case                              | Input                                                                                                                | Expected                                                                                                                           |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| straight cubic, handles at thirds | `V` (100, 0), straight side from (0, 0); cubic (100, 0) (100, 50/3) (100, 100/3) (100, 50); `r` 10                   | `t*` 0.2, `Tc` (100, 10), `D` (90, 10), `Tl` (90, 0), `s` 10                                                                       |
| straight cubic, handle off at `V` | cubic (100, 0) (100, 0) (100, 100/3) (100, 50); `r` 10                                                               | same `Tc`, `D`, `Tl`; `t*` root of `t²(2 − t) = 0.2`, 0.347938                                                                     |
| equilateral                       | straight cubic at thirds from (100, 0) to (50, 50√3); `r` 10                                                         | `s` = 10√3 = 17.32051, `Tc` (91.33975, 15), `D` (82.67949, 10)                                                                     |
| quarter circle as a cubic         | `K = 400(√2 − 1)/3`; cubic (0, 0) (0, K) (K − 100, 100) (−100, 100); straight side from (−200, 0); `r` = 100(√2 − 1) | `t*` = ½, `Tc` (50√2 − 100, 50√2), `D` (100√2 − 200, 100√2 − 100)                                                                  |
| two straight cubics               | from (100, 0) to (0, 0) and to (100, 50), at thirds; `r` 10                                                          | `t_a` 0.1, `t_b` 0.2, `D` (90, 10)                                                                                                 |
| the Product Owner's rectangle     | 120 × 80, radius 20; point 3 in handles mode, handle after `(−1/√2, 1/√2)`, before its opposite                      | corner 2: `D` (100.15277, 20), `Tc` (120.15232, 19.86518); corner 4: `D` (20, 60.10582), `Tc` (19.90229, 80.10558); both radius 20 |
| degenerate                        | `σ = 0`; `r` 0; a side of length 0                                                                                   | no arc, no `NaN`                                                                                                                   |

The rectangle's centres were checked independently: on the line `y = 20`, the point whose distance to 200 000 samples of the curve is 20 has `x` = 100.152770476, as the method.
