import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { PAGE_HISTORIES } from "./page-histories";

/**
 * Empties the page's history (F40), at Reset, at a step of the guided test and when the page is
 * mounted: the next redraw starts it again from the page as it then is, nothing to undo nor to
 * redo (`updateHistory`); a step due is dropped with it.
 *
 * @kind procedure
 * @param document - page document, its history then dropped
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function clearPageHistory(document: Document): void {
  PAGE_HISTORIES.delete(document);
  DUE_HISTORY_STEPS.delete(document);
}
