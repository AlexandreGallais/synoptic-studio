import { fitInBox, stretchedPolygon, unitRegularPolygon } from "../../geometry";

import type { RegularPolygon } from "./regular-polygon";
import type { Point } from "../../math";

/**
 * Contour of a regular polygon: the largest one with a flat base fitting its `width × height`
 * box, placed by its anchor (centered by default, F07), or, stretched, the polygon of rule C
 * touching the four sides of its box (`DERIV-stretched-polygon`, ADR-0037, its anchor without
 * effect), in its box at its position (ADR-0035), clockwise on screen from the right end of
 * its base (Q1, Q11, Q18 amended). The vertices are derived, fractional, never stored (ADR-0003).
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`)
 * @returns its `n` vertices in drawing order
 * @see DERIV-regular-polygon-fit
 */
export function regularPolygonContour(polygon: RegularPolygon): readonly Point[] {
  const placed = polygon.stretched
    ? stretchedPolygon(polygon.corners, polygon)
    : fitInBox(unitRegularPolygon(polygon.corners), polygon, polygon.anchor);

  // Adding a position of 0 is exact: the drawing of F02 and F07 is unchanged.
  return placed.map((point) => ({
    x: point.x + polygon.x,
    y: point.y + polygon.y,
  }));
}
