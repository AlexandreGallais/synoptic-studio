import type { Vector } from "./vector";

/**
 * Vector turned by a quarter turn in the positive direction of angles.
 *
 * Formula: (x, y)⊥ = (−y, x), the rotation matrix with θ = π/2. In the SVG frame (y down), the
 * positive direction is clockwise on screen.
 *
 * @kind math
 * @param vector - vector to turn
 * @returns the turned vector, same length
 * @see REF-MATHWORLD-ROTATION-MATRIX
 */
export function quarterTurn(vector: Vector): Vector {
  return { x: -vector.y, y: vector.x };
}
