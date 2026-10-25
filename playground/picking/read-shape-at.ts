import { PICK_TOLERANCE, screenToUser, shapeAtPoint } from "../../src";
import { readView } from "../view";

import type { PickArea, Point } from "../../src";

/**
 * Reads the shape under a point of the drawing as a click reaches it (`shapeAtPoint`, F36): the
 * reach of a shape without area `PICK_TOLERANCE` screen pixels, in user units at the view's scale
 * (`screenToUser`, F42), so that a line is reached as far on screen at any zoom.
 *
 * @kind procedure
 * @param document - page document
 * @param areas - what each shape offers the pointer, the first drawn first (`readPickAreas`)
 * @param point - the point, in the coordinates of the drawing's SVG
 * @returns the index of the shape, the topmost reached; −1 for none
 * @see RULE-interaction-2-selector
 */
export function readShapeAt(document: Document, areas: readonly PickArea[], point: Point): number {
  return shapeAtPoint(areas, point, screenToUser(PICK_TOLERANCE, readView(document).scale));
}
