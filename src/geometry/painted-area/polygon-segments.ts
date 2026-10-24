import { cyclicItem } from "../contour";

import type { Point } from "../../math";
import type { Segment } from "../contour";

/**
 * Sides of a closed polygon as segments, each from a point to the next, the last back to the
 * first (Q11); a side of length 0 is kept.
 *
 * @kind geometry
 * @param points - its vertices, in contour order
 * @returns one segment per vertex, in the same order; none for no vertex
 * @see DERIV-painted-area
 */
export function polygonSegments(points: readonly Point[]): readonly Segment[] {
  return points.map((start, index) => ({
    end: cyclicItem(points, index + 1, start),
    kind: "segment",
    start,
  }));
}
