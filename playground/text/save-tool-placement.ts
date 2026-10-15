import { readShapeKind, selectElement } from "../page";

import { readFieldValue } from "./read-field-value";
import { TOOL_PLACEMENT } from "./tool-placement";

/**
 * Keeps the placement of the kind about to be left, when the other kind is chosen (US-032): its
 * X, Y and angle fields and the shape's position. Choosing a shape after a shape keeps nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param kind - the kind about to be chosen
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
export function saveToolPlacement(document: Document, kind: string): void {
  const wasText = readShapeKind(document) === "text";

  if (wasText === (kind === "text")) {
    return;
  }

  const { x, y } = selectElement(document, "shape").dataset;

  selectElement(document, "placement").dataset[
    wasText ? TOOL_PLACEMENT.textKey : TOOL_PLACEMENT.shapeKey
  ] = JSON.stringify({
    fields: TOOL_PLACEMENT.fields.map((id) => readFieldValue(document, id)),
    position: { x, y },
  });
}
