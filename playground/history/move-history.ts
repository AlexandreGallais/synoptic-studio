import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { writeShapeSelection } from "../selection";

import { PAGE_HISTORIES } from "./page-histories";
import { readHeldAction } from "./read-held-action";
import { saveHistoryStep } from "./save-history-step";

import type { History, HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Writes a state of the history back into the page (F40): the panel leaves the shape it edits,
 * keeping its settings as the last of its kind, as Escape does; the state's drawing is written
 * below, then its selection made, one shape loaded into the panel as a click loads it, several
 * kept by their indices (`writeShapeSelection`).
 *
 * @kind procedure
 * @param document - page document
 * @param state - the drawing and the selection to show
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function writeHistoryState(document: Document, state: HistoryState<KeptShape>): void {
  writeShapeSelection(document, []);
  writeStoredDrawing(document, DRAWING_KEYS.below, state.drawing);
  writeShapeSelection(document, state.selection);
}

/**
 * Undoes or redoes a step of the page (F40): what the page shows recorded first
 * (`saveHistoryStep`), so that a change not yet recorded is the step undone, then the history
 * moved (`undoneHistory`, `redoneHistory`) and its present written back, the drawing and its
 * selection (`writeHistoryState`), then the page redrawn; nothing while a press holds its action
 * (`readHeldAction`), nor when there is nothing to undo or redo. The tool armed stays armed.
 *
 * @kind procedure
 * @param document - page document
 * @param move - `undoneHistory` or `redoneHistory`
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function moveHistory(
  document: Document,
  move: (history: History<KeptShape>) => History<KeptShape>,
  update: () => void,
): void {
  if (readHeldAction(document) !== undefined) {
    return;
  }

  const saved = saveHistoryStep(document);
  const moved = move(saved);

  if (moved === saved) {
    return;
  }

  PAGE_HISTORIES.set(document, moved);
  writeHistoryState(document, moved.present);
  update();
}
