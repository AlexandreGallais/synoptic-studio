import { cyclicItem } from "../../geometry";

import { resizeRadii } from "./resize-radii";

import type { Corner } from "../../geometry";
import type { Point } from "../../math";

/**
 * Corners of a shape: each vertex of its contour with the requested radius of the same number
 * (ADR-0029), clamped later by the geometry of F01 (ADR-0007).
 *
 * @kind domain
 * @param points - vertices of the contour, in drawing order (Q11)
 * @param radii - requested radius of each vertex, as many as `points` (`areValidRadii`); a vertex
 * without one is sharp (radius 0), as a new vertex is (ADR-0029)
 * @returns one corner per vertex, in drawing order
 * @see RULE-shapes-2-corner-radius
 */
export function cornersWithRadii(
  points: readonly Point[],
  radii: readonly number[],
): readonly Corner[] {
  const resized = resizeRadii(radii, points.length);

  return points.map((point, index) => ({ point, radius: cyclicItem(resized, index, 0) }));
}
