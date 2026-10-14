import { cornersWithRadii } from "../corner-radius";

import { regularPolygonContour } from "./regular-polygon-contour";

import type { RegularPolygon } from "./regular-polygon";
import type { Corner } from "../../geometry";

/**
 * Straight corners of a regular polygon, placed at its position: each vertex of its contour with
 * its requested radius (ADR-0029), clamped later by the geometry of F01 (ADR-0007, Q8); its angle
 * is not applied (`turnedCorners`, ADR-0035).
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`)
 * @returns one corner per vertex, in drawing order
 * @see docs/domain/shapes.md#regular-polygon
 */
export function regularPolygonCorners(polygon: RegularPolygon): readonly Corner[] {
  return cornersWithRadii(regularPolygonContour(polygon), polygon.radii);
}
