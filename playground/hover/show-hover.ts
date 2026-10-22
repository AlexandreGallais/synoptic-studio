import {
  PICK_TOLERANCE,
  contourBand,
  contourPiecesToPathData,
  createPathElement,
  shapeAtPoint,
} from "../../src";
import { VIEW_SCALE, createViewSvg } from "../kept-shapes";
import { selectElement } from "../page";
import { readArmedTool } from "../toolbar";

import { HOVER_POINTS } from "./hover-points";
import { readPickAreas } from "./read-pick-areas";

import type { Point } from "../../src";

/** Width of the outline, in screen pixels: a band of half of it on each side of the contour. */
const OUTLINE_WIDTH = 2;

/** The longest chord of the outline along a curve, in screen pixels: no corner shows at that size. */
const OUTLINE_STEP = 2;

/**
 * Creates the outline of the shape under a point (`shapeAtPoint`, the reach of a shape without
 * area `PICK_TOLERANCE` over the view's scale): the band about its contour as drawn
 * (`contourBand`), of class `hover`, a fill, never a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point of the drawing the pointer is over
 * @returns its `<path>`; nothing when no shape is under the point
 * @see docs/domain/interaction.md#_2-selector
 */
function createOutlineElements(document: Document, point: Point): readonly SVGPathElement[] {
  const areas = readPickAreas(document);
  const area = areas[shapeAtPoint(areas, point, PICK_TOLERANCE / VIEW_SCALE)];

  if (area === undefined) {
    return [];
  }

  const band = contourBand(area.pieces, OUTLINE_WIDTH / 2 / VIEW_SCALE, OUTLINE_STEP / VIEW_SCALE);
  const outline = createPathElement(
    document,
    band.map((subpath) => contourPiecesToPathData(subpath)).join(" "),
  );

  outline.classList.add("hover");

  return [outline];
}

/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createViewSvg`), which the pointer passes through, holding, with the selector armed, the
 * outline of the shape under the pointer (`createOutlineElements`): a band 2 screen pixels wide
 * about its contour as drawn, a fill, never a native stroke; nothing with a tool armed, nor once
 * the pointer has left the drawing. The drawing's `<svg>` is never touched, so that the export
 * holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showHover(document: Document): void {
  const svg = createViewSvg(document);
  const point = HOVER_POINTS.get(document);

  if (point !== undefined && readArmedTool(document) === "selector") {
    svg.append(...createOutlineElements(document, point));
  }

  selectElement(document, "overlay").replaceChildren(svg);
}
