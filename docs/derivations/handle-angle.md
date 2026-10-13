# DERIV-handle-angle — A handle from an angle and a length, from the x axis or from its side

Used by: F12 (US-024, research note 0013 methods 3 and 4). Sources: `REF-AUTOCAD-POLAR` (a point typed as a distance and an angle from the last point), `REF-MATHWORLD-ROTATION-MATRIX` (rotation of a vector), `REF-MDN-ATAN2` (the angle of a vector), `DERIV-turned-shape` (angles clockwise on screen, from 0 to 359).

## Notation

| Symbol   | Definition                                                                                   |
| -------- | -------------------------------------------------------------------------------------------- |
| `θ`, `L` | the angle typed, degrees, clockwise on screen (y down), and the length, ≥ 0                  |
| `d(θ)`   | `turnDirection(θ)`: the unit vector at θ, `(cos θ, sin θ)` on screen                         |
| `u`      | the unit direction of the side the handle leaves along, from its point towards the other end |
| `R(θ)v`  | `v` turned by θ: `(vₓ cos θ − v_y sin θ, vₓ sin θ + v_y cos θ)`                              |

## Step 1 — From the x axis

A point typed as a distance and an angle from the last point (`REF-AUTOCAD-POLAR`) is the point plus `L` times the unit vector at that angle. The handle is that step: `h = L · d(θ)`. AutoCAD counts angles counterclockwise with y up; on screen with y down, the same formula counts them clockwise, as every angle of the project (`DERIV-turned-shape`).

Check: θ = 330°, L = 30: `h = (25.98076, −15)`; from (100, 0) the end is at (125.98076, −15).

## Step 2 — From the side

The handle after a point leaves along its side: its angle is counted from `u` instead of the x axis, `h = L · R(θ) u` (`REF-MATHWORLD-ROTATION-MATRIX`; on screen R turns clockwise, as `turnAbout`). On the side from (100, 0) to (200, 0), `u = (1, 0)` and step 2 is step 1; on the side from (100, 0) to (100, 100), `u = (0, 1)` and θ = 330°, L = 30 give `h = (15, 25.98076)`: 30° from the side, on its left as one travels down it — the outside of a shape drawn clockwise.

## Step 3 — The handle before, mirrored

The handle before a point leaves along the side towards the previous point, `u = unit(previous − point)`; with the same θ it would turn the other way round the side and bend it into an S. The Product Owner wants the same values at both ends to give a symmetric arch (2026-10-13), so the handle before is mirrored: `h = L · R(−θ) u`. Check on the side from (100, 0) to (200, 0): after A with θ = 330° is (25.98, −15); before B, `u = (−1, 0)`, `R(−330°)(−1, 0) = (−0.86603, −0.5)`, times 30 is (−25.98, −15): both above the side.

## Step 4 — Reading a handle back

The angle of a vector from the x axis is `atan2(h_y, hₓ)` (`REF-MDN-ATAN2`), counterclockwise with y up, so clockwise on screen; converted to degrees and brought into 0 to 359. From the side: `u × h = |u| |h| sin θ` (`REF-MATHWORLD-PERP-DOT`) and `u · h = |u| |h| cos θ` (`REF-MATHWORLD-DOT`), so the angle between `u` and `h` is `atan2(u × h, u · h)`, negated for the handle before (step 3). The length is `|h|`. A handle of length 0 has no angle: it reads 0.

## Checks (test cases)

| Case                                             | Expected         |
| ------------------------------------------------ | ---------------- |
| x axis, 330°, 30                                 | (25.98076, −15)  |
| x axis, 90°, 10                                  | (0, 10)          |
| side (1, 0), after, 330°, 30                     | (25.98076, −15)  |
| side (0, 1), after, 330°, 30                     | (15, 25.98076)   |
| side (−1, 0), before, 330°, 30                   | (−25.98076, −15) |
| read back (15, 25.98076) from side (0, 1), after | 330°, 30         |
