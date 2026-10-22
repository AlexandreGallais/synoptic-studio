import { subtract } from "../../src";
import { DRAWING_MARGIN } from "../kept-shapes";
import { readPointerOffset, selectElement } from "../page";

import type { Point } from "../../src";

/**
 * Reads the point of the drawing under a click, in the coordinates of its SVG: one user unit to one
 * CSS pixel (US-004), the pointer on the canvas (`readPointerOffset`) less the margin of the
 * drawing's origin from its top-left (`createDrawingSvg`).
 *
 * @kind procedure
 * @param document - page document
 * @param event - the click on the canvas
 * @returns the point, possibly fractional
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function readDrawingPoint(document: Document, event: MouseEvent): Point {
  const { point } = readPointerOffset(selectElement(document, "canvas"), event);

  return subtract(point, { x: DRAWING_MARGIN, y: DRAWING_MARGIN });
}
