# DERIV-local-radius-clamp — Local proportional reduction of corner radii

Used by: ADR-0007; unequal radii per vertex (F03, ADR-0029) need no new step: every quantity below is already per vertex. Source principle: `REF-CSS-BR` (proportional reduction factor).

## Notation

Closed contour of vertices `v₀ … vₙ₋₁` (indices modulo n).

| Symbol | Definition                                                                          |
| ------ | ----------------------------------------------------------------------------------- |
| `eᵢ`   | edge from `vᵢ` to `vᵢ₊₁`                                                            |
| `Lᵢ`   | length of `eᵢ`                                                                      |
| `τᵢ`   | turning angle at `vᵢ` (between incoming and outgoing directions), `\|τᵢ\| ∈ [0, π[` |
| `rᵢ`   | radius requested at `vᵢ`, integer ≥ 0                                               |
| `sᵢ`   | setback requested at `vᵢ`                                                           |

## Step 1 — Setback of a fillet

Angle between both edges: `θᵢ = π − |τᵢ|`.
Setback: `sᵢ = rᵢ / tan(θᵢ / 2) = rᵢ · tan(|τᵢ| / 2)` (`DERIV-fillet-setback`).

Limit cases:

- `τᵢ = 0` (aligned vertices) → `sᵢ = 0`, no arc.
- `|τᵢ| → π` (very sharp spike) → `sᵢ → ∞`, always bounded by step 2.

## Step 2 — Factor per edge

Total demand on the edge: `Sᵢ = sᵢ + sᵢ₊₁`.
Factor: `fᵢ = 1` if `Sᵢ ≤ Lᵢ`, otherwise `fᵢ = Lᵢ / Sᵢ`.

## Step 3 — Factor per vertex

`gᵢ = min(fᵢ₋₁, fᵢ)` (the vertex respects both its edges).
Effective radius: `r'ᵢ = gᵢ · rᵢ`. Effective setback: `s'ᵢ = gᵢ · sᵢ` (the setback is linear in `r` at a fixed angle).

## Proof of validity

For every edge `eᵢ`:
`s'ᵢ + s'ᵢ₊₁ = gᵢ sᵢ + gᵢ₊₁ sᵢ₊₁ ≤ fᵢ sᵢ + fᵢ sᵢ₊₁ = fᵢ Sᵢ ≤ Lᵢ`
because `gᵢ ≤ fᵢ` and `gᵢ₊₁ ≤ fᵢ`. The arcs of one edge never overlap → no "anti-corner".

## Checks (test cases)

| Case                                         | Computation                                                                             | Expected result                                                                                |
| -------------------------------------------- | --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| Square 100, one corner at 1000, others at 0  | `τ = π/2` → `s = 1000`; `f = 100/1000` on both edges                                    | `r' = 100`: quarter circle over the whole side                                                 |
| Edge 25, two right corners at 100            | `S = 200`, `f = 0.125`                                                                  | `r' = 12.5` each, meeting in the middle                                                        |
| Square 100, 4 corners at 50                  | `S = 100 = L`, `f = 1`                                                                  | `r' = 50`: circle                                                                              |
| Aligned vertices, `r = 30`                   | `s = 0`                                                                                 | no arc                                                                                         |
| Rectangle 100 × 20, radii 30, 10, 0, 0 (F03) | left edge: `S = 0 + 30 = 30 > 20`, `f = 2/3`; top: `S = 40 ≤ 100`; right: `S = 10 ≤ 20` | top-left `r' = 2/3 · 30 = 20`, top-right `r' = 10`                                             |
| Rectangle 100 × 30, radii 40, 20, 0, 0 (F03) | left: `S = 40 > 30`, `f = 3/4`; top: `S = 60 ≤ 100`; right: `S = 20 ≤ 30`               | top-left `r' = 30`, top-right `r' = 20`: each corner reduced by its own edges only             |
| Rectangle 20 × 100, radii 30, 10, 0, 0 (F03) | top: `S = 40 > 20`, `f = 1/2`; left: `S = 30 ≤ 100`; right: `S = 10 ≤ 100`              | top-left `r' = 15`, top-right `r' = 5`: one short edge shared in proportion                    |
| Rectangle 30 × 20, radii 30, 10, 0, 0 (F03)  | left: `S = 30 > 20`, `f = 2/3`; top: `S = 40 > 30`, `f = 3/4`; right: `S = 10 ≤ 20`     | top-left `r' = 20`, top-right `r' = 7.5`: 2.5 of the top edge stays unused (known limit below) |

## Known limits

- Not maximal: when `gᵢ` is imposed by the other edge, a margin may stay unused.
- Concave contours: an arc may in theory touch a non-adjacent edge. To be detected, not corrected, in v1.
