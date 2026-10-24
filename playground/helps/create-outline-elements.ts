import { contourPiecesToPathData } from "../../src";

import { createDashedLineElements } from "./create-dashed-line-elements";

import type { PickArea } from "../../src";

/** Width of an outline, in screen pixels: whole pixels on a contour lying on whole units. */
const OUTLINE_WIDTH = 2;

/**
 * Creates the outline of a shape in the layer above the drawing (F36, US-086): the dashed line of
 * the helps, 2 screen pixels wide, on its contour as drawn (`contourPiecesToPathData`,
 * `createDashedLineElements`), whatever its size and the part of it the view shows.
 *
 * @kind procedure
 * @param document - page document
 * @param area - what the shape offers the pointer, its contour as drawn
 * @param name - its class: `hover` for the shape under the pointer, `selection` for one selected
 * @returns the white line's `<path>`, then the dashes'
 * @see docs/domain/interaction.md#_2-selector
 */
export function createOutlineElements(
  document: Document,
  area: PickArea,
  name: string,
): readonly SVGPathElement[] {
  return createDashedLineElements(document, {
    data: contourPiecesToPathData(area.pieces),
    name,
    width: OUTLINE_WIDTH,
  });
}
