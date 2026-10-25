import { NO_VERTEX_SELECTION, deletionOutcome, freeShapeWithoutPoints } from "../../src";
import { readSelection, writeSelection } from "../page";
import { readEditedFreeShape, writeStoredFreeShape } from "../shape";

import type { FreeShape } from "../../src";

/**
 * Reads the free shape whose selected points may be deleted: a valid free shape (the rectangle
 * included), at least one point selected, and enough points left to make a shape.
 *
 * @kind procedure
 * @param document - page document
 * @param selected - numbers of the points selected, from 0
 * @returns the free shape, `undefined` when nothing may be deleted
 */
function readDeletableShape(
  document: Document,
  selected: readonly number[],
): FreeShape | undefined {
  const shape = readEditedFreeShape(document);
  const isAllowed = [
    selected.length > 0,
    shape !== undefined && deletionOutcome(shape, selected) === "allowed",
  ].every(Boolean);

  return isAllowed ? shape : undefined;
}

/**
 * Deletes the points selected, their neighbours joined, keeps the shape on the page and clears
 * the selection; does nothing when the deletion is refused (F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 */
export function writeDeletedPoints(document: Document): void {
  const { selected } = readSelection(document);
  const shape = readDeletableShape(document, selected);

  if (shape === undefined) {
    return;
  }

  writeStoredFreeShape(document, freeShapeWithoutPoints(shape, selected));
  writeSelection(document, NO_VERTEX_SELECTION);
}
