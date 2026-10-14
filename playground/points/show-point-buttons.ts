import { edgeOfSelection, isFreeShape } from "../../src";
import { selectElement } from "../page";

import { showDeleteButton } from "./show-delete-button";

import type { VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/**
 * Shows the buttons acting on the points: for a regular polygon, the conversion (greyed when it
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
  const isPolygon = "anchor" in model;
  const edge = edgeOfSelection(
    selection.selected,
    reading.corners.length * Math.sign(model.width * model.height),
  );
  const convert = selectElement(document, "convert");

  convert.hidden = !isPolygon;
  convert.toggleAttribute("disabled", model.width === 0 || model.height === 0);
  selectElement(document, "convert-hint").hidden = !isPolygon;

  showDeleteButton(document, reading, selection);

  selectElement(document, "add-midpoint").toggleAttribute(
    "disabled",
    !isFreeShape(model) || edge === undefined,
  );
}
