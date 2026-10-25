import { pointUnderPointer } from "../../src";
import { readPointerOffset, selectElement } from "../page";
import { readView } from "../view";

import type { Point } from "../../src";

/**
 * Reads the point of the drawing under the pointer, in the coordinates of its SVG: the pointer's
 * offset on the canvas (`readPointerOffset`) through the view, its origin plus the offset over its
 * scale (`pointUnderPointer`, ADR-0098), computed, never read back from the page.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press, the move or the click on the canvas
 * @returns the point, possibly fractional
 * @see RULE-interaction-7-navigation-and-history
 */
export function readDrawingPoint(document: Document, event: MouseEvent): Point {
  const { point } = readPointerOffset(selectElement(document, "canvas"), event);

  return pointUnderPointer(readView(document), point);
}
