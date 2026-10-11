import { positionAfterResize } from "../src";

import { readNumber } from "./read-number";
import { selectElement } from "./select-element";
import { writeShapePosition } from "./write-shape-position";

import type { Size, TurnedBox } from "../src";

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
 * Reads the width and height typed.
 *
 * @kind procedure
 * @param document - page document
 * @returns them, nothing when one is not a number ≥ 0
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
function readTypedSize(document: Document): Size | undefined {
  const size = { height: readNumber(document, "height"), width: readNumber(document, "width") };

  return [size.width, size.height].every((value) => value >= 0) ? size : undefined;
}

/**
 * Places a turned shape whose width or height was just typed so that its turned top-left corner
 * stays where it was drawn, as Figma (F04, `positionAfterResize`). A straight shape keeps its
 * position, its top-left fixed (F09); a size that is not a number ≥ 0 moves nothing.
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
