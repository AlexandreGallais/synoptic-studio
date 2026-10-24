import { isBetween } from "../../math";

import type { BoundingBox } from "./bounding-box";
import type { Point } from "../../math";

/**
 * Whether a point lies in an upright box, its sides included: each coordinate in the box's
 * interval on its axis (`DERIV-box-overlap` step 1).
 *
 * Formula: minX ≤ x ≤ maxX and minY ≤ y ≤ maxY.
 *
 * @kind geometry
 * @param box - its extreme coordinates
 * @param point - the point asked
 * @returns true when the point is in the box or on its outline
 * @see REF-WIKI-BOUNDING-BOX
 */
export function isPointInBox(box: BoundingBox, point: Point): boolean {
  return isBetween(point.x, box.minX, box.maxX) && isBetween(point.y, box.minY, box.maxY);
}
