import { turningAngle } from "./turning-angle";

import type { CornerPoints } from "./corner-points";

/**
 * Turning angle at every corner of a cyclic contour (`τᵢ` of `DERIV-local-radius-clamp`), each
 * from its triple of `cornerTriples`.
 *
 * @kind geometry
 * @param corners - every vertex with its neighbours, in drawing order
 * @returns one signed angle per corner, in radians
 * @see DERIV-turning-angle
 */
export function turningAngles(corners: readonly CornerPoints[]): readonly number[] {
  return corners.map((corner) => turningAngle(corner.previous, corner.vertex, corner.next));
}
