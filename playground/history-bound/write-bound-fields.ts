import { writeInputValue } from "../page";

import type { HistoryBound } from "../../src";

/**
 * Shows a bound in its fields (F40, `HISTORY_BOUND_FIELDS`): the memory size in MB, the steps
 * kept, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param bound - the memory size and the steps kept
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function writeBoundFields(document: Document, bound: HistoryBound): void {
  writeInputValue(document, "history-memory", bound.steps);
}
