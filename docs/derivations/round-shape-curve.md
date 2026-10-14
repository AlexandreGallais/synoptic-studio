# DERIV-round-shape-curve — A circle or an ellipse as four cubic sides

Used by: F10 in E14 (EN-027, ADR-0048). Sources: `REF-POMAX-BEZIER` (a circular arc of angle θ as a cubic: handles `4/3 · tan(θ/4)` times the radius, 0.55228 for a quarter circle, its error about 0.00027 of the radius), `REF-WIKI-BEZIER` (a four-piece cubic approximates a circle with handles `4(√2 − 1)/3`; an affine transformation of a curve is that of its control points; the derivative), `REF-WIKI-TAN-HALF-ANGLE` (`tan ½θ`), `DERIV-cubic-sides` (a side from a point and its handles).

## Notation

| Symbol   | Definition                                                        |
| -------- | ----------------------------------------------------------------- |
| `w, h`   | width and height of the round shape's box, integers ≥ 0           |
| `a, b`   | the half axes, `w/2` and `h/2`; the centre of the box is `(a, b)` |
| `κ`      | the handle of a quarter circle of radius 1, `4(√2 − 1)/3`         |
| `P1..P4` | the shape's points: top, right, bottom, left, clockwise on screen |

## Step 1 — A quarter circle as a cubic

A circular arc of angle θ and radius `r` is approximated by the cubic whose handles leave its ends along the tangents, each of length `r · 4/3 · tan(θ/4)` (`REF-POMAX-BEZIER`). For a quarter, `θ = π/2`: `tan(π/8) = (1 − cos(π/4)) / sin(π/4) = √2 − 1` (`REF-WIKI-TAN-HALF-ANGLE`, with `η = 0`), so `κ = 4(√2 − 1)/3 ≈ 0.5522847498`, the value of `REF-WIKI-BEZIER`. The cubic from (1, 0) to (0, 1) with controls (1, κ) and (κ, 1) passes at `t = 1/2` through `((4 + 3κ)/8, (4 + 3κ)/8) = (√2/2, √2/2)`, on the circle; its largest distance from the centre is about 1.00027, near `t = 0.21` and `t = 0.79` (`REF-POMAX-BEZIER`: an error of about 0.00027 of the radius).

## Step 2 — The ellipse as the circle stretched

The ellipse filling the box is the unit circle scaled by `a` across and `b` down, then moved to `(a, b)`: an affine map. A cubic's image by an affine map is the cubic of the mapped control points (`REF-WIKI-BEZIER`). So the four cubics of step 1, their control points scaled, draw the ellipse with the same relative error: a point off the circle by a vector `(δx, δy)` is off the ellipse by `(a · δx, b · δy)`, at most about `0.00027 · max(a, b)`. In a square box it is the circle of step 1, of radius `a`.

## Step 3 — The points and their handles

On screen, `y` grows downwards and clockwise goes top, right, bottom, left (Q11):

| Point       | Position | Handle before | Handle after |
| ----------- | -------- | ------------- | ------------ |
| P1 (top)    | `(a, 0)` | `(−κa, 0)`    | `(κa, 0)`    |
| P2 (right)  | `(w, b)` | `(0, −κb)`    | `(0, κb)`    |
| P3 (bottom) | `(a, h)` | `(κa, 0)`     | `(−κa, 0)`   |
| P4 (left)   | `(0, b)` | `(0, κb)`     | `(0, −κb)`   |

At each point the two handles are opposite and of the same length: Mirror angle and length (`REF-FIGMA-HANDLE-MIRRORING`). Each side is the cubic of `DERIV-cubic-sides` step 1 from a point, its handle after, the next point's handle before, to the next point; with radii 0 no corner is filleted.

## Step 4 — The box

Each side's four control points move the same way along x and along y (from P1 to P2: x `a, a + κa, w, w`, y `0, 0, b − κb, b`), so the differences of its control points never change sign along an axis; the derivative is a Bernstein sum of those differences (`REF-WIKI-BEZIER`) and does not vanish inside the side. Its extremes are its ends, and the shape's straight box is exactly `0 ≤ x ≤ w`, `0 ≤ y ≤ h`: the ellipse touches the four sides of its box at its four points.

## Step 5 — Degenerate boxes

With `w = 0`, `a = 0`: P1 and P3 are at `(0, 0)` and `(0, h)`, the horizontal handles have length 0, and the vertical ones `κb`. Every side runs along `x = 0`: the shape draws the segment from `(0, 0)` to `(0, h)`, as a rectangle of width 0 does (Q15). With `w = h = 0` every point is at the origin.

## Checks (test cases)

| Case                   | Computation                          | Expected                            |
| ---------------------- | ------------------------------------ | ----------------------------------- |
| κ                      | `4(√2 − 1)/3`                        | 0.55228475                          |
| 100 × 100, handles     | `50κ`                                | 27.61424                            |
| 100 × 100, side 1 at ½ | `(50, 50) + 50 · (√2/2, −√2/2)`      | (85.35534, 14.64466), on the circle |
| 120 × 80, top handles  | `60κ`                                | 33.13708                            |
| 120 × 80, side handles | `40κ`                                | 22.09139                            |
| 120 × 80, side 1 at ½  | `(60, 40) + (60 · √2/2, −40 · √2/2)` | (102.42641, 11.71573)               |
| 100 × 100, largest gap | step 1, `50 × 0.00027`               | about 0.0136 from the circle        |
| 0 × 50                 | step 5                               | the segment from (0, 0) to (0, 50)  |
