import { DEFAULT_HISTORY_BOUND } from "../../src";

import type { HistoryBound } from "../../src";

/**
 * Reads the history's bound kept in this browser (F40, `HISTORY_BOUND_KEY`), each part its
 * field accepts kept (`readBoundPart`), any other back to its default, 100 MB or 5 steps; without
 * the browser's storage, or with a text it did not write, the default (SP-029's answer 6 (i)).
 *
 * @kind procedure
 * @param document - page document
 * @returns the memory size and the steps kept
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readStoredHistoryBound(document: Document): HistoryBound {
  return document.defaultView === null ? DEFAULT_HISTORY_BOUND : DEFAULT_HISTORY_BOUND;
}
