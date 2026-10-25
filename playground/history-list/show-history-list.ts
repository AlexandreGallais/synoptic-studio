import { selectElement } from "../page";

import { createHistoryRows } from "./create-history-rows";
import { SHOWN_HISTORIES } from "./shown-histories";

import type { SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Shows the list of the history's steps, unless it shows that history already
 * (`SHOWN_HISTORIES`): its rows written again (`createHistoryRows`); no history, no row.
 *
 * @kind procedure
 * @param document - page document
 * @param history - the page's history; undefined before its first redraw
 */
export function showHistoryList(
  document: Document,
  history: SizedHistory<KeptShape> | undefined,
): void {
  if (SHOWN_HISTORIES.get(document) === history) {
    return;
  }

  SHOWN_HISTORIES.set(document, history);
  selectElement(document, "history-list").replaceChildren(...createHistoryRows(document, history));
}
