import { REGULAR_POLYGON_CORNER_RANGE } from "./regular-polygon-corner-range";

/**
 * Whether a number of corners is allowed for a regular polygon: an integer from 3 to 12 (Q19).
 *
 * @kind domain
 * @param corners - number of corners to check
 * @returns `true` when it is allowed
 * @see docs/domain/shapes.md#regular-polygon
 */
export function isRegularPolygonCornerCount(corners: number): boolean {
  const { max, min } = REGULAR_POLYGON_CORNER_RANGE;

  return Number.isSafeInteger(corners) && corners >= min && corners <= max;
}
