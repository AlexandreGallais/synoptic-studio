import { edgeOfSelection, isFreeShape } from "../../src";
import { selectElement } from "../page";

import { showDeleteButton } from "./show-delete-button";

import type { VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/**
 * Shows the buttons acting on the points: for a regular polygon or a round shape, the
 * conversion — two for a round shape — (greyed when it has no width or height) and a line saying it is converted first; for a free shape (the
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
  const isConverted = !isFreeShape(model);
  const isFlat = model.width === 0 || model.height === 0;
  const edge = edgeOfSelection(
    selection.selected,
    reading.corners.length * Math.sign(model.width * model.height),
  );
  const convert = selectElement(document, "convert");

  const rounded = selectElement(document, "convert-rounded");

  convert.hidden = !isConverted;
  convert.toggleAttribute("disabled", isFlat);
  // A round shape also converts to the rounded rectangle of its box (F10.AC5).
  rounded.hidden = !("kind" in model);
  rounded.toggleAttribute("disabled", isFlat);
  selectElement(document, "convert-hint").hidden = !isConverted;

  showDeleteButton(document, reading, selection);

  selectElement(document, "add-midpoint").toggleAttribute(
    "disabled",
    !isFreeShape(model) || edge === undefined,
  );
}
