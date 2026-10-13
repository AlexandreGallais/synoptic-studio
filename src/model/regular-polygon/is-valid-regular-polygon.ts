import { areValidRadii } from "../corner-radius";
import { isAnchor, isModelSize, isShapePlacement } from "../placement";

import { isRegularPolygonCornerCount } from "./is-regular-polygon-corner-count";

import type { RegularPolygon } from "./regular-polygon";

/**
 * Whether a regular polygon can enter the model: an integer number of corners from 3 to 12
 * (Q19), width and height integers ≥ 0, one corner radius per corner, integer ≥ 0 (ADR-0029),
 * an anchor of `min`, `mid` or `max`
 * on each axis (F07), a `stretched` boolean (ADR-0037), an angle from 0 to 359 and a finite position (ADR-0035).
 *
 * A size of 0 gives a degenerate polygon, allowed (Q15); a negative or fractional value is
 * rejected (ADR-0003). A radius larger than the polygon is valid: it is clamped (ADR-0007).
 *
 * @kind domain
 * @param polygon - number of corners, width, height, radii and anchor to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#regular-polygon
 */
export function isValidRegularPolygon(polygon: RegularPolygon): boolean {
  const sizes = [polygon.width, polygon.height];

  return [
    isRegularPolygonCornerCount(polygon.corners),
    sizes.every((value) => isModelSize(value)),
    areValidRadii(polygon.radii, polygon.corners),
    isAnchor(polygon.anchor),
    typeof polygon.stretched === "boolean",
    isShapePlacement(polygon),
  ].every(Boolean);
}
