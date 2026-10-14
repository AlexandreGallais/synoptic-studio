import { average } from "../arithmetic";

import type { Point } from "../vector";

/**
 * Middle of a segment: the point dividing it into two segments of equal length.
 *
 * Formula: M = ((x₁ + x₂)/2, (y₁ + y₂)/2). Halving is exact in binary floating point, so the only
 * rounding is the one of each sum.
 *
 * @kind math
 * @param start - one end of the segment
 * @param end - the other end
 * @returns the middle of the segment
 * @see REF-MATHWORLD-MIDPOINT
 */
export function midpoint(start: Point, end: Point): Point {
  return { x: average(start.x, end.x), y: average(start.y, end.y) };
}
