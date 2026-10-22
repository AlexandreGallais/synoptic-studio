import { PLACED_SHAPE_SIZE, centredPosition } from "../../src";
import { writeInputValue } from "../page";
import { writeShapePosition } from "../position";

import type { Point } from "../../src";

/**
 * Types the box of a rectangle, a polygon or a round shape placed by a click (F35): 100 × 100,
 * centred on the point on integers (`centredPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the SVG
 * @see docs/domain/interaction.md#_1-tools
 */
export function writePlacedBox(document: Document, point: Point): void {
  const size = { height: PLACED_SHAPE_SIZE, width: PLACED_SHAPE_SIZE };

  writeInputValue(document, "width", size.width);
  writeInputValue(document, "height", size.height);
  writeShapePosition(document, centredPosition(point, size));
}
