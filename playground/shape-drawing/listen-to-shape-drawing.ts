import { selectElement } from "../page";
import { readDrawingPoint } from "../placing";

import { startShapeDrawing } from "./start-shape-drawing";
import { stopShapeDrawing } from "./stop-shape-drawing";
import { updateShapeDrawing } from "./update-shape-drawing";

/**
 * Listens to Shift pressed or released during a drag (F36): the shape changes at once.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
function listenToShift(document: Document, update: () => void): void {
  document.addEventListener("keydown", (event) => {
    if (event.key === "Shift") {
      updateShapeDrawing(document, { isShift: true }, update);
    }
  });

  document.addEventListener("keyup", (event) => {
    if (event.key === "Shift") {
      updateShapeDrawing(document, { isShift: false }, update);
    }
  });
}

/**
 * Listens to drags on the drawing with a tool armed (F36): the press starts a drag
 * (`startShapeDrawing`); each move and each press or release of Shift redraws the new shape
 * (`updateShapeDrawing`); the release, or a drag cancelled, keeps it (`stopShapeDrawing`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToShapeDrawing(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  /**
   * Draws the shape at the pointer's place, then ends the drag.
   *
   * @param event - the release, or the drag cancelled
   */
  function stop(event: PointerEvent): void {
    updateShapeDrawing(document, { pointer: readDrawingPoint(document, event) }, update);
    stopShapeDrawing(document);
  }

  canvas.addEventListener("pointerdown", (event) => {
    startShapeDrawing(document, event);
  });

  canvas.addEventListener("pointermove", (event) => {
    const pointer = readDrawingPoint(document, event);

    updateShapeDrawing(document, { isShift: event.shiftKey, pointer }, update);
  });

  canvas.addEventListener("pointerup", stop);
  canvas.addEventListener("pointercancel", stop);
  listenToShift(document, update);
}
