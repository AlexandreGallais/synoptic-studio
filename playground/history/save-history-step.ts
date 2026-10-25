import { measuredHistory, sizedBoundedHistory, sizedHistoryWithStep } from "../../src";
import { readPageHistoryBound } from "../history-bound";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";

import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { PAGE_HISTORIES } from "./page-histories";
import { readRefusedKind } from "./read-refused-kind";

import type { HistoryBound, HistoryState, SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the history once the page's drawing and selection are recorded, its size kept: a step
 * when the drawing changed, the selection alone replaced otherwise (`sizedHistoryWithStep`), the
 * history then bounded (`sizedBoundedHistory`) by the bound the designer set, a history within it
 * kept by comparing two numbers; with no history yet, one starts from them, nothing past nor to
 * come (`measuredHistory`, ADR-0097).
 *
 * @kind procedure
 * @param history - the page's history; undefined before its first redraw
 * @param present - the drawing and the selection the page shows
 * @param bound - the memory size and the steps kept (`readPageHistoryBound`)
 * @returns the history recorded
 * @see RULE-interaction-7-navigation-and-history
 */
function readSavedHistory(
  history: SizedHistory<KeptShape> | undefined,
  present: HistoryState<KeptShape>,
  bound: HistoryBound,
): SizedHistory<KeptShape> {
  return history === undefined
    ? measuredHistory({ future: [], past: [], present })
    : sizedBoundedHistory(sizedHistoryWithStep(history, present), bound);
}

/**
 * Records the drawing and the selection as the page shows them (`readSavedHistory`), the step
 * due, if any, taken; nothing is recorded while the shape the panel edits is refused
 * (`readRefusedKind`), the drawing then kept without it, the step due dropped.
 *
 * @kind procedure
 * @param document - page document, redrawn from the last action
 * @returns the page's history once recorded
 * @see RULE-interaction-7-navigation-and-history
 */
export function saveHistoryStep(document: Document): SizedHistory<KeptShape> {
  const history = PAGE_HISTORIES.get(document);

  DUE_HISTORY_STEPS.delete(document);

  if (history !== undefined && readRefusedKind(document) !== undefined) {
    return history;
  }

  const present = { drawing: readKeptDrawing(document), selection: readSelectedShapes(document) };
  const saved = readSavedHistory(history, present, readPageHistoryBound(document));

  PAGE_HISTORIES.set(document, saved);

  return saved;
}
