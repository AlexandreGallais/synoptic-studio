# DERIV-arc-bounding-box — The straight box of a contour of segments and fillet arcs

Used by: F04 (the straight box of a turned shape, EN-015). Sources: `REF-SVG2-COORDS` §8.10 (the bounding box is the tightest axis-aligned rectangle around the geometric shape, as `getBBox` returns it), `REF-OPENSTAX-UNIT-CIRCLE` (the point of the unit circle at angle `t` is `(cos t, sin t)`), `REF-MATHWORLD-PERP-DOT` (`a⊥·b = |a| |b| sin θ`).

## Notation

| Symbol | Definition |
| --- | --- |
| `C, r` | center and radius of a fillet arc |
| `S, E` | its start and end; `s = S − C`, `e = E − C` |
| `σ` | `+1` when the arc turns in the positive direction of angles (clockwise on screen), `−1` otherwise |
| `u × v` | the perp dot product `uₓ v_y − u_y vₓ` |
| `d` | one of the four axis directions `(1, 0)`, `(0, 1)`, `(−1, 0)`, `(0, −1)` |

## Step 1 — A segment

A segment lies between its ends: its box is the box of its two ends. A contour's box is the box of its pieces' boxes, so only the arcs need more.

## Step 2 — Where an arc reaches its extreme x and y

A point of the arc is `C + r (cos t, sin t)` (`REF-OPENSTAX-UNIT-CIRCLE`, scaled by `r`, moved to `C`). Along the unit circle `cos t` goes from 1 to −1 as `t` goes from 0 to π, and back from π to 2π: it only turns at multiples of π, where the point is `C ± r (1, 0)`. Likewise `sin t` only turns at `π/2 + kπ`, the points `C ± r (0, 1)`. So on an arc, the extreme x and y are at its ends or at the axis points `C + r d` the arc passes through.

## Step 3 — Whether the arc passes through an axis point

A fillet sweeps the turning angle of its corner, more than 0 and less than π (`DERIV-fillet-arc`); an arc without length (a corner that keeps no fillet, dropped by `hasLength`) has `s = e` and is counted by its ends only. The perp dot product has the sign of `sin θ`, `θ` the angle from one vector to the other (`REF-MATHWORLD-PERP-DOT`). For an arc of less than π, the direction `d` lies on it when it is reached from `s` and reaches `e` in the arc's direction:

`σ · (s × d) ≥ 0` and `σ · (d × e) ≥ 0`

Both angles are then in `[0, π]` in the arc's direction, and their sum is the sweep, so `d` lies between `s` and `e`. The signs are computed in the same frame as `σ` (y down), so no screen convention enters.

## Step 4 — The box

The box of a contour is the box of: the ends of every piece, and the axis points `C + r d` of every arc that passes through them (step 3). It is tight: every point counted is on the drawn outline, and no point of the outline lies outside (step 2), which is the box `getBBox` returns (`REF-SVG2-COORDS` §8.10). Strokes are not in it.

## Checks (test cases)

| Case | Points counted | Expected box |
| --- | --- | --- |
| Square 100 × 100, sharp, turned 45° about (50, 50) | the four corners | x and y from −20.71068 to 120.71068 |
| Square 100 × 100, radius 50 (a circle), any angle | the axis points of the four arcs | x and y from 0 to 100 |
| Square 100 × 100, radius 10, straight | arc ends only: each quarter arc ends on an axis | x and y from 0 to 100 |
| Square 100 × 100, radius 10, turned 45° | the arcs crossing the axes | x and y from 50 − (40√2 + 10) = −16.56854 to 116.56854 |
