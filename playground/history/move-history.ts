import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { stopNewText } from "../new-text";
import { writeShapeSelection } from "../selection";

import { PAGE_HISTORIES } from "./page-histories";
import { readHeldAction } from "./read-held-action";
import { readRefusedKind } from "./read-refused-kind";
import { saveHistoryStep } from "./save-history-step";

import type { HistoryState, SizedHistory } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Writes a state of the history back into the page: the panel leaves the shape it edits,
 * keeping its settings as the last of its kind, as Escape does; the state's drawing is written
 * below, then its selection made, one shape loaded into the panel as a click loads it, several
 * kept by their indices (`writeShapeSelection`).
 *
 * @kind procedure
 * @param document - page document
 * @param state - the drawing and the selection to show
 * @see RULE-interaction-7-navigation-and-history
 */
function writeHistoryState(document: Document, state: HistoryState<KeptShape>): void {
  writeShapeSelection(document, []);
  writeStoredDrawing(document, DRAWING_KEYS.below, state.drawing);
  writeShapeSelection(document, state.selection);
}

/**
 * Reads the history an undo or a redo leaves: a new text's typing ended first
 * (`stopNewText`), so that a text typed is the step undone and one still empty is dropped; then
 * what the page shows recorded (`saveHistoryStep`), then the history moved; with a shape refused
 * (`readRefusedKind`), or a new text dropped, the history as recorded, unmoved, so that its
 * present gives the shape back with its last valid values, or the drawing before the text.
 *
 * @kind procedure
 * @param document - page document
 * @param move - `sizedUndoneHistory`, `sizedRedoneHistory`, or `sizedHistoryAt` to a state
 * @returns the history whose present to write back; undefined with nothing to undo or redo
 * @see RULE-interaction-7-navigation-and-history
 */
function readMovedHistory(
  document: Document,
  move: (history: SizedHistory<KeptShape>) => SizedHistory<KeptShape>,
): SizedHistory<KeptShape> | undefined {
  stopNewText(document);

  const isRefused = readRefusedKind(document) !== undefined;
  const saved = saveHistoryStep(document);

  if (isRefused) {
    return saved;
  }

  const moved = move(saved);

  return moved === saved ? undefined : moved;
}

/**
 * Undoes or redoes a step of the page: what the page shows recorded first
 * (`saveHistoryStep`), so that a change not yet recorded is the step undone, then the history
 * moved (`readMovedHistory`, `sizedUndoneHistory`, `sizedRedoneHistory`) and its present written back, the drawing and its
 * selection (`writeHistoryState`), then the page redrawn; nothing while a press holds its action
 * (`readHeldAction`), nor when there is nothing to undo or redo. A shape refused
 * (`readRefusedKind`) is given back with its last valid values instead, the present written back,
 * the history unmoved. The tool armed stays armed.
 *
 * @kind procedure
 * @param document - page document
 * @param move - `sizedUndoneHistory`, `sizedRedoneHistory`, or `sizedHistoryAt` to a state
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-7-navigation-and-history
 */
export function moveHistory(
  document: Document,
  move: (history: SizedHistory<KeptShape>) => SizedHistory<KeptShape>,
  update: () => void,
): void {
  if (readHeldAction(document) !== undefined) {
    return;
  }

  const moved = readMovedHistory(document, move);

  if (moved === undefined) {
    return;
  }

  PAGE_HISTORIES.set(document, moved);
  writeHistoryState(document, moved.present);
  update();
}
