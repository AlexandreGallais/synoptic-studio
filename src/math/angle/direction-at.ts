import type { Vector } from "../vector";

/**
 * Unit vector at an angle: the point of the unit circle where the angle's side meets it.
 *
 * Formula: (cos t, sin t).
 *
 * @kind math
 * @param angle - angle in radians, from the x axis
 * @returns the unit vector
 * @see REF-OPENSTAX-UNIT-CIRCLE
 */
export function directionAt(angle: number): Vector {
  return { x: Math.cos(angle), y: Math.sin(angle) };
}
