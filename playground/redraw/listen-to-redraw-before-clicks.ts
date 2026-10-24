import { selectElement } from "../page";

import { drawPendingRedraw } from "./draw-pending-redraw";

/**
 * Listens to the pointer released and the clicks on the drawing, before the drawing's own
 * listeners, to draw the redraw asked and not drawn yet (`drawPendingRedraw`, US-082): a value
 * typed then applied as the drawing is pressed, the field left, is drawn into its shape before the
 * release or the click selects another, places a shape or selects none, even within the same
 * frame (note 0032 §10).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToRedrawBeforeClicks(document: Document): void {
  const canvas = selectElement(document, "canvas");

  for (const type of ["pointerup", "click"]) {
    canvas.addEventListener(type, () => {
      drawPendingRedraw(document);
    });
  }
}
