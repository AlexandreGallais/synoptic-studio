import { dot, perpDot } from "../vector";

import type { Vector } from "../vector";

/**
 * Signed angle from one vector to another, from −π to π: positive when the second is clockwise
 * from the first on screen. The perp-dot product is ‖a‖‖b‖ sin θ and the dot product ‖a‖‖b‖ cos θ
 * (`DERIV-turning-angle`).
 *
 * Formula: θ = atan2(a ⊥· b, a · b).
 *
 * @kind math
 * @param from - first vector
 * @param to - second vector
 * @returns the angle in radians; with a zero vector, 0 or ±π by the signs of the zeros
 * @see DERIV-turning-angle
 */
export function signedAngle(from: Vector, to: Vector): number {
  return Math.atan2(perpDot(from, to), dot(from, to));
}
