import { isAnchor } from "./is-anchor";
import { isModelSize } from "./is-model-size";
import { isRegularPolygonCornerCount } from "./is-regular-polygon-corner-count";

import type { RegularPolygon } from "./regular-polygon";

/**
 * Whether a regular polygon can enter the model: an integer number of corners from 3 to 12
 * (Q19), width, height and corner radius integers ≥ 0, and an anchor of `min`, `mid` or `max`
 * on each axis (F07).
 *
 * A size of 0 gives a degenerate polygon, allowed (Q15); a negative or fractional value is
 * rejected (ADR-0003). A radius larger than the polygon is valid: it is clamped (ADR-0007).
 *
 * @kind domain
 * @param polygon - number of corners, width, height, radius and anchor to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#regular-polygon
 */
export function isValidRegularPolygon(polygon: RegularPolygon): boolean {
  const sizes = [polygon.width, polygon.height, polygon.radius];

  return (
    isRegularPolygonCornerCount(polygon.corners) &&
    sizes.every((value) => isModelSize(value)) &&
    isAnchor(polygon.anchor)
  );
}
