import { createPathElement } from "../../src";

import type { DashedLine } from "./dashed-line";

/**
 * Creates a help of the layer above the drawing as one line of white and black dashes (US-086,
 * ADR-0099): the path it follows stroked white, of class `<name>-light`, then stroked black over
 * it, dashed 4 and 4 screen pixels from the path's start, of class `<name>`, so that its dashes
 * alternate black and white, as Inkscape draws a control's dashes again in a second color in the
 * gaps of the first (`REF-INKSCAPE-CANVAS-ITEM-RECT`); each line's width and dashes divided by the
 * view's scale (`screenToUser`), so that they keep their size on screen; `fill: none` and the
 * colors in the page's style.
 *
 * @kind procedure
 * @param document - page document
 * @param line - the path it follows, its class and its width in screen pixels
 * @returns the white line's `<path>`, then the dashes'
 * @see docs/domain/interaction.md#_2-selector
 */
export function createDashedLineElements(
  document: Document,
  line: DashedLine,
): readonly SVGPathElement[] {
  return [createPathElement(document, line.data)];
}
