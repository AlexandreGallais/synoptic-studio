import { cyclicItem } from "../geometry";

import type { Corner } from "../geometry";
import type { Point } from "../math";

/**
 * Corners of a shape: each vertex of its contour with the requested radius of the same number
 * (ADR-0029), clamped later by the geometry of F01 (ADR-0007).
 *
 * @kind domain
 * @param points - vertices of the contour, in drawing order (Q11)
 * @param radii - requested radius of each vertex, as many as `points` (`areValidRadii`)
 * @returns one corner per vertex, in drawing order
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function cornersWithRadii(
  points: readonly Point[],
  radii: readonly number[],
): readonly Corner[] {
  return points.map((point, index) => ({ point, radius: cyclicItem(radii, index, 0) }));
}
