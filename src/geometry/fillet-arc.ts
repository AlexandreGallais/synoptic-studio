import { add, quarterTurn, scale, subtract, unit } from "../math";

import { filletSetback } from "./fillet-setback";
import { turningAngle } from "./turning-angle";

import type { Arc } from "./arc";
import type { CornerPoints } from "./corner-points";

/**
 * Fillet arc of a corner: tangent points, center and direction.
 *
 * Formulas: T_in = V − s · u_in, T_out = V + s · u_out, C = T_in + sign(τ) · r · u_in⊥, positive
 * when τ > 0, with s = r · tan(|τ| / 2). Degenerates to the vertex (start = end = V) when no arc
 * exists: aligned vertex, zero-length edge or radius 0. The turning angle must be in ]−π, π[: a
 * spike (τ = ±π) is outside the derivation; it is consumed by the clamp (Q16).
 *
 * @kind geometry
 * @param corner - vertex `V` with its previous and next vertices
 * @param radius - effective radius `r`, >= 0
 * @returns the arc from the incoming to the outgoing tangent point
 * @see DERIV-fillet-arc
 */
export function filletArc(corner: CornerPoints, radius: number): Arc {
  const { next, previous, vertex } = corner;
  const incoming = unit(subtract(vertex, previous));
  const normal = quarterTurn(incoming);
  const angle = turningAngle(previous, vertex, next);
  const setback = filletSetback(radius, angle);
  const outgoing = unit(subtract(next, vertex));
  const start = add(vertex, scale(incoming, -setback));

  return {
    center: add(start, scale(normal, Math.sign(angle) * radius)),
    end: add(vertex, scale(outgoing, setback)),
    kind: "arc",
    positive: angle > 0,
    radius,
    start,
  };
}
