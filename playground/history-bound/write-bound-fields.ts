import { writeInputValue } from "../page";

import { HISTORY_BOUND_FIELDS } from "./history-bound-fields";

import type { HistoryBound } from "../../src";

/**
 * Shows a bound in its fields (`HISTORY_BOUND_FIELDS`): the memory size in MB, the steps
 * kept, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param bound - the memory size and the steps kept
 */
export function writeBoundFields(document: Document, bound: HistoryBound): void {
  const { size, steps } = HISTORY_BOUND_FIELDS;

  writeInputValue(document, size.id, bound.size / size.unit);
  writeInputValue(document, steps.id, bound.steps / steps.unit);
}
