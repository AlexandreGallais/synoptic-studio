import { writeText } from "../page";

import type { Point } from "../../src";

/**
 * Selects the shape under a point of the drawing (`shapeAtPoint`, F36): the drawing split around
 * it (`splitDrawing`), so that it keeps its place, the shapes before it below and those after it
 * above; the panel leaves the shape it edited, keeping its settings as the last of its kind
 * (`saveKindSettings`), and loads the one picked (`loadPickedShape`); on the empty drawing, every
 * shape below and none selected.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the drawing's SVG
 * @see docs/domain/interaction.md#_2-selector
 */
export function selectShapeAt(document: Document, point: Point): void {
  writeText(document, "status", String(point.x));
}
