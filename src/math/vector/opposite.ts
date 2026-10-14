import { scale } from "./scale";

import type { Vector } from "./vector";

/**
 * Opposite of a vector: the same length, the other way; A − B is A + (−B).
 *
 * Formula: −v = (−1) · v.
 *
 * @kind math
 * @param vector - any vector
 * @returns the opposite vector
 * @see REF-MATHWORLD-VECTOR-DIFFERENCE
 */
export function opposite(vector: Vector): Vector {
  return scale(vector, -1);
}
