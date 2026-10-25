import { sizedHistoryAt, sizedRedoneHistory, sizedUndoneHistory } from "../../src";
import { listenToHistoryBound } from "../history-bound";
import { readStepState } from "../history-list";
import { selectElement } from "../page";

import { clearPageHistory } from "./clear-page-history";
import { listenToHistoryKeys } from "./listen-to-history-keys";
import { listenToPanelSteps } from "./listen-to-panel-steps";
import { moveHistory } from "./move-history";
import { scheduleHistoryStep } from "./schedule-history-step";

/** The buttons that start the page again, as it opens: Reset and the guided test's steps. */
const RESTARTS = ["tool-reset", "guide-previous", "guide-next"];

/**
 * Listens to the ends of the drawing's actions: a step asked at each click on the drawing,
 * each release of the pointer on it, a drag cancelled included (`scheduleHistoryStep`); a key
 * released asks its own, with the keys of the history (`listenToHistoryKeys`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-7-navigation-and-history
 */
function listenToActionEnds(document: Document, update: () => void): void {
  for (const type of ["click", "pointerup", "pointercancel"]) {
    selectElement(document, "canvas").addEventListener(type, () => {
      scheduleHistoryStep(document, update);
    });
  }
}

/**
 * Listens to what moves the page's history: the Undo and Redo buttons (`moveHistory`), and a
 * click on a step of the list, going back or forward to the state after it, as many undos or
 * redos at once (`readStepState`, `sizedHistoryAt`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-7-navigation-and-history
 */
function listenToHistoryMoves(document: Document, update: () => void): void {
  for (const [id, move] of [
    ["history-undo", sizedUndoneHistory],
    ["history-redo", sizedRedoneHistory],
  ] as const) {
    selectElement(document, id).addEventListener("click", () => {
      moveHistory(document, move, update);
    });
  }

  selectElement(document, "history-list").addEventListener("click", (event) => {
    const state = readStepState(event);

    if (state !== undefined) {
      moveHistory(document, (history) => sizedHistoryAt(history, state), update);
    }
  });
}

/**
 * Listens to what records, undoes, redoes and bounds the steps of the page, its history
 * emptied first, the page being mounted: the fields of its bound, read from the browser, a bound
 * applied asking a step that the drawing unchanged does not take, but that bounds the history by
 * it at once (`listenToHistoryBound`, `scheduleHistoryStep`); the ends of the drawing's actions (`listenToActionEnds`) and of
 * the panel's changes (`listenToPanelSteps`); the Undo and Redo buttons and the list of steps
 * (`listenToHistoryMoves`); Reset
 * and the guided test's Previous and Next, which empty the history (`clearPageHistory`), then
 * redraw; the keys of the history (`listenToHistoryKeys`). Wired after the drawing's listeners and
 * the toolbar's, so that a step follows the action.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-7-navigation-and-history
 */
export function listenToHistory(document: Document, update: () => void): void {
  clearPageHistory(document);

  listenToHistoryBound(document, () => {
    scheduleHistoryStep(document, update);
  });

  listenToActionEnds(document, update);
  listenToHistoryMoves(document, update);

  for (const id of RESTARTS) {
    selectElement(document, id).addEventListener("click", () => {
      clearPageHistory(document);
      update();
    });
  }

  listenToPanelSteps(document);
  listenToHistoryKeys(document, update);
}
