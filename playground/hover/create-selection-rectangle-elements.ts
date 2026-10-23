import { createPathElement, screenToUser } from "../../src";
import { readBoxBandPathData, readSidesPathData } from "../helps-frame";
import { readView } from "../view";

import { readRectangleDrag } from "./read-rectangle-drag";
import { readSelectionBox } from "./read-selection-box";

import type { BoundingBox } from "../../src";

/** Width of the rectangle's outline, in screen pixels: a band of half of it on each side. */
const OUTLINE_WIDTH = 1;

/** Width of the light band under the outline, in screen pixels: 1 pixel of it on each side. */
const LIGHT_WIDTH = 3;

/**
 * Creates a band of a width centred on the rectangle's sides (`readBoxBandPathData`), divided by
 * the view's scale (`screenToUser`), of a class.
 *
 * @kind procedure
 * @param document - page document
 * @param box - the rectangle, in user units
 * @param width - the band's width, in screen pixels
 * @returns its `<path>`
 * @see docs/domain/interaction.md#_2-selector
 */
function createSidesBand(document: Document, box: BoundingBox, width: number): SVGPathElement {
  const half = screenToUser(width / 2, readView(document).scale);

  return createPathElement(document, readBoxBandPathData(box, { inward: half, outward: half }));
}

/**
 * Creates the selection rectangle in the layer above the drawing (F37, F43), while a drag with the
 * Selector from an empty place has gone further than 3 pixels from the press (`readRectangleDrag`):
 * its area, a light tint of the outline's color, of class `selection-area`; a light band 3 screen
 * pixels wide centred on its sides, of class `selection-rectangle-light`; then its outline over
 * it, a band 1 screen pixel wide, of class `selection-rectangle` (`createSidesBand`), as Inkscape
 * draws its rubberband over a light outline (`REF-INKSCAPE-NODE-HANDLES`); each a fill with a hole,
 * never a native stroke; a rectangle thinner than a band gets no hole in it.
 *
 * @kind procedure
 * @param document - page document
 * @returns its area, its light band and its outline; nothing without such a drag
 * @see docs/domain/interaction.md#_2-selector
 */
export function createSelectionRectangleElements(document: Document): readonly SVGPathElement[] {
  const drag = readRectangleDrag(document);

  if (drag === undefined) {
    return [];
  }

  const box = readSelectionBox(drag);
  const area = createPathElement(document, readSidesPathData(box, 0));
  const light = createSidesBand(document, box, LIGHT_WIDTH);
  const outline = createSidesBand(document, box, OUTLINE_WIDTH);

  area.classList.add("selection-area");
  light.classList.add("selection-rectangle-light");
  outline.classList.add("selection-rectangle");

  return [area, light, outline];
}
