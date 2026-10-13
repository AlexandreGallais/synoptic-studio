# DERIV-handle-tension — A point's handles from a tension, as a cardinal spline

Used by: F12 (US-026, research note 0013 method 7). Sources: `REF-WIKI-HERMITE` (the cardinal spline's tangents and the Bézier form of a Hermite segment), `REF-INKSCAPE-NODE-TYPES` (auto-smooth nodes, whose handles follow their neighbours).

## Notation

| Symbol          | Definition                                              |
| --------------- | ------------------------------------------------------- |
| `P₋`, `P`, `P₊` | the previous point, the point, the next point, as drawn |
| `c`             | the tension, from 0 to 1 (typed as a whole percentage)  |
| `m`             | the tangent at `P`                                      |

## Step 1 — The tangent of a cardinal spline

With points spaced one unit apart, the cardinal spline's tangent at a point is `m = (1 − c)(P₊ − P₋) / 2` (`REF-WIKI-HERMITE`: `m_k = (1 − c)(p_{k+1} − p_{k−1}) / (x_{k+1} − x_{k−1})`, the spacing 2). `c = 0` is the Catmull–Rom spline, `c = 1` gives zero tangents.

## Step 2 — From the tangent to the handles

A Hermite segment from `p₀` to `p₁` with tangents `m₀`, `m₁` is the cubic Bézier `p₀, p₀ + m₀/3, p₁ − m₁/3, p₁` (`REF-WIKI-HERMITE`, Representations). So the handle after `P` is `m / 3` and the handle before is `−m / 3`: `after = (1 − c)(P₊ − P₋) / 6`, `before = −after` — on one line, of the same length, as Figma's mirrored angle and length.

## Step 3 — Following the neighbours

The handles are a function of the neighbours only: moving a neighbour moves them, as an auto-smooth node's handles adapt (`REF-INKSCAPE-NODE-TYPES`). They are computed from the points drawn, never stored. They commute with a stretch of the shape: both are linear in the points (`DERIV-free-shape-stretch` step 5).

## Checks (test cases)

| Case                                        | Expected                                                  |
| ------------------------------------------- | --------------------------------------------------------- |
| P₋ (0, 0), P (100, 0), P₊ (200, 100), c = 0 | after (33.33333, 16.66667), before (−33.33333, −16.66667) |
| same, c = 1                                 | (0, 0) and (0, 0)                                         |
| same, c = 0.5                               | after (16.66667, 8.33333)                                 |
