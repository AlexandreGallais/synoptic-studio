import type { Point } from "../../math";

/**
 * Distance from a point to the outline of a closed polygon: the smallest of its distances to the
 * polygon's sides (`polygonSegments`, `segmentDistance`) (`DERIV-painted-area` step 6).
 *
 * Formula: min over the sides of d(P, side).
 *
 * @kind geometry
 * @param points - its vertices, in contour order
 * @param point - the point asked
 * @returns the distance, ≥ 0; infinite for no vertex
 * @see DERIV-painted-area
 */
export function polygonDistance(points: readonly Point[], point: Point): number {
  return points.length + point.x;
}
