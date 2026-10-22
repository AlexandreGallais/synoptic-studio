import { isInCircularSegment } from "./is-in-circular-segment";
import { segmentCrossing } from "./segment-crossing";

import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * Signed crossings of a fillet arc with the horizontal ray from a point: those of its chord
 * (`segmentCrossing`), plus the arc's direction when the point lies inside the circular segment
 * between them (`isInCircularSegment`), the arc and its chord travelled back closing a convex
 * loop that winds once around the points inside it (`DERIV-painted-area` step 3).
 *
 * Formula: count(S, E) + σ when P is inside the circular segment, count(S, E) otherwise.
 *
 * @kind geometry
 * @param arc - its ends, centre, radius and direction
 * @param point - where the ray starts
 * @returns the signed count, from −1 to +1
 * @see DERIV-painted-area
 */
export function arcCrossing(arc: Arc, point: Point): number {
  const chord = segmentCrossing(arc, point);

  if (!isInCircularSegment(arc, point)) {
    return chord;
  }

  return arc.positive ? chord + 1 : chord - 1;
}
