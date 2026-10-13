import { positionAfterResize } from "../../src";
import { selectElement } from "../page";
import { readShape, writeCappedSizes } from "../shape";

import { writeShapePosition } from "./write-shape-position";

import type { Size, TurnedBox } from "../../src";

/**
 * Reads the angle, position and size of the turned shape as last drawn, kept by `writeDrawnBox`.
 *
 * @kind procedure
 * @param document - page document
 * @returns the box last drawn, nothing when the shape drawn was straight
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
function readDrawnBox(document: Document): TurnedBox | undefined {
  const { drawnBox } = selectElement(document, "shape").dataset;

  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeDrawnBox only, from a valid shape
  return drawnBox === undefined ? undefined : (JSON.parse(drawnBox) as TurnedBox);
}

/**
 * Reads the width and height of the shape as it will be drawn: capped at 100 000 (Q20), and a
 * kept free shape's exact size while its field still shows it rounded.
 *
 * @kind procedure
 * @param document - page document
 * @returns them, nothing for a refused shape
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
function readTypedSize(document: Document): Size | undefined {
  writeCappedSizes(document);

  const { isValid, model } = readShape(document);

  return isValid ? { height: model.height, width: model.width } : undefined;
}

/**
 * Places a turned shape whose width or height was just typed so that its turned top-left corner
 * stays where it was drawn, as Figma (F04, `positionAfterResize`). A straight shape keeps its
 * position, its top-left fixed (F09); a refused shape moves nothing.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function writeResizedPosition(document: Document): void {
  const before = readDrawnBox(document);
  const size = readTypedSize(document);

  if (before !== undefined && size !== undefined) {
    writeShapePosition(document, positionAfterResize(before, size));
  }
}
