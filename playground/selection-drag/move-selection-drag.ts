import { draggedBox } from "../../src";
import { SELECTION_DRAGS } from "../hover";
import { readDrawingPoint } from "../placing";

/**
 * Follows the pointer during a press with the Selector (F37): its point and whether Shift is held
 * kept (F39), the drag moved once the pointer has left the press's integer point (`draggedBox`),
 * even if it comes back; nothing without a press on. What it shows is drawn by
 * `showSelectionDrag`.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the pointer's move or release on the canvas
 * @see docs/domain/interaction.md#_2-selector
 */
export function moveSelectionDrag(document: Document, event: PointerEvent): void {
  const drag = SELECTION_DRAGS.get(document);

  if (drag === undefined) {
    return;
  }

  const pointer = readDrawingPoint(document, event);
  const { height, width } = draggedBox({ isSquare: false, press: drag.press, release: pointer });
  // Width and height are integers ≥ 0: their sum is 0 for the press's integer point alone.
  const isMoved = drag.isMoved || width + height > 0;

  SELECTION_DRAGS.set(document, { ...drag, isMoved, isShift: event.shiftKey, pointer });
}
