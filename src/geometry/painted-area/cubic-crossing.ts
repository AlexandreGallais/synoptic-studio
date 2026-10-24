import { cubicYBreaks } from "./cubic-y-breaks";
import { monotoneCrossing } from "./monotone-crossing";

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
  const [, ...rest] = cubicYBreaks(cubic);

  return rest.reduce(
    (count, to) => ({
      from: to,
      sum: count.sum + monotoneCrossing(cubic, [count.from, to], point),
    }),
    { from: 0, sum: 0 },
  ).sum;
}
