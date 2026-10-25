import { PLACED_SHAPE_SIZE, centeredPosition } from "../../src";
import { writeInputValue } from "../page";
import { writeShapePosition } from "../position";

import type { Point } from "../../src";

/**
 * Types the box of a rectangle, a polygon or a round shape placed by a click: 100 × 100,
 * centered on the point on integers (`centeredPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the SVG
 */
export function writePlacedBox(document: Document, point: Point): void {
  const size = { height: PLACED_SHAPE_SIZE, width: PLACED_SHAPE_SIZE };

  writeInputValue(document, "width", size.width);
  writeInputValue(document, "height", size.height);
  writeShapePosition(document, centeredPosition(point, size));
}
