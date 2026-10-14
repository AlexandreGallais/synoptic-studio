import { hasArea, isRoundShape, roundShapeAsRoundedRectangle } from "../../src";

import { readShape } from "./read-shape";
import { writeStoredFreeShape } from "./write-stored-free-shape";

import type { RoundShape } from "../../src";

/**
 * Reads the round shape shown, when it is valid.
 *
 * @kind procedure
 * @param document - page document
 * @returns the round shape, `undefined` for another shape or a refused one
 * @see docs/backlog/stories/E14-F10-US-030-convert-round-shape.md
 */
function readValidRoundShape(document: Document): RoundShape | undefined {
  const { isValid, model } = readShape(document);

  return isValid && isRoundShape(model) ? model : undefined;
}

/**
 * Converts the valid round shape shown into the rounded rectangle of its box, its 4 corners
 * rounded by half its smaller side rounded up, and keeps it on the page (F10.AC5); a box of width
 * or height 0 is not converted (F08).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E14-F10-US-030-convert-round-shape.md
 */
export function writeRoundedRectangle(document: Document): void {
  const shape = readValidRoundShape(document);

  if (shape !== undefined && hasArea(shape)) {
    writeStoredFreeShape(document, roundShapeAsRoundedRectangle(shape));
  }
}
