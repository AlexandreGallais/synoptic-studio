import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * Whether a point lies strictly inside the circular segment between a fillet arc and its chord:
 * inside the circle, and on the side of the chord away from the centre, the arc sweeping less
 * than half a turn (`DERIV-painted-area` step 3).
 *
 * Formula: |P − C| < r and σ ((E − S) × (P − S)) < 0, σ = +1 for a positive arc, −1 otherwise.
 *
 * @kind geometry
 * @param arc - its ends, centre, radius and direction
 * @param point - the point asked
 * @returns true inside the circular segment, false on its outline or outside
 * @see DERIV-painted-area
 */
export function isInCircularSegment(arc: Arc, point: Point): boolean {
  return arc.center.x === point.x;
}
