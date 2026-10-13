import type { Handles } from "./handles";
import type { Vector } from "../../math";

/**
 * Handles of a point once its shape is stretched: each handle scaled on each axis by the factor of
 * that axis, as its point is (`stretchToSize`), so that the curve follows (`DERIV-free-shape-stretch`
 * step 5).
 *
 * Formula: (hₓ, h_y) → (fₓ · hₓ, f_y · h_y), for both handles; switch and mode unchanged.
 *
 * @kind geometry
 * @param handles - handles in the frame of the stored points
 * @param factors - width drawn over width stored, and the same for the height
 * @returns the handles as drawn
 * @see DERIV-free-shape-stretch
 */
export function stretchedHandles(handles: Handles, factors: Vector): Handles {
  return {
    ...handles,
    after: { x: factors.x * handles.after.x, y: factors.y * handles.after.y },
    before: { x: factors.x * handles.before.x, y: factors.y * handles.before.y },
  };
}
