import { selectElement } from "../page";
import { readDrawingPoint } from "../placing";
import { readArmedTool } from "../toolbar";

import { PRESSED_TOOLS } from "./pressed-tools";
import { selectShapeAt } from "./select-shape-at";

/**
 * Listens to clicks on the drawing with the selector (F36): each press keeps the tool armed then
 * (`PRESSED_TOOLS`); a click whose press was made with the selector picks the shape under it, or
 * none on the empty drawing (`selectShapeAt`), then redraws; the click ending a drag or placing a
 * shape with a tool picks nothing, though the selector is armed again by then.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToPicking(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointerdown", () => {
    PRESSED_TOOLS.set(document, readArmedTool(document));
  });

  canvas.addEventListener("click", (event) => {
    const pressed = PRESSED_TOOLS.get(document);

    PRESSED_TOOLS.delete(document);

    if (pressed !== "selector") {
      return;
    }

    selectShapeAt(document, readDrawingPoint(document, event), event.shiftKey);
    update();
  });
}
