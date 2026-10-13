# DERIV-handle-share — A handle as shares of its side, and CSS's cubic-bezier() laid on a side

Used by: F12 (US-025, research note 0013 methods 5 and 6). Sources: `REF-CSS-EASING` §2.2 (`cubic-bezier(x1, y1, x2, y2)`: the control points of a cubic from (0, 0) to (1, 1)), `REF-MATHWORLD-ROTATION-MATRIX` (a quarter turn), `REF-MATHWORLD-DOT` (projection on a direction).

## Notation

| Symbol   | Definition                                                                                                      |
| -------- | --------------------------------------------------------------------------------------------------------------- |
| `s`      | the handle's side, from its point to the other end: B − A for the handle after A, A − B for the handle before B |
| `n`      | `s` turned a quarter turn: `(s_y, −sₓ)` for a handle after, `(−s_y, sₓ)` for a handle before (step 2)           |
| `a`, `c` | the shares along and across the side, as fractions (25 % is 0.25)                                               |

## Step 1 — Shares of the side

A handle as a share `a` of its side and a share `c` across it: `h = a · s + c · n`, with `|n| = |s|`, so that 100 % across is as long as the side. The same shares give the same bend on any side, whatever its direction and length: the map from the unit square to the side is a similarity.

Check: the side from (100, 0) to (200, 0), handle after A, 25 % along and 30 % across: `s = (100, 0)`, `n = (0, −100)`, `h = (25, −30)`, its end at (125, −30) — above the side, the outside of a shape drawn clockwise.

## Step 2 — The handle before, mirrored

For the handle before B, `s = A − B` runs the other way; turning it the same way would put "across" below the side. The Product Owner wants equal values at both ends to make a symmetric arch (2026-10-13), so the handle before turns `s` the other way: `n = (−s_y, sₓ)`. Check: before B (200, 0), `s = (−100, 0)`, `n = (0, −100)`: 25 % along and 30 % across end at (175, −30), the mirror of (125, −30) about x = 150.

## Step 3 — Reading shares back

`n` is perpendicular to `s` and as long, so `a = (h · s) / |s|²` and `c = (h · n) / |s|²` (`REF-MATHWORLD-DOT`). A side of length 0 has no shares: refused.

## Step 4 — CSS's cubic-bezier() on a side

`cubic-bezier(x1, y1, x2, y2)` places a cubic's control points between ends fixed at (0, 0) and (1, 1), x values from 0 to 1 (`REF-CSS-EASING`). Laying that diagonal on the side from A to B sends (0, 0) to A and (1, 1) to B: a point (x, y) of the unit square goes `(x + y) / 2` along the side and `(y − x) / 2` across it, to the left of travel — the n of a handle after (step 1). So the first control point is `A + ((x1 + y1)/2) s + ((y1 − x1)/2) n` and the second `A + ((x2 + y2)/2) s + ((y2 − x2)/2) n`, with `s = B − A`.

Check: `cubic-bezier(0.42, 0, 0.58, 1)` on (100, 0) → (200, 0): first (121, 21), second (179, −21): the S of the easing, starting below the side.

## Checks (test cases)

| Case                                                  | Expected              |
| ----------------------------------------------------- | --------------------- |
| after A (100, 0), side (100, 0), 25 %, 30 %           | (25, −30)             |
| before B (200, 0), side (−100, 0), 25 %, 30 %         | (−25, −30)            |
| read back (25, −30) after, side (100, 0)              | 25 %, 30 %            |
| cubic-bezier(0.42, 0, 0.58, 1) on (100, 0) → (200, 0) | (121, 21), (179, −21) |
