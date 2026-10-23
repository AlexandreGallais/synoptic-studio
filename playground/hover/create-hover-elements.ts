import { createOutlineElement } from "../helps-frame";

import { readPickAreas } from "./read-pick-areas";
import { readShapeAt } from "./read-shape-at";

import type { Point } from "../../src";

/**
 * Creates the outline of the shape under a point as a click reaches it (`readShapeAt`), found on
 * its whole contour: the band about
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
  const area = areas[readShapeAt(document, areas, point)];

  return area === undefined ? [] : [createOutlineElement(document, area, "hover")];
}
