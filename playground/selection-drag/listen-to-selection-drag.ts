import { selectElement } from "../page";
import { readDrawingPoint } from "../placing";

import { clearSelectionDrag } from "./clear-selection-drag";
import { moveSelectionDrag } from "./move-selection-drag";
import { startSelectionDrag } from "./start-selection-drag";
import { stopSelectionDrag } from "./stop-selection-drag";

/**
 * Listens to drags on the drawing with the Selector (F37): the press starts one
 * (`startSelectionDrag`), each move follows the pointer (`moveSelectionDrag`), the release selects
 * the shapes its rectangle takes (`stopSelectionDrag`), a drag cancelled selects nothing
 * (`clearSelectionDrag`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToSelectionDrag(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointerdown", (event) => {
    startSelectionDrag(document, event);
  });

  canvas.addEventListener("pointermove", (event) => {
    moveSelectionDrag(document, readDrawingPoint(document, event));
  });

  canvas.addEventListener("pointerup", (event) => {
    stopSelectionDrag(document, event, update);
  });

  canvas.addEventListener("pointercancel", () => {
    clearSelectionDrag(document);
  });
}
