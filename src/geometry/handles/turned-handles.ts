import { rotate, withoutNegativeZero } from "../../math";

import type { Handles } from "./handles";
import type { Vector } from "../../math";

/**
 * Handles of a point once its shape is turned: each vector turned by the angle, clockwise on
 * screen, as the point is (`DERIV-cubic-sides` step 5).
 *
 * Formula: h′ = R(θ) h.
 *
 * @kind geometry
 * @param handles - switch, mode and the two vectors, before the turn
 * @param direction - unit vector of the angle (`turnDirection`)
 * @returns both vectors turned by the angle, switch and mode kept
 * @see DERIV-cubic-sides
 */
export function turnedHandles(handles: Handles, direction: Vector): Handles {
  return {
    ...handles,
    after: withoutNegativeZero(rotate(handles.after, direction)),
    before: withoutNegativeZero(rotate(handles.before, direction)),
  };
}
