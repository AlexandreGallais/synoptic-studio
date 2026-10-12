import { edgeOfSelection, isConvertible, isFreeShape } from "../src";

import { selectElement } from "./select-element";
import { showDeleteButton } from "./show-delete-button";

import type { ShapeReading } from "./shape-reading";
import type { VertexSelection } from "../src";

/**
 * Shows the buttons acting on the points: for a regular polygon or a round shape, the conversion (greyed when it
 * has no width or height) and a line saying it is converted first; for a free shape (the
 * rectangle included), adding a point, greyed unless two neighbouring points are selected
 * and deleting points (F08.AC1, F08.AC3, F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @param selection - vertices selected
 * @see docs/backlog/stories/E01-F08-US-015-add-middle-point.md
 */
export function showPointButtons(
  document: Document,
  reading: ShapeReading,
  selection: VertexSelection,
): void {
  const { model } = reading;
  // A polygon or a round shape keeps its points: it is converted first (F08, F10).
  const hasFixedPoints = !isFreeShape(model);
  const edge = edgeOfSelection(
    selection.selected,
    reading.corners.length * Math.sign(model.width * model.height),
  );
  const convert = selectElement(document, "convert");

  convert.hidden = !hasFixedPoints;
  convert.toggleAttribute("disabled", !isConvertible(model));
  selectElement(document, "convert-hint").hidden = !hasFixedPoints;

  showDeleteButton(document, reading, selection);

  selectElement(document, "add-midpoint").toggleAttribute(
    "disabled",
    hasFixedPoints || edge === undefined,
  );
}
