import { ZERO_VECTOR, subtract, unit } from "../../math";

import type { Vector } from "../../math";
import type { Cubic } from "../contour";

/**
 * The unit tangent of a cubic at its start, even when its first handle is off: the first
 * non-zero of P₁ − P₀, P₂ − P₀, P₃ − P₀, the limit of B′(t)'s direction as t tends to 0
 * (`DERIV-fillet-beside-curve` step 1); (0, 0) for a cubic without length.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the direction it leaves its start along
 * @see DERIV-fillet-beside-curve
 */
export function cubicStartDirection(cubic: Cubic): Vector {
  const leaving = [cubic.first, cubic.second, cubic.end]
    .map((point) => subtract(point, cubic.start))
    .find((vector) => vector.x !== 0 || vector.y !== 0);

  return unit(leaving ?? ZERO_VECTOR);
}
