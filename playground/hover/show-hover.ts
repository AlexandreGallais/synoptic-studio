import { PICK_TOLERANCE, screenToUser, shapeAtPoint } from "../../src";
import { VIEW_SCALE, createViewSvg } from "../kept-shapes";
import { selectElement } from "../page";
import { readArmedTool } from "../toolbar";

import { createOutlineElement } from "./create-outline-element";
import { createSelectionElements } from "./create-selection-elements";
import { createSelectionRectangleElements } from "./create-selection-rectangle-elements";
import { HOVER_POINTS } from "./hover-points";
import { readPickAreas } from "./read-pick-areas";

import type { Point } from "../../src";

/**
 * Creates the outline of the shape under a point (`shapeAtPoint`, the reach of a shape without
 * area `PICK_TOLERANCE` in user units, `screenToUser`): the band about its contour as drawn
 * (`createOutlineElement`), of class `hover`.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the drawing the pointer is over; undefined for none
 * @returns its `<path>`; nothing when no shape is under the point
 * @see docs/domain/interaction.md#_2-selector
 */
function createHoverElements(
  document: Document,
  point: Point | undefined,
): readonly SVGPathElement[] {
  if (point === undefined) {
    return [];
  }

  const areas = readPickAreas(document);
  const area = areas[shapeAtPoint(areas, point, screenToUser(PICK_TOLERANCE, VIEW_SCALE))];

  return area === undefined ? [] : [createOutlineElement(document, area, "hover")];
}

/**
 * Reads the point whose shape is outlined as under the pointer (`HOVER_POINTS`): with the selector
 * armed only, a click then picking a shape, and none while a selection rectangle is drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param isShown - whether the outline may be drawn: false while a selection rectangle is
 * @returns the point; undefined for none
 * @see docs/domain/interaction.md#_2-selector
 */
function readHoverPoint(document: Document, isShown: boolean): Point | undefined {
  return isShown && readArmedTool(document) === "selector" ? HOVER_POINTS.get(document) : undefined;
}

/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createViewSvg`), which the pointer passes through, holding the outline of the shape selected
 * (`createSelectionElements`), then, with the selector armed, the outline of the shape under the
 * pointer (`createHoverElements`): each a band 2 screen pixels wide about its contour as drawn, a
 * fill, never a native stroke; no hover with a tool armed, nor once the pointer has left the
 * drawing, nor while a selection rectangle is drawn, which comes last
 * (`createSelectionRectangleElements`, F37). The drawing's `<svg>` is never touched, so that the
 * export holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showHover(document: Document): void {
  const svg = createViewSvg(document);
  const rectangle = createSelectionRectangleElements(document);

  svg.append(
    ...createSelectionElements(document),
    ...createHoverElements(document, readHoverPoint(document, rectangle.length === 0)),
    ...rectangle,
  );

  selectElement(document, "overlay").replaceChildren(svg);
}
