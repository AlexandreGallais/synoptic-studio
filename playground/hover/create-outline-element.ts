import { contourBand, contourPiecesToPathData, createPathElement } from "../../src";
import { VIEW_SCALE } from "../kept-shapes";

import type { PickArea } from "../../src";

/** Width of the outline, in screen pixels: a band of half of it on each side of the contour. */
const OUTLINE_WIDTH = 2;

/** The longest chord of the outline along a curve, in screen pixels: no corner shows at that size. */
const OUTLINE_STEP = 2;

/**
 * Creates the outline of a shape in the layer above the drawing (F36): the band 2 screen pixels
 * wide about its contour as drawn (`contourBand`), a curve followed by chords of at most 2 screen
 * pixels, both divided by the view's scale; a fill, never a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param area - what the shape offers the pointer, its contour as drawn
 * @param name - its class: `hover` for the shape under the pointer, `selection` for the one selected
 * @returns its `<path>`
 * @see docs/domain/interaction.md#_2-selector
 */
export function createOutlineElement(
  document: Document,
  area: PickArea,
  name: string,
): SVGPathElement {
  const band = contourBand(area.pieces, OUTLINE_WIDTH / 2 / VIEW_SCALE, OUTLINE_STEP / VIEW_SCALE);
  const outline = createPathElement(
    document,
    band.map((subpath) => contourPiecesToPathData(subpath)).join(" "),
  );

  outline.classList.add(name);

  return outline;
}
