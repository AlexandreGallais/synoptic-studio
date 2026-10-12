import { cyclicItem } from "./cyclic-item";
import { filletSetback } from "./fillet-setback";
import { turningAngles } from "./turning-angles";

import type { Corner } from "./corner";

/**
 * Setback requested at every corner of a cyclic contour, before any reduction.
 *
 * Formula: sᵢ = rᵢ · tan(|τᵢ| / 2).
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, in drawing order
 * @returns one setback per corner, 0 at aligned vertices
 * @see DERIV-local-radius-clamp
 */
export function cornerSetbacks(corners: readonly Corner[]): readonly number[] {
  const angles = turningAngles(corners.map((corner) => corner.point));

  return corners.map((corner, index) => filletSetback(corner.radius, cyclicItem(angles, index, 0)));
}
