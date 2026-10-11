import {
  contourPiecesToPathData,
  createPathElement,
  rectangleCorners,
  roundedContour,
  turnedCorners,
} from "../src";

import { createStraightBoxElement } from "./create-straight-box-element";

import type { BoundingBox, Corner, FreeShape, RegularPolygon } from "../src";

/** Width of the line drawing the turned box, in user units. */
const LINE = 1;

/**
 * Corners of the shape's box, inset on each side, turned with the shape (ADR-0035).
 *
 * @kind procedure
 * @param model - shape read from the inputs
 * @param inset - distance from the box's sides, in user units
 * @returns the four corners, turned
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readBoxCorners(model: FreeShape | RegularPolygon, inset: number): readonly Corner[] {
  const box = rectangleCorners({
    angle: 0,
    height: Math.max(model.height - 2 * inset, 0),
    radii: [0, 0, 0, 0],
    width: Math.max(model.width - 2 * inset, 0),
    x: model.x + inset,
    y: model.y + inset,
  });

  return turnedCorners(box, model);
}

/**
 * Path data of a sharp contour of corners.
 *
 * @kind procedure
 * @param corners - corners, radius 0
 * @returns the `d` of the closed contour
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readPathData(corners: readonly Corner[]): string {
  return contourPiecesToPathData(roundedContour(corners));
}

/**
 * Draws the boxes of a shape behind it. A polygon's box, grey, shows its anchor (F07) and turns
 * with it. A turned shape also gets its straight box, dashed, and its turned box as a thin frame,
 * the selection (F04): drawn by fills, never by a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param model - shape read from the inputs
 * @param straight - straight box of the drawn shape
 * @returns the polygon's box and, for a turned shape, its straight box and its frame
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function createBoxElements(
  document: Document,
  model: FreeShape | RegularPolygon,
  straight: BoundingBox,
): readonly SVGPathElement[] {
  const outer = readPathData(readBoxCorners(model, 0));
  const box = createPathElement(document, outer);
  const frame = createPathElement(
    document,
    `${outer} ${readPathData(readBoxCorners(model, LINE))}`,
  );

  box.classList.add("box");
  frame.classList.add("turned-box");

  const boxes = "anchor" in model ? [box] : [];

  return model.angle === 0
    ? boxes
    : [createStraightBoxElement(document, straight), ...boxes, frame];
}
