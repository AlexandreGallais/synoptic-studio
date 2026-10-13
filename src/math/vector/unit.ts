import { norm } from "./norm";

import type { Vector } from "./vector";

/**
 * Vector of length 1 with the direction of a vector.
 *
 * Formula: v̂ = v / ‖v‖, defined for a nonzero vector; the zero vector (an edge of length 0, Q15)
 * has no direction and gives (0, 0), never a division by zero (convention of `DERIV-fillet-arc`
 * step 1; the cited definition covers nonzero vectors only).
 *
 * @kind math
 * @param vector - direction to normalize
 * @returns the unit vector, or (0, 0) for the zero vector
 * @see REF-MATHWORLD-UNIT-VECTOR
 */
export function unit(vector: Vector): Vector {
  const length = norm(vector);

  return length === 0 ? { x: 0, y: 0 } : { x: vector.x / length, y: vector.y / length };
}
