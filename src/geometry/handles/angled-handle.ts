import { ZERO_VECTOR, add, scale, turnDirection } from "../../math";
import { turnAbout } from "../placement";

import type { Vector } from "../../math";

/**
 * A handle from an angle and a length counted from a direction: the direction turned by the angle,
 * clockwise on screen, scaled to the length (`DERIV-handle-angle` steps 1 and 2).
 *
 * Formula: h = L · R(θ) u; with u = (1, 0), h = L · (cos θ, sin θ).
 *
 * @kind geometry
 * @param direction - unit vector the angle is counted from: the x axis, or the handle's side
 * @param angle - degrees, clockwise on screen
 * @param length - the handle's length, ≥ 0
 * @returns the handle, a vector from its point
 * @see DERIV-handle-angle
 */
export function angledHandle(direction: Vector, angle: number, length: number): Vector {
  const turned = turnAbout(direction, ZERO_VECTOR, turnDirection(angle));

  // Adding zero turns −0 into 0: a handle of length 0 is (0, 0), as a new one.
  return add(scale(turned, length), ZERO_VECTOR);
}
