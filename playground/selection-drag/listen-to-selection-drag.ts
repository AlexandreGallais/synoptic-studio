import { selectElement } from "../page";

import { clearSelectionDrag } from "./clear-selection-drag";
import { moveSelectionDrag } from "./move-selection-drag";
import { showSelectionDrag } from "./show-selection-drag";
import { startSelectionDrag } from "./start-selection-drag";
import { stopSelectionDrag } from "./stop-selection-drag";

/**
 * Listens to drags on the drawing with the Selector: the press starts one
 * (`startSelectionDrag`), each move follows the pointer (`moveSelectionDrag`) and shows its
 * rectangle or the shapes it moves (`showSelectionDrag`), the release selects the shapes its
 * rectangle takes or lands those it moves (`stopSelectionDrag`), a drag cancelled selects nothing
 * and gives back the shapes it moved (`clearSelectionDrag`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-2-selector
 */
export function listenToSelectionDrag(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointerdown", (event) => {
    startSelectionDrag(document, event);
  });

  canvas.addEventListener("pointermove", (event) => {
    moveSelectionDrag(document, event);
    showSelectionDrag(document, update);
  });

  canvas.addEventListener("pointerup", (event) => {
    stopSelectionDrag(document, event, update);
  });

  canvas.addEventListener("pointercancel", () => {
    clearSelectionDrag(document, update);
  });
}
