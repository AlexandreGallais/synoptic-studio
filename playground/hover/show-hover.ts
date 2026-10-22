import { PICK_TOLERANCE, shapeAtPoint } from "../../src";
import { VIEW_SCALE, createViewSvg } from "../kept-shapes";
import { selectElement } from "../page";
import { readArmedTool } from "../toolbar";

import { createOutlineElement } from "./create-outline-element";
import { createSelectionElements } from "./create-selection-elements";
import { HOVER_POINTS } from "./hover-points";
import { readPickAreas } from "./read-pick-areas";

import type { Point } from "../../src";

/**
 * Creates the outline of the shape under a point (`shapeAtPoint`, the reach of a shape without
 * area `PICK_TOLERANCE` over the view's scale): the band about its contour as drawn
 * (`createOutlineElement`), of class `hover`.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the drawing the pointer is over
 * @returns its `<path>`; nothing when no shape is under the point
 * @see docs/domain/interaction.md#_2-selector
 */
function createHoverElements(document: Document, point: Point): readonly SVGPathElement[] {
  const areas = readPickAreas(document);
  const area = areas[shapeAtPoint(areas, point, PICK_TOLERANCE / VIEW_SCALE)];

  return area === undefined ? [] : [createOutlineElement(document, area, "hover")];
}

/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createViewSvg`), which the pointer passes through, holding the outline of the shape selected
 * (`createSelectionElements`), then, with the selector armed, the outline of the shape under the
 * pointer (`createHoverElements`): each a band 2 screen pixels wide about its contour as drawn, a
 * fill, never a native stroke; no hover with a tool armed, nor once the pointer has left the
 * drawing. The drawing's `<svg>` is never touched, so that the export holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showHover(document: Document): void {
  const svg = createViewSvg(document);
  const point = HOVER_POINTS.get(document);

  svg.append(...createSelectionElements(document));

  if (point !== undefined && readArmedTool(document) === "selector") {
    svg.append(...createHoverElements(document, point));
  }

  selectElement(document, "overlay").replaceChildren(svg);
}
