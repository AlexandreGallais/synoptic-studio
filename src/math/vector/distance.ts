import { norm } from "./norm";
import { subtract } from "./subtract";

import type { Point } from "./point";

/**
 * Distance between two points: the length of the segment joining them, by the Pythagorean
 * theorem.
 *
 * Formula: |AB| = ‖B − A‖ = √((x_B − x_A)² + (y_B − y_A)²).
 *
 * @kind math
 * @param from - one point
 * @param to - the other
 * @returns the distance, ≥ 0
 * @see REF-MATHWORLD-DISTANCE
 */
export function distance(from: Point, to: Point): number {
  return norm(subtract(to, from));
}
