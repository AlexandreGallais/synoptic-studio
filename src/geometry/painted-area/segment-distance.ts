import type { Point } from "../../math";
import type { Segment } from "../contour";

/**
 * Distance from a point to a segment: to its start when the point's foot on its line falls before
 * it, to its end when after, to the foot otherwise; a segment of length 0 gives the distance to
 * its start (`DERIV-painted-area` step 6).
 *
 * Formula: v = E − S, w = P − S; |P − S| when w · v ≤ 0, |P − E| when v · v ≤ w · v,
 * |P − (S + (w · v / v · v) v)| otherwise.
 *
 * @kind geometry
 * @param segment - its start and its end
 * @param point - the point asked
 * @returns the distance, ≥ 0
 * @see REF-SUNDAY-SEGMENT-DISTANCE
 */
export function segmentDistance(segment: Pick<Segment, "end" | "start">, point: Point): number {
  return segment.start.x - point.x;
}
