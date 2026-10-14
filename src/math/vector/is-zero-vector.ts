import type { Vector } from "./vector";

/**
 * Whether a vector is the zero vector: both components 0, so that it has no direction.
 *
 * Formula: v = 0 ⇔ vₓ = 0 and v_y = 0.
 *
 * @kind math
 * @param vector - any vector
 * @returns `true` for (0, 0), signs of zero aside
 * @see REF-OPENSTAX-CALC3-VECTORS
 */
export function isZeroVector(vector: Vector): boolean {
  return vector.x === 0 && vector.y === 0;
}
