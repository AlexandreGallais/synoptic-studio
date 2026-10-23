import { clearPageHistory } from "./clear-page-history";
import { listenToHistoryKeys } from "./listen-to-history-keys";

/**
 * Listens to what records, undoes and redoes the steps of the page (F40), its history emptied
 * first, the page being mounted: a step asked at each click on the drawing, each release of the
 * pointer on it, a drag cancelled included, and each key released outside the panel's fields
 * (`scheduleHistoryStep`), so that a key held is one step from its press to its release (SP-029's
 * answer 3); the Undo and Redo buttons (`moveHistory`); Reset and the guided test's Previous and
 * Next, which empty the history (`clearPageHistory`); the keys of the history
 * (`listenToHistoryKeys`). Wired after the drawing's listeners and the toolbar's, so that a step
 * follows the action.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToHistory(document: Document, update: () => void): void {
  clearPageHistory(document);
  listenToHistoryKeys(document, update);
}
