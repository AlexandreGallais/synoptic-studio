import { isFreeShape, shapeAsFreeShape, shapeKind } from "../src";

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
  // A polygon of width or height 0 has no box to stretch (ADR-0033); a round shape converts to
  // the rectangle tool's free shape, valid at a size of 0 (ADR-0036).
  const hasBox = shapeKind(model) === "round" || model.width * model.height > 0;
  const isConvertible = [isValid, !isFreeShape(model), hasBox].every(Boolean);

  if (isConvertible) {
    writeStoredFreeShape(document, shapeAsFreeShape(model));
  }
}
