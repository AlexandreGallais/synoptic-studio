import {
  contourPiecesToPathData,
  createPathElement,
  rectangleCorners,
  roundedContour,
} from "../src";

import type { Rectangle, RegularPolygon } from "../src";

/**
 * Draws the box a polygon is placed in, a grey sharp rectangle from (0, 0) to (width, height)
 * behind it, so that its anchor shows (F07); a rectangle fills its box and gets none.
 *
 * @kind procedure
 * @param document - page document
 * @param model - shape read from the inputs
 * @returns the box's `<path>` for a polygon, nothing for a rectangle
 * @see docs/backlog/stories/E01-F07-US-010-anchor-picker-playground.md
 */
export function createBoxElements(
  document: Document,
  model: Rectangle | RegularPolygon,
): readonly SVGPathElement[] {
  if (!("anchor" in model)) {
    return [];
  }

  const corners = rectangleCorners({ height: model.height, radius: 0, width: model.width });
  const box = createPathElement(document, contourPiecesToPathData(roundedContour(corners)));

  box.classList.add("box");

  return [box];
}
