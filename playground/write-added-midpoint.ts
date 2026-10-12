import { edgeOfSelection, freeShapeWithMidpoint } from "../src";

import { readEditedFreeShape } from "./read-edited-free-shape";
import { readSelection } from "./read-selection";
import { readShape } from "./read-shape";
import { writeSelection } from "./write-selection";
import { writeStoredFreeShape } from "./write-stored-free-shape";

/**
 * Adds a sharp point in the middle of the edge between the two neighbouring points selected of a
 * valid free shape (the rectangle included), keeps the shape on the page and selects the new
 * point alone, ready to be moved (F08.AC3).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-US-015-add-middle-point.md
 */
export function writeAddedMidpoint(document: Document): void {
  const shape = readEditedFreeShape(document);
  const { corners, model } = readShape(document);
  // No point is added to a shape of width or height 0: it would lie on its one line (AUD-005).
  const edge = edgeOfSelection(
    readSelection(document).selected,
    corners.length * Math.sign(model.width * model.height),
  );

  if (shape === undefined || edge === undefined) {
    return;
  }

  writeStoredFreeShape(document, freeShapeWithMidpoint(shape, edge));
  writeSelection(document, { selected: [edge + 1], start: edge + 1 });
}
