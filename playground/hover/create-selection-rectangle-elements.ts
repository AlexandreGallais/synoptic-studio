import { createPathElement, screenToUser } from "../../src";
import { createDashedLineElements, readInsetBoxPathData } from "../helps";
import { readView } from "../view";

import { readRectangleDrag } from "./read-rectangle-drag";
import { readSelectionBox } from "./read-selection-box";

/** Width of the rectangle's line, in screen pixels, laid half of it inside the rectangle. */
const LINE = 1;

/**
 * Creates the selection rectangle in the layer above the drawing, while a drag with
 * the Selector from an empty place has gone further than 3 pixels from the press
 * (`readRectangleDrag`): its area, a light tint of the outline's color, of class `selection-area`,
 * a fill; then the dashed line of the helps 1 screen pixel wide on its sides moved half a pixel
 * inward (`readInsetBoxPathData`, `createDashedLineElements`), of classes
 * `selection-rectangle-light` and `selection-rectangle`, as Inkscape draws its rubberband
 * (`REF-INKSCAPE-CANVAS-ITEM-RECT`).
 *
 * @kind procedure
 * @param document - page document
 * @returns its area, its white line and its dashes; nothing without such a drag
 */
export function createSelectionRectangleElements(document: Document): readonly SVGPathElement[] {
  const drag = readRectangleDrag(document);

  if (drag === undefined) {
    return [];
  }

  const box = readSelectionBox(drag);
  const area = createPathElement(document, readInsetBoxPathData(box, 0));
  const inset = screenToUser(LINE / 2, readView(document).scale);

  area.classList.add("selection-area");

  return [
    area,
    ...createDashedLineElements(document, {
      data: readInsetBoxPathData(box, inset),
      name: "selection-rectangle",
      width: LINE,
    }),
  ];
}
