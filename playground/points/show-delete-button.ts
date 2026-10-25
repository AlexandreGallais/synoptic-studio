import { deletionOutcome, isFreeShape } from "../../src";
import { selectElement } from "../page";

import type { DeletionOutcome, VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/** Line shown under the Delete button for each outcome; `none` when nothing can be deleted. */
const DELETION_MESSAGES: Readonly<Record<DeletionOutcome | "none", string>> = {
  allowed: "",
  flat: "The points left would have no width or no height: select others to delete.",
  none: "",
  "too-few": "A shape keeps at least 3 points: select fewer to delete.",
};

/**
 * Reads what deleting the points selected would give: `none` on a regular polygon or a round
 * shape (converted first) or with no point selected.
 *
 * @kind procedure
 * @param reading - valid shape
 * @param selection - vertices selected
 * @returns the outcome of the deletion, or `none`
 */
function readDeletionOutcome(
  reading: ShapeReading,
  selection: VertexSelection,
): DeletionOutcome | "none" {
  const { model } = reading;

  return isFreeShape(model) && selection.selected.length > 0
    ? deletionOutcome(model, selection.selected)
    : "none";
}

/**
 * Shows the Delete button, greyed unless the deletion is allowed, with a line saying why when the
 * points left would not make a shape (F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @param selection - vertices selected
 */
export function showDeleteButton(
  document: Document,
  reading: ShapeReading,
  selection: VertexSelection,
): void {
  const outcome = readDeletionOutcome(reading, selection);
  const message = selectElement(document, "delete-message");

  message.textContent = DELETION_MESSAGES[outcome];
  message.hidden = DELETION_MESSAGES[outcome] === "";
  selectElement(document, "delete-points").toggleAttribute("disabled", outcome !== "allowed");
}
