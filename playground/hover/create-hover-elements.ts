import { createOutlineElements } from "../helps";
import { readPickAreas, readShapeAt } from "../picking";

import type { Point } from "../../src";

/**
 * Creates the outline of the shape under a point as a click reaches it (`readShapeAt`), found on
 * its whole contour: the dashed line of the helps on its contour as drawn (`createOutlineElements`),
 * of classes `hover-light` and `hover`.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the drawing the pointer is over; undefined for none
 * @returns their `<path>` elements; nothing when no shape is under the point
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

  return area === undefined ? [] : createOutlineElements(document, area, "hover");
}
