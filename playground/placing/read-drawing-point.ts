import { DRAWING_MARGIN } from "../kept-shapes";
import { selectElement } from "../page";

import type { Point } from "../../src";

/**
 * Reads the point of the drawing under a click, in the coordinates of its SVG: one user unit to one
 * CSS pixel (US-004), the origin a margin from the canvas's top-left (`createDrawingSvg`).
 *
 * @kind procedure
 * @param document - page document
 * @param event - the click on the canvas
 * @returns the point, possibly fractional
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function readDrawingPoint(document: Document, event: MouseEvent): Point {
  const { left, top } = selectElement(document, "canvas").getBoundingClientRect();

  return { x: event.clientX - left - DRAWING_MARGIN, y: event.clientY - top - DRAWING_MARGIN };
}
