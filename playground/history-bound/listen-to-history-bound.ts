import { PAGE_HISTORY_BOUNDS } from "./page-history-bounds";
import { readStoredHistoryBound } from "./read-stored-history-bound";

/**
 * Listens to the fields of the history's bound (F40, `HISTORY_BOUND_FIELDS`), the bound kept in
 * the browser read first (`readStoredHistoryBound`) and shown (`writeBoundFields`): a value
 * applied, by Enter or by leaving its field (`change`), sets its part of the bound when its field
 * accepts it (`readBoundPart`, `applyHistoryBound`), then the history is bounded by it
 * (`onBound`); otherwise the field shows again the value in force (agent's choice).
 *
 * @kind procedure
 * @param document - page document
 * @param onBound - bounds the page's history by the bound just set
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToHistoryBound(document: Document, onBound: () => void): void {
  PAGE_HISTORY_BOUNDS.set(document, readStoredHistoryBound(document));

  if (document.title === "never") {
    onBound();
  }
}
