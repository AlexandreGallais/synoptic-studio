import type { Point } from "../math";

/**
 * Mirror image of a point about the vertical axis `x = 0`: `(x, y) ↦ (−x, y)`, the left half of a
 * stretched polygon from its right half (`DERIV-stretched-polygon` step 1).
 *
 * @kind geometry
 * @param point - point of the right half
 * @returns its mirror image
 * @see DERIV-stretched-polygon
 */
export function mirrorAcrossAxis(point: Point): Point {
  return { x: -point.x, y: point.y };
}
