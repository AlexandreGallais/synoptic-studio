# DERIV-fillet-setback — Setback of a corner fillet

Used by: EN-004, `DERIV-local-radius-clamp` step 1. Replaces the unverified `REF-GG-FILLET` with steps each backed by a verified source.

## Notation

| Symbol | Definition                                                    |
| ------ | ------------------------------------------------------------- |
| `B`    | corner vertex                                                 |
| `θ`    | interior angle at `B`, between the two edges                  |
| `τ`    | turning angle at `B` (`DERIV-turning-angle`), `θ = π − \|τ\|` |
| `r`    | fillet radius, `r ≥ 0`                                        |
| `D`    | center of the fillet                                          |
| `E`    | tangent point on one edge                                     |
| `s`    | setback: distance `BE` from the vertex to the tangent point   |

## Step 1 — Center on the bisector

Take `D` on the bisector of the angle at `B`, at distance `r` from one edge. Bisecting the angle gives two congruent right triangles, so the perpendiculars from `D` to both edges are equal (Euclid IV.4, `REF-EUCLID-IV4`): `D` is at distance `r` from both edges.

## Step 2 — Tangency

The line drawn at right angles to a radius from its end touches the circle (Euclid III.16, porism, `REF-EUCLID-III16`), so the circle of center `D` and radius `r` is tangent to both edges at the feet of the perpendiculars. Conversely the radius to the point of contact is perpendicular to the tangent (Euclid III.18, `REF-EUCLID-III18`): the triangle `BED` is right-angled at `E`.

## Step 3 — Angles of the triangle `BED`

- Angle at `B`: `θ / 2` (bisector).
- Angle at `E`: `π / 2` (step 2).
- Angle at `D`: `π − π/2 − θ/2 = (π − θ) / 2 = |τ| / 2`, since the angles of a triangle sum to two right angles (Euclid I.32, `REF-EUCLID-I32`).

## Step 4 — Setback

In the right triangle `BED`, the tangent of an angle is the opposite side over the adjacent side (`REF-MATHWORLD-TANGENT`). At `D`, the opposite side is `BE = s`, the adjacent side is `DE = r`:

`s = r · tan(|τ| / 2)`, equivalently `s = r / tan(θ / 2)` (same triangle, angle at `B`).

Cross-check: road design uses the same relation for the tangent length of a circular curve, `T = R tan(Δ/2)`, `Δ` being the supplement of the interior angle between the tangents, i.e. `|τ|` (`REF-WIKIBOOKS-HORIZONTAL-CURVES`, level-6 source, agreement only).

## Checks (test cases)

| `r` | `\|τ\|`                         | `s`                                  |
| --- | ------------------------------- | ------------------------------------ |
| 10  | `π/2` (right corner)            | `10 · tan(π/4) = 10`                 |
| 30  | `0` (aligned)                   | `0`: no arc                          |
| 10  | `π/3`                           | `10 · tan(π/6) = 10 / √3 ≈ 5.773503` |
| 0   | `π/2`                           | `0`: sharp corner                    |
| 10  | `−π/2` (counter-clockwise turn) | `10`: only `\|τ\|` matters           |

## Limits

- `|τ| → π` (very sharp spike): `s → ∞`; the clamp of `DERIV-local-radius-clamp` bounds it.
