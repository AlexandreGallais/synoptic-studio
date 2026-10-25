import { PAGE_HISTORY_BOUNDS } from "./page-history-bounds";
import { writeBoundFields } from "./write-bound-fields";
import { writeStoredHistoryBound } from "./write-stored-history-bound";

import type { HistoryBound } from "../../src";

/**
 * Applies a bound the designer set: kept for the page (`PAGE_HISTORY_BOUNDS`) and in the
 * browser for the next session (`writeStoredHistoryBound`), its fields showing it
 * (`writeBoundFields`).
 *
 * @kind procedure
 * @param document - page document
 * @param bound - the memory size and the steps kept
 * @see RULE-interaction-7-navigation-and-history
 */
export function applyHistoryBound(document: Document, bound: HistoryBound): void {
  PAGE_HISTORY_BOUNDS.set(document, bound);
  writeStoredHistoryBound(document, bound);
  writeBoundFields(document, bound);
}
