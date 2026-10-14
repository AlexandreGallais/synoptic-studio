import type { Point } from "./point";
import type { Vector } from "./vector";

/**
 * Vector from `b` to `a`, component-wise.
 *
 * Formula: a − b = (aₓ − bₓ, a_y − b_y).
 *
 * @kind math
 * @param a - end point
 * @param b - start point
 * @returns the displacement from `b` to `a`
 * @see REF-MATHWORLD-VECTOR-DIFFERENCE
 */
export function subtract(a: Point, b: Point): Vector {
  return { x: a.x - b.x, y: a.y - b.y };
}
