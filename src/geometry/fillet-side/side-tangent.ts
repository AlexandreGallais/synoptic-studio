import { isZeroVector, unit } from "../../math";
import { cubicStartDirection, cubicVelocity } from "../curve";

import type { Vector } from "../../math";
import type { Cubic } from "../contour";

/**
 * The unit tangent of a side at a parameter: its velocity's direction, or its direction at the
 * corner where the velocity vanishes, a handle off (`DERIV-fillet-beside-curve` step 1).
 *
 * @kind geometry
 * @param cubic - the side, written from the corner
 * @param t - the parameter, from 0 to 1
 * @returns the unit tangent at t
 * @see DERIV-fillet-beside-curve
 */
export function sideTangent(cubic: Cubic, t: number): Vector {
  const velocity = cubicVelocity(cubic, t);

  return isZeroVector(velocity) ? cubicStartDirection(cubic) : unit(velocity);
}
