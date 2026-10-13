# DERIV-free-shape-stretch — Stretching a free shape by its width and height, without loss

Used by: ADR-0032, F09; ADR-0033, F08 (fractional points and size); ADR-0046, F12 (handles, step 5). Sources: `REF-SVG2-COORDS` §8.2 (the viewBox transform with `align = none`: one scale per axis, then a translation), `REF-CSS-TRANSFORMS` §3 (a transform about an origin). `fitInBox` (`DERIV-regular-polygon-fit`) shares the scale, with one factor for both axes, but not the translation: it places the box by its anchor, while this map keeps the top-left fixed.

## Notation

| Symbol          | Definition                                                                   |
| --------------- | ---------------------------------------------------------------------------- |
| `p₀ … pₙ₋₁`     | the points of the free shape, as stored, in contour order (Q11)              |
| `o = (oₓ, o_y)` | top-left of their bounding box: the smallest x and the smallest y            |
| `nₓ, n_y`       | width and height of that box: largest minus smallest coordinate on each axis |
| `w, h`          | width and height asked for the shape, integers ≥ 0                           |
| `p'ᵢ`           | the point drawn                                                              |

## Step 1 — A scale about a fixed point

SVG maps a box onto another with one scale per axis when `align` is `none`: scale-x = target width / box width, scale-y likewise, then a translation (`REF-SVG2-COORDS` §8.2). CSS builds a transform about an origin by translating by the origin, applying the transform, and translating back (`REF-CSS-TRANSFORMS` §3). With the origin `o`:

`p' = o + (sx · (pₓ − oₓ), sy · (p_y − o_y))`

## Step 2 — The factors that give the asked size, the top-left fixed

`sx = w / nₓ`, `sy = h / n_y` when `nₓ, n_y > 0`. Computed as `p'ₓ = oₓ + w · (pₓ − oₓ) / nₓ`, multiplying first: with integers the product is exact below 2⁵³, and one correctly rounded division gives `w` exactly at the far edge (`(29 / 7) · 7` is 29.000000000000004, `(29 · 7) / 7` is 29). Likewise for y. The point `o` maps to itself, so the top-left of the box stays fixed (Product Owner, F09).

The drawn box has width `sx · nₓ = w` and height `sy · n_y = h`: the point at the smallest x maps to `oₓ`, the one at the largest to `oₓ + sx · nₓ = oₓ + w`, and a scale with `sx ≥ 0` keeps the order of the coordinates. Likewise for y. At its own size (`w = nₓ`) the formula gives back each point mathematically, but not always bit for bit in floating point (`o + w·(p − o)/w` may differ from `p` by an ulp): the code therefore does not compute that axis and draws it as stored, bit for bit, which keeps a converted polygon exactly where it was (ADR-0033); otherwise, with fractional inputs, a drawn point may differ from the exact one by an ulp (about 1e-16 of the size), far below the 5 decimals written (Q10). The far edge is exact while `w · nₓ` stays below 2⁵³; beyond (sizes near the largest safe integer, Q20), it can be off by a unit — accepted, as for the fit of F02.

Degenerate box: when `nₓ = 0` (every point on one vertical line), no factor gives a width (`w / 0` is infinite). Such a shape is refused: the stored points must span a box of positive width and height (ADR-0032). The rectangle tool never gives one (step 4).

## Step 3 — Without loss

The stored points never change: the drawn points are a function of the stored points and of `w, h` only. So `w = 0` draws every point at `oₓ`, and `w` back to its first value draws exactly the first points: nothing was lost (Product Owner, F09: stretching must never be destructive).

## Step 4 — The rectangle tool

The rectangle tool stores the unit square `(0, 0), (1, 0), (1, 1), (0, 1)`, top-left first, clockwise (Q11): `o = (0, 0)`, `nₓ = n_y = 1`, so `sx = w`, `sy = h`, and the points drawn are `(0, 0), (w, 0), (w, h), (0, h)` — the rectangle of F01. The products `1 · w`, `0 · w` are exact in floating point: the drawing is the same as before, to the last bit.

## Step 5 — Handles (F12, ADR-0046)

A handle is a vector from its point, `h = c − p` with `c` its control point (`REF-FIGMA-VECTOR-NETWORK`). An affine map acts on a Bézier curve through its control points (`REF-WIKI-BEZIER`), so the control point is drawn `c' = o + S(c − o)` with `S = diag(sx, sy)` (step 1). Then `c' − p' = S(c − o) − S(p − o) = S(c − p)`: the handle drawn is `h' = (sx · hₓ, sy · h_y)` — the factors of step 2, no translation. On an axis where `w = nₓ` the points are drawn as stored (step 3), so the factor taken is 1 (`stretchFactors`, `stretchedHandles`).

Check: the unit square drawn 200 × 100, a handle (0.1, −0.2) at (0, 0): drawn (20, −20).

## Checks (test cases)

| Case                                              | Computation                                   | Expected points drawn                                       |
| ------------------------------------------------- | --------------------------------------------- | ----------------------------------------------------------- |
| Rectangle tool, 100 × 50                          | `sx = 100`, `sy = 50`                         | (0, 0), (100, 0), (100, 50), (0, 50)                        |
| Rectangle tool, 0 × 50, then 100 × 50             | `sx = 0`, then `100`                          | all x at 0, then the line above                             |
| Points (1, 0), (2, 2), (0, 2) to 10 × 4           | `o = (0, 0)`, `n = 2 × 2`, `sx = 5`, `sy = 2` | (5, 0), (10, 4), (0, 4)                                     |
| Points (3, 3), (5, 3), (5, 7) to 4 × 8            | `o = (3, 3)`, `n = 2 × 4`, `sx = 2`, `sy = 2` | (3, 3), (7, 3), (7, 11): top-left kept                      |
| Points (0, 0), (7, 0), (7, 1) to 29 × 1           | `29 · 7 / 7`, not `29 / 7 · 7`                | (0, 0), (29, 0), (29, 1): the far edge exact                |
| Points (0, 0), (3, 0), (3, 2), (0, 2) to 100 × 50 | `sx = 100 / 3`, `sy = 25`                     | (0, 0), (100, 0), (100, 50), (0, 50): a rectangle stays one |
