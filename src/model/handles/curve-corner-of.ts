import { NO_HANDLES } from "./no-handles";

import type { Corner, CurveCorner } from "../../geometry";

/**
 * A corner of a free shape made from a corner without handles: its point and radius, its handles
 * off (a rectangle or a polygon converted, a point added).
 *
 * @kind domain
 * @param corner - point and requested radius
 * @returns the same corner with handles off
 * @see docs/domain/shapes.md
 */
export function curveCornerOf(corner: Corner): CurveCorner {
  return { handles: NO_HANDLES, point: corner.point, radius: corner.radius };
}
