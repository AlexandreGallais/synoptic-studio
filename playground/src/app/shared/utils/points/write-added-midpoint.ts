import { edgeOfSelection, freeShapeWithMidpoint, hasArea } from "editor";

import { readSelection, writeSelection } from "../page";
import { readEditedFreeShape, readShape, writeStoredFreeShape } from "../shape";

/**
 * Adds a sharp point in the middle of the edge between the two neighbouring points selected of a
 * valid free shape (the rectangle included), keeps the shape on the page and selects the new
 * point alone, ready to be moved (F08.AC3).
 *
 * @kind procedure
 * @param document - page document
 */
export function writeAddedMidpoint(document: Document): void {
  const shape = readEditedFreeShape(document);
  const { corners, model } = readShape(document);
  // No point is added to a shape of width or height 0: it would lie on its one line.
  const edge = edgeOfSelection(
    readSelection(document).selected,
    corners.length * Number(hasArea(model)),
  );

  if (shape === undefined || edge === undefined) {
    return;
  }

  writeStoredFreeShape(document, freeShapeWithMidpoint(shape, edge));
  writeSelection(document, { selected: [edge + 1], start: edge + 1 });
}
