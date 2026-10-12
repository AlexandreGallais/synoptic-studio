import { cyclicItem } from "./cyclic-item";
import { turningAngle } from "./turning-angle";

import type { Point } from "../math";

/**
 * Turning angle at every vertex of a cyclic contour (`τᵢ` of `DERIV-local-radius-clamp`).
 *
 * The first vertex turns from the last edge, the last vertex towards the first edge (Q11).
 *
 * @kind geometry
 * @param contour - vertices in drawing order
 * @returns one signed angle per vertex, in radians
 * @see DERIV-turning-angle
 */
export function turningAngles(contour: readonly Point[]): readonly number[] {
  return contour.map((vertex, index) =>
    turningAngle(
      cyclicItem(contour, index - 1, vertex),
      vertex,
      cyclicItem(contour, index + 1, vertex),
    ),
  );
}
