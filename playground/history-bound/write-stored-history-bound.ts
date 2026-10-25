import { HISTORY_BOUND_KEY } from "./history-bound-key";
import { readBrowserStorage } from "./read-browser-storage";

import type { HistoryBound } from "../../src";

/**
 * Keeps the history's bound in this browser for the next session (`HISTORY_BOUND_KEY`);
 * without the browser's storage, or with a storage full, nothing is kept and the page goes on,
 * the bound kept until the page is closed.
 *
 * @kind procedure
 * @param document - page document
 * @param bound - the memory size and the steps kept
 */
export function writeStoredHistoryBound(document: Document, bound: HistoryBound): void {
  const storage = readBrowserStorage(document);

  if (storage === undefined) {
    return;
  }

  const written = JSON.stringify({ size: bound.size, steps: bound.steps });

  try {
    storage.setItem(HISTORY_BOUND_KEY, written);
  } catch {
    // The storage is a convenience: the bound holds for the session without it.
  }
}
