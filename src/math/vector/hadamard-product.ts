import type { Vector } from "./vector";

/**
 * Product of two vectors component by component: a vector scaled by a different factor on each
 * axis, as a stretch or the circle made an ellipse.
 *
 * Formula: v ⊙ f = (vₓ fₓ, v_y f_y).
 *
 * @kind math
 * @param vector - vector to scale
 * @param factors - factor of each axis
 * @returns the scaled vector
 * @see REF-WIKI-HADAMARD
 */
export function hadamardProduct(vector: Vector, factors: Vector): Vector {
  return { x: factors.x * vector.x, y: factors.y * vector.y };
}
