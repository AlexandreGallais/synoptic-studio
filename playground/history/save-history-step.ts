import { boundedHistory, historyWithStep } from "../../src";
import { readPageHistoryBound } from "../history-bound";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";

import { DUE_HISTORY_STEPS } from "./due-history-steps";
import { PAGE_HISTORIES } from "./page-histories";
import { readRefusedKind } from "./read-refused-kind";

import type { History, HistoryBound, HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The bound each history the page recorded was last bounded by, by the history itself, so that a
 * record that took no step, the past and the states to come the same lists, is not measured
 * again while the bound is the same (`readSavedHistory`).
 */
const BOUNDED_HISTORIES = new WeakMap<History<KeptShape>, HistoryBound>();

/**
 * Reads the history once the page's drawing and selection are recorded: a step when the drawing
 * changed, the selection alone replaced otherwise (`historyWithStep`), the history then bounded
 * (`boundedHistory`) by the bound the designer set, unless it took no step and was bounded by
 * that bound already (`BOUNDED_HISTORIES`), its size then the same; with no history yet, one
 * starts from them, nothing past nor to come.
 *
 * @kind procedure
 * @param history - the page's history; undefined before its first redraw
 * @param present - the drawing and the selection the page shows
 * @param bound - the memory size and the steps kept (`readPageHistoryBound`)
 * @returns the history recorded
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function readSavedHistory(
  history: History<KeptShape> | undefined,
  present: HistoryState<KeptShape>,
  bound: HistoryBound,
): History<KeptShape> {
  if (history === undefined) {
    return { future: [], past: [], present };
  }

  const stepped = historyWithStep(history, present);
  const isBounded = [
    BOUNDED_HISTORIES.get(history) === bound,
    stepped.past === history.past,
    stepped.future === history.future,
  ].every(Boolean);
  const saved = isBounded ? stepped : boundedHistory(stepped, bound);

  BOUNDED_HISTORIES.set(saved, bound);

  return saved;
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

  const present = { drawing: readKeptDrawing(document), selection: readSelectedShapes(document) };
  const saved = readSavedHistory(history, present, readPageHistoryBound(document));

  PAGE_HISTORIES.set(document, saved);

  return saved;
}
