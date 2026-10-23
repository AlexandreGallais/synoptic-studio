import { PICK_TOLERANCE, screenToUser, shapeAtPoint } from "../../src";
import { createOutlineElement } from "../helps-frame";
import { readView } from "../view";

import { readPickAreas } from "./read-pick-areas";

import type { Point } from "../../src";

/**
 * Creates the outline of the shape under a point (`shapeAtPoint`, the reach of a shape without
 * area `PICK_TOLERANCE` in user units, `screenToUser`), found on its whole contour: the band about
 * its contour as drawn (`createOutlineElement`), of class `hover`, computed only within the helps'
 * box (EN-149).
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the drawing the pointer is over; undefined for none
 * @returns its `<path>`; nothing when no shape is under the point
 * @see docs/domain/interaction.md#_2-selector
 */
export function createHoverElements(
  document: Document,
  point: Point | undefined,
): readonly SVGPathElement[] {
  if (point === undefined) {
    return [];
  }

  const areas = readPickAreas(document);
  const reach = screenToUser(PICK_TOLERANCE, readView(document).scale);
  const area = areas[shapeAtPoint(areas, point, reach)];

  return area === undefined ? [] : [createOutlineElement(document, area, "hover")];
}
