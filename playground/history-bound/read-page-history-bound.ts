import { DEFAULT_HISTORY_BOUND } from "../../src";

import { PAGE_HISTORY_BOUNDS } from "./page-history-bounds";

import type { HistoryBound } from "../../src";

/**
 * Reads the bound the page's history is kept within (`PAGE_HISTORY_BOUNDS`): the one the
 * designer set, or the default, 100 MB and 5 steps, before the page is mounted.
 *
 * @kind procedure
 * @param document - the playground's document
 * @returns the memory size and the steps kept whatever their size
 * @see RULE-interaction-7-navigation-and-history
 */
export function readPageHistoryBound(document: Document): HistoryBound {
  return PAGE_HISTORY_BOUNDS.get(document) ?? DEFAULT_HISTORY_BOUND;
}
