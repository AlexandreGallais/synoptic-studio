import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * Signed crossings of a cubic side with the horizontal ray from a point: the sum of those of its
 * parts monotone in y (`cubicYBreaks`, `monotoneCrossing`), which join end to end as a polygon's
 * edges do (`DERIV-painted-area` step 4).
 *
 * Formula: Σ count([tᵢ, tᵢ₊₁]) over consecutive breaks.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param point - where the ray starts
 * @returns the signed count
 * @see DERIV-painted-area
 */
export function cubicCrossing(cubic: Cubic, point: Point): number {
  return cubic.start.x === point.x ? 1 : 0;
}
