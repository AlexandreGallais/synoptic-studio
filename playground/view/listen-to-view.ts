import { selectElement } from "../page";

import { listenToCanvasSize } from "./listen-to-canvas-size";
import { listenToPanDrag } from "./listen-to-pan-drag";
import { listenToWheel } from "./listen-to-wheel";
import { PAGE_VIEWS } from "./page-views";
import { WHEEL_ZOOMS } from "./wheel-zooms";

/** The buttons that start the page again, as it opens: Reset and the guided test's steps. */
const RESTARTS = ["tool-reset", "guide-previous", "guide-next"];

/**
 * Listens to what changes the view of the drawing (ADR-0098, SP-030's answers 2 and 6), the page
 * mounted at the opening view: the canvas's size, whatever changes it (`listenToCanvasSize`,
 * US-084), the view keeping its scale and origin; the wheel
 * and the pinch (`listenToWheel`), a drag that pans (`listenToPanDrag`), wired before the drawing's
 * listeners so that a pan stops them; Reset and the guided test's steps, which bring back the
 * opening view, the zoom asked dropped, the page redrawn by their own listeners.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see RULE-interaction-7-navigation-and-history
 */
export function listenToView(document: Document, update: () => void): void {
  PAGE_VIEWS.delete(document);
  WHEEL_ZOOMS.delete(document);
  listenToCanvasSize(document, update);
  listenToWheel(document, update);
  listenToPanDrag(document, update);

  for (const id of RESTARTS) {
    selectElement(document, id).addEventListener("click", () => {
      PAGE_VIEWS.delete(document);
      WHEEL_ZOOMS.delete(document);
    });
  }
}
