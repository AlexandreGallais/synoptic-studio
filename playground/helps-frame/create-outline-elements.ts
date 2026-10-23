import { createPathElement, screenToUser } from "../../src";
import { readView } from "../view";

import { readBandPathData } from "./read-band-path-data";

import type { PickArea } from "../../src";

/** Width of the outline, in screen pixels: a band of half of it on each side of the contour. */
const OUTLINE_WIDTH = 2;

/** Width of the light band under the outline, in screen pixels: 1 pixel of it on each side. */
const LIGHT_WIDTH = 4;

/** The longest chord of the outline along a curve, in screen pixels: no corner shows at that size. */
const OUTLINE_STEP = 2;

/**
 * Creates one band about a shape's contour as drawn, a curve followed by chords of at most 2
 * screen pixels, both lengths in user units at the view's scale (`screenToUser`), computed within
 * the helps' frame (`readBandPathData`, EN-149).
 *
 * @kind procedure
 * @param document - page document
 * @param area - what the shape offers the pointer, its contour as drawn
 * @param width - the band's width, in screen pixels
 * @returns its `<path>`, without a class
 * @see docs/domain/interaction.md#_2-selector
 */
function createBand(document: Document, area: PickArea, width: number): SVGPathElement {
  const { scale } = readView(document);

  return createPathElement(
    document,
    readBandPathData(document, area.pieces, {
      halfWidth: screenToUser(width / 2, scale),
      step: screenToUser(OUTLINE_STEP, scale),
    }),
  );
}

/**
 * Creates the outline of a shape in the layer above the drawing (F36, F43): two bands about its
 * contour as drawn, so that it shows on any color, as Inkscape draws its controls' outline under
 * their stroke (`REF-INKSCAPE-CANVAS-ITEM-RECT`): first a light one 4 screen pixels wide, of class
 * `<name>-light`, then the accent's 2 pixels wide over it, of class `<name>`, 1 pixel of the light
 * one showing on each side (`createBand`); each keeps its size on screen at any zoom, and is a
 * fill, never a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param area - what the shape offers the pointer, its contour as drawn
 * @param name - its class: `hover` for the shape under the pointer, `selection` for one selected
 * @returns the light band's `<path>`, then the outline's
 * @see docs/domain/interaction.md#_2-selector
 */
export function createOutlineElements(
  document: Document,
  area: PickArea,
  name: string,
): readonly SVGPathElement[] {
  const light = createBand(document, area, LIGHT_WIDTH);
  const outline = createBand(document, area, OUTLINE_WIDTH);

  light.classList.add(`${name}-light`);
  outline.classList.add(name);

  return [light, outline];
}
