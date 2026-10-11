import { cornersWithRadii } from "./corners-with-radii";
import { regularPolygonContour } from "./regular-polygon-contour";

import type { RegularPolygon } from "./regular-polygon";
import type { Corner } from "../geometry";

/**
 * Corners of a regular polygon: each vertex of its contour with its requested radius (ADR-0029),
 * clamped later by the geometry of F01 (ADR-0007, Q8).
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`)
 * @returns one corner per vertex, in drawing order
 * @see docs/domain/shapes.md#regular-polygon
 */
export function regularPolygonCorners(polygon: RegularPolygon): readonly Corner[] {
  return cornersWithRadii(regularPolygonContour(polygon), polygon.radii);
}
