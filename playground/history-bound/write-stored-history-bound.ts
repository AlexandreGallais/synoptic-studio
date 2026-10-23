import { readBrowserStorage } from "../checklist";

import { HISTORY_BOUND_KEY } from "./history-bound-key";

import type { HistoryBound } from "../../src";

/**
 * Keeps the history's bound in this browser for the next session (F40, `HISTORY_BOUND_KEY`);
 * without the browser's storage, or with a storage full, nothing is kept and the page goes on,
 * the bound kept until the page is closed.
 *
 * @kind procedure
 * @param document - page document
 * @param bound - the memory size and the steps kept
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function writeStoredHistoryBound(document: Document, bound: HistoryBound): void {
  readBrowserStorage(document)?.setItem(HISTORY_BOUND_KEY, String(bound.size));
}
