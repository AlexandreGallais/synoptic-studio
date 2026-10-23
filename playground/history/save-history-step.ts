import { DEFAULT_HISTORY_BOUND, boundedHistory, historyWithStep } from "../../src";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";

import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { PAGE_HISTORIES } from "./page-histories";
import { readRefusedKind } from "./read-refused-kind";

import type { History, HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the history once the page's drawing and selection are recorded: a step when the drawing
 * changed, the selection alone replaced otherwise (`historyWithStep`), the history then bounded
 * (`boundedHistory`, `DEFAULT_HISTORY_BOUND`); with no history yet, one starts from them, nothing
 * past nor to come.
 *
 * @kind procedure
 * @param history - the page's history; undefined before its first redraw
 * @param present - the drawing and the selection the page shows
 * @returns the history recorded
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function readSavedHistory(
  history: History<KeptShape> | undefined,
  present: HistoryState<KeptShape>,
): History<KeptShape> {
  return history === undefined
    ? { future: [], past: [], present }
    : boundedHistory(historyWithStep(history, present), DEFAULT_HISTORY_BOUND);
}

/**
 * Records the drawing and the selection as the page shows them (F40, `readSavedHistory`), the step
 * due, if any, taken; nothing is recorded while the shape the panel edits is refused
 * (`readRefusedKind`), the drawing then kept without it, the step due dropped.
 *
 * @kind procedure
 * @param document - page document, redrawn from the last action
 * @returns the page's history once recorded
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function saveHistoryStep(document: Document): History<KeptShape> {
  const history = PAGE_HISTORIES.get(document);

  DUE_HISTORY_STEPS.delete(document);

  if (history !== undefined && readRefusedKind(document) !== undefined) {
    return history;
  }

  const saved = readSavedHistory(history, {
    drawing: readKeptDrawing(document),
    selection: readSelectedShapes(document),
  });

  PAGE_HISTORIES.set(document, saved);

  return saved;
}
