import { cyclicItem } from "../contour";

import { filletSetback } from "./fillet-setback";

import type { Corner } from "../contour";

/**
 * Setback requested at every corner of a cyclic contour, before any reduction.
 *
 * Formula: sᵢ = rᵢ · tan(|τᵢ| / 2).
 *
 * @kind geometry
 * @param corners - vertices with their requested radii, in drawing order
 * @param angles - turning angle `τᵢ` of each corner, in drawing order
 * @returns one setback per corner, 0 at aligned vertices
 * @see DERIV-local-radius-clamp
 */
export function cornerSetbacks(
  corners: readonly Corner[],
  angles: readonly number[],
): readonly number[] {
  return corners.map((corner, index) => filletSetback(corner.radius, cyclicItem(angles, index, 0)));
}
