import type { Vector } from "./vector";

/**
 * Length of a vector (Euclidean, or L2, norm).
 *
 * Formula: ‖v‖ = √(vₓ² + v_y²).
 *
 * @kind math
 * @param vector - displacement
 * @returns its length, >= 0
 * @see REF-MATHWORLD-VECTOR-NORM
 */
export function norm(vector: Vector): number {
  return Math.hypot(vector.x, vector.y);
}
