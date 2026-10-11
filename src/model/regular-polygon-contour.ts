import { fitInBox, unitRegularPolygon } from "../geometry";

import type { RegularPolygon } from "./regular-polygon";
import type { Point } from "../math";

/**
 * Contour of a regular polygon: the largest one with a flat base fitting its `width × height`
 * box, centered in it, clockwise on screen from the topmost vertex, the leftmost on a tie
 * (Q1, Q11, Q18). The vertices are derived, fractional, never stored (ADR-0003).
 *
 * @kind domain
 * @param polygon - valid polygon (`isValidRegularPolygon`)
 * @returns its `n` vertices in drawing order
 * @see DERIV-regular-polygon-fit
 */
export function regularPolygonContour(polygon: RegularPolygon): readonly Point[] {
  return fitInBox(unitRegularPolygon(polygon.corners), polygon, {
    horizontal: "mid",
    vertical: "mid",
  });
}
