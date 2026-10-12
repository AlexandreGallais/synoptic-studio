import { isConvertible, shapeAsFreeShape } from "../src";

import { readShape } from "./read-shape";
import { writeStoredFreeShape } from "./write-stored-free-shape";

/**
 * Converts the polygon or the round shape shown into a free shape, unmoved, and keeps it on the
 * page (F08.AC1, F10.AC3); a refused shape, a free shape already, or a shape of width or height
 * 0, which has no box to stretch, is not converted (ADR-0033); a round shape of size 0 is (ADR-0036).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F10-US-020-round-shape-playground.md
 */
export function writeConvertedShape(document: Document): void {
  const { isValid, model } = readShape(document);

  if (isValid && isConvertible(model)) {
    writeStoredFreeShape(document, shapeAsFreeShape(model));
  }
}
