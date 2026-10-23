import type { Point } from "../../math";

/**
 * A point with its axes swapped, the map (x, y) ↦ (y, x) that turns a box's vertical sides into
 * horizontal ones (`DERIV-box-overlap` step 8).
 *
 * Formula: (x, y) ↦ (y, x).
 *
 * @kind geometry
 * @param point - any point of the plane
 * @returns its image, x and y exchanged
 * @see DERIV-box-overlap
 */
export function transposedPoint(point: Point): Point {
  return point;
}
