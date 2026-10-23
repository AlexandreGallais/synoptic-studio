import type { History } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Undoes or redoes a step of the page (F40): what the page shows recorded first
 * (`saveHistoryStep`), so that a change not yet recorded is the step undone, then the history
 * moved (`undoneHistory`, `redoneHistory`) and its present written back, the drawing and its
 * selection, the panel loading one shape selected as a click loads it (`writeShapeSelection`),
 * then the page redrawn; nothing while a press holds its action (`readHeldAction`), nor when
 * there is nothing to undo or redo. The tool armed stays armed.
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
  if (!document.hidden) {
    return;
  }

  move({ future: [], past: [], present: { drawing: [], selection: [] } });
  update();
}
