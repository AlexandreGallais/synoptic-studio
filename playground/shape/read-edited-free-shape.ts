import { isFreeShape } from "../../src";

import { readShape } from "./read-shape";

import type { FreeShape } from "../../src";

/**
 * Reads the free shape whose points may be typed: the one shown, when it is valid and not a
 * regular polygon nor a round shape (converted first).
 *
 * @kind procedure
 * @param document - page document
 * @returns the free shape, `undefined` when none can be edited
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function readEditedFreeShape(document: Document): FreeShape | undefined {
  const { isValid, model } = readShape(document);

  return isValid && isFreeShape(model) ? model : undefined;
}
