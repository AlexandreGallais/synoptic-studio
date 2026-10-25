import { isInPaintedArea, polygonDistance } from "../../geometry";

import type { PickArea } from "./pick-area";
import type { Point } from "../../math";

/**
 * Whether a point reaches a shape: in its painted area by the nonzero rule (`isInPaintedArea`);
 * for a shape without area, within a tolerance of the outline its corners join
 * (`polygonDistance`).
 *
 * @kind domain
 * @param area - the shape's contour, corners and whether it is flat (`shapePickArea`, `textPickArea`)
 * @param point - the point asked, in user units
 * @param tolerance - the reach of a shape without area, in user units (`PICK_TOLERANCE` over the view's scale)
 * @returns true when the point reaches the shape
 * @see RULE-interaction-2-selector
 */
export function isPickAreaAt(area: PickArea, point: Point, tolerance: number): boolean {
  return area.isFlat
    ? polygonDistance(area.corners, point) <= tolerance
    : isInPaintedArea(area.pieces, point);
}
