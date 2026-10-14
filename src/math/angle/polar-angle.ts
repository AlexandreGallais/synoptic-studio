import type { Vector } from "../vector";

/**
 * Angle of a vector from the x axis, counterclockwise in the usual frame (clockwise on screen
 * where y points down), from −π to π.
 *
 * Formula: atan2(v_y, vₓ).
 *
 * @kind math
 * @param vector - any vector; (0, 0) gives 0 or ±π by the signs of its zeros
 * @returns the angle in radians
 * @see REF-MDN-ATAN2
 */
export function polarAngle(vector: Vector): number {
  return Math.atan2(vector.y, vector.x);
}
