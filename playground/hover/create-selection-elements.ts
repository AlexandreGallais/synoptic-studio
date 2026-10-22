import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";

import { createOutlineElement } from "./create-outline-element";
import { readShapePickArea } from "./read-shape-pick-area";

/**
 * Creates the outline of the shape selected, the one the panel edits (F36), in the layer above the
 * drawing: the band about its contour as drawn, as the hover's (`createOutlineElement`), of class
 * `selection`, a fill, never a native stroke; shown whichever tool is armed.
 *
 * @kind procedure
 * @param document - page document
 * @returns its `<path>`; nothing while no shape is selected, or the one edited is refused
 * @see docs/domain/interaction.md#_2-selector
 */
export function createSelectionElements(document: Document): readonly SVGPathElement[] {
  return readStoredDrawing(document, DRAWING_KEYS.edited).map(({ shape }) =>
    createOutlineElement(document, readShapePickArea(document, shape), "selection"),
  );
}
