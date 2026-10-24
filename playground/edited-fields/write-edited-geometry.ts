import { shapePanelValues } from "../../src";
import { selectElement, writeInputValue } from "../page";
import { writeShapePosition } from "../position";
import { readStoredFreeShape } from "../shape";

import type { Shape } from "../../src";

/**
 * Writes the width and height of a shape set by its fields in the free shape the page keeps, when
 * it keeps one, and only them, so that a size with fractions, a converted polygon's, is read back
 * exactly while its field shows it (`readFreeShapeShape`); its points, radii and angle stay.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - the shape set
 * @see docs/domain/interaction.md#_2-selector
 */
function writeStoredSize(document: Document, shape: Shape): void {
  const kept = readStoredFreeShape(document);

  if (kept === undefined) {
    return;
  }

  selectElement(document, "shape").dataset["freeShape"] = JSON.stringify({
    ...kept,
    height: shape.height,
    width: shape.width,
  });
}

/**
 * Writes a shape the panel edits alone, set by its X, Y, Width or Height (US-085), into the page:
 * its size (`writeStoredSize`) and its position, the top-left of its box before it turns
 * (`writeShapePosition`, ADR-0035), then its values in its four fields (`shapePanelValues`), X and
 * Y the start of its straight box, as the redraw shows them, the field typed in included.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - the shape set, or as it was for a value refused
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeEditedGeometry(document: Document, shape: Shape): void {
  const values = shapePanelValues(shape);

  writeStoredSize(document, shape);
  writeShapePosition(document, { x: shape.x, y: shape.y });
  writeInputValue(document, "width", values.width);
  writeInputValue(document, "height", values.height);
  writeInputValue(document, "shape-x", values.x);
  writeInputValue(document, "shape-y", values.y);
}
