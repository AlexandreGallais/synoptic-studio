import { perpDot, subtract } from "../../math";

import { isLevelCrossed } from "./is-level-crossed";

import type { Point } from "../../math";
import type { Segment } from "../contour";

/**
 * Signed crossing of a segment with the horizontal ray from a point toward growing x: +1 when the
 * segment crosses it with y growing, −1 with y falling, 0 when it misses it, by the edge
 * crossing rules (`isLevelCrossed`) and a crossing strictly right of the point, read from the side
 * of the segment the point lies on (`DERIV-painted-area` step 2).
 *
 * Formula: d = sign(E_y − S_y); d when min(S_y, E_y) ≤ p_y < max(S_y, E_y) and
 * sign((E − S) × (P − S)) = d, else 0.
 *
 * @kind geometry
 * @param segment - its start and its end
 * @param point - where the ray starts
 * @returns +1, −1 or 0
 * @see REF-SUNDAY-INCLUSION
 */
export function segmentCrossing(segment: Pick<Segment, "end" | "start">, point: Point): number {
  const { end, start } = segment;

  if (!isLevelCrossed(start.y, end.y, point.y)) {
    return 0;
  }

  const direction = Math.sign(end.y - start.y);
  const side = perpDot(subtract(end, start), subtract(point, start));

  return Math.sign(side) === direction ? direction : 0;
}
