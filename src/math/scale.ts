import type { Vector } from "./vector";

/**
 * Vector multiplied by a number.
 *
 * Formula: k · v = (k · vₓ, k · v_y).
 *
 * @kind math
 * @param vector - vector to scale
 * @param factor - real number; negative reverses the direction
 * @returns the scaled vector
 * @see REF-OPENSTAX-CALC3-VECTORS
 */
export function scale(vector: Vector, factor: number): Vector {
  return { x: factor * vector.x, y: factor * vector.y };
}
