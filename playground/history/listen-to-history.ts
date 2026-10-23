import { redoneHistory, undoneHistory } from "../../src";
import { selectElement } from "../page";
import { readKeyField } from "../selection-keys";

import { clearPageHistory } from "./clear-page-history";
import { listenToHistoryKeys } from "./listen-to-history-keys";
import { listenToPanelSteps } from "./listen-to-panel-steps";
import { moveHistory } from "./move-history";
import { scheduleHistoryStep } from "./schedule-history-step";

/** The buttons that start the page again, as it opens: Reset and the guided test's steps. */
const RESTARTS = ["tool-reset", "guide-previous", "guide-next"];

/**
 * Listens to the ends of the drawing's actions (F40): a step asked at each click on the drawing,
 * each release of the pointer on it, a drag cancelled included, and each key released outside the
 * panel's fields (`scheduleHistoryStep`), so that a key held is one step from its press to its
 * release (SP-029's answer 3).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function listenToActionEnds(document: Document, update: () => void): void {
  for (const type of ["click", "pointerup", "pointercancel"]) {
    selectElement(document, "canvas").addEventListener(type, () => {
      scheduleHistoryStep(document, update);
    });
  }

  document.addEventListener("keyup", (event) => {
    if (readKeyField(event) === undefined) {
      scheduleHistoryStep(document, update);
    }
  });
}

/**
 * Listens to what records, undoes and redoes the steps of the page (F40), its history emptied
 * first, the page being mounted: the ends of the drawing's actions (`listenToActionEnds`) and of
 * the panel's changes (`listenToPanelSteps`); the Undo and Redo buttons (`moveHistory`); Reset
 * and the guided test's Previous and Next, which empty the history (`clearPageHistory`), then
 * redraw; the keys of the history (`listenToHistoryKeys`). Wired after the drawing's listeners and
 * the toolbar's, so that a step follows the action.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToHistory(document: Document, update: () => void): void {
  clearPageHistory(document);
  listenToActionEnds(document, update);

  for (const [id, move] of [
    ["history-undo", undoneHistory],
    ["history-redo", redoneHistory],
  ] as const) {
    selectElement(document, id).addEventListener("click", () => {
      moveHistory(document, move, update);
    });
  }

  for (const id of RESTARTS) {
    selectElement(document, id).addEventListener("click", () => {
      clearPageHistory(document);
      update();
    });
  }

  listenToPanelSteps(document);
  listenToHistoryKeys(document, update);
}
