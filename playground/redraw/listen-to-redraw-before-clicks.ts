import { selectElement } from "../page";

import { drawPendingRedraw } from "./draw-pending-redraw";
import { PENDING_REDRAWS } from "./pending-redraws";

/**
 * Listens to the pointer released and the clicks on the drawing, before the drawing's own
 * listeners, in the capture phase (`REF-MDN-ADD-EVENT-LISTENER`, `capture`), so that neither a
 * listener of the canvas added before it nor one inside it acts first, to draw the
 * redraw asked and not drawn yet (`drawPendingRedraw`): a value
 * typed then applied as the drawing is pressed, the field left, is drawn into its shape before the
 * release or the click selects another, places a shape or selects none, even within the same
 * frame (note 0032 §10). The page being mounted, the redraw the mount before asked and did not draw
 * is forgotten, the mount drawing the page itself, so that it never stands for the
 * redraws this mount asks.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-7-navigation-and-history
 */
export function listenToRedrawBeforeClicks(document: Document): void {
  const canvas = selectElement(document, "canvas");

  PENDING_REDRAWS.delete(document);

  for (const type of ["pointerup", "click"]) {
    canvas.addEventListener(
      type,
      () => {
        drawPendingRedraw(document);
      },
      { capture: true },
    );
  }
}
