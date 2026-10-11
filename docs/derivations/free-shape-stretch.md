# DERIV-free-shape-stretch — Stretching a free shape by its width and height, without loss

Used by: ADR-0032, F09. Sources: `REF-CSS-TRANSFORMS` (§3 transform rendering model, §9.1 `scale(sx, sy)`).

## Notation

| Symbol          | Definition                                                                   |
| --------------- | ---------------------------------------------------------------------------- |
| `p₀ … pₙ₋₁`     | the points of the free shape, as stored, in contour order (Q11)              |
| `o = (oₓ, o_y)` | top-left of their bounding box: the smallest x and the smallest y            |
| `nₓ, n_y`       | width and height of that box: largest minus smallest coordinate on each axis |
| `w, h`          | width and height asked for the shape, integers ≥ 0                           |
| `p'ᵢ`           | the point drawn                                                              |

## Step 1 — A scale about a fixed point

The transform rendering model of CSS (`REF-CSS-TRANSFORMS` §3) builds a transform about an origin in three steps: translate by the origin, apply the transform, translate by the negated origin. With the scale `scale(sx, sy)` (§9.1) and the origin `o`:

`p' = o + (sx · (pₓ − oₓ), sy · (p_y − o_y))`

## Step 2 — The factors that give the asked size, the top-left fixed

`sx = w / nₓ`, `sy = h / n_y` when `nₓ, n_y > 0`. The point `o` maps to itself, so the top-left of the box stays fixed (Product Owner, F09).

The drawn box has width `sx · nₓ = w` and height `sy · n_y = h`: the point at the smallest x maps to `oₓ`, the one at the largest to `oₓ + sx · nₓ = oₓ + w`, and a scale with `sx ≥ 0` keeps the order of the coordinates. Likewise for y.

Degenerate box: when `nₓ = 0` (every point on one vertical line), no factor gives a width; every point is drawn at `oₓ`. Same for `n_y`. The rectangle tool never gives such a box (step 4).

## Step 3 — Without loss

The stored points never change: the drawn points are a function of the stored points and of `w, h` only. So `w = 0` draws every point at `oₓ`, and `w` back to its first value draws exactly the first points: nothing was lost (Product Owner, F09: « il faudrait pas que ce soit destructif »).

## Step 4 — The rectangle tool

The rectangle tool stores the unit square `(0, 0), (1, 0), (1, 1), (0, 1)`, top-left first, clockwise (Q11): `o = (0, 0)`, `nₓ = n_y = 1`, so `sx = w`, `sy = h`, and the points drawn are `(0, 0), (w, 0), (w, h), (0, h)` — the rectangle of F01. The products `1 · w`, `0 · w` are exact in floating point: the drawing is the same as before, to the last bit.

## Checks (test cases)

| Case                                    | Computation                                   | Expected points drawn                  |
| --------------------------------------- | --------------------------------------------- | -------------------------------------- |
| Rectangle tool, 100 × 50                | `sx = 100`, `sy = 50`                         | (0, 0), (100, 0), (100, 50), (0, 50)   |
| Rectangle tool, 0 × 50, then 100 × 50   | `sx = 0`, then `100`                          | all x at 0, then the line above        |
| Points (1, 0), (2, 2), (0, 2) to 10 × 4 | `o = (0, 0)`, `n = 2 × 2`, `sx = 5`, `sy = 2` | (5, 0), (10, 4), (0, 4)                |
| Points (3, 3), (5, 3), (5, 7) to 4 × 8  | `o = (3, 3)`, `n = 2 × 4`, `sx = 2`, `sy = 2` | (3, 3), (7, 3), (7, 11): top-left kept |
