import { DUE_HISTORY_STEPS } from "./due-history-steps";

/**
 * Asks a step once the page is redrawn from the action that just finished (F40): a click on the
 * drawing, a release of the pointer, a key released outside the panel's fields; the redraw takes
 * it (`updateHistory`), after the action's own, so that the shape the panel edits is written back
 * as last drawn first (ADR-0095, decision 3).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-7-navigation-and-history
 */
export function scheduleHistoryStep(document: Document, update: () => void): void {
  DUE_HISTORY_STEPS.add(document);
  update();
}
