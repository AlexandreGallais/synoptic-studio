import { HISTORY_LIST_ROWS, historySteps } from "../../src";

import { createLeftOutRows } from "./create-left-out-rows";
import { createStepRow } from "./create-step-row";
import { readKeptShapeFields } from "./read-kept-shape-fields";

import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Creates the rows of the list of the history's steps: the steps around the present
 * (`historySteps`, `HISTORY_LIST_ROWS`), each a button named by its action
 * (`readKeptShapeFields`, `createStepRow`), and a line for the steps left out before and after
 * them (`createLeftOutRows`).
 *
 * @kind procedure
 * @param document - page document
 * @param history - the page's history; undefined before its first redraw
 * @returns the list's items, the oldest first; none without a history
 * @see RULE-interaction-7-navigation-and-history
 */
export function createHistoryRows(
  document: Document,
  history: SizedHistory<KeptShape> | undefined,
): readonly HTMLLIElement[] {
  if (history === undefined) {
    return [];
  }

  const { earlier, later, rows } = historySteps(history, HISTORY_LIST_ROWS, readKeptShapeFields);

  return [
    ...createLeftOutRows(document, earlier, "earlier"),
    ...rows.map((row) => createStepRow(document, row)),
    ...createLeftOutRows(document, later, "later"),
  ];
}
