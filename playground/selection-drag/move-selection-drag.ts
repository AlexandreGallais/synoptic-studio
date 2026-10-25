import { isDragged } from "../../src";
import { SELECTION_DRAGS } from "../hover";
import { readDrawingPoint } from "../placing";
import { readView } from "../view";

/**
 * Follows the pointer during a press with the Selector: its point and whether Shift and Alt
 * are held kept, the drag moved once the pointer has gone further than 3 pixels from the press at the
 * view's scale (`isDragged`, SP-030's answer 8), even if it comes back; nothing without a press on. What it shows is drawn by
 * `showSelectionDrag`.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the pointer's move or release on the canvas
 * @see RULE-interaction-2-selector
 */
export function moveSelectionDrag(document: Document, event: PointerEvent): void {
  const drag = SELECTION_DRAGS.get(document);

  if (drag === undefined) {
    return;
  }

  const pointer = readDrawingPoint(document, event);
  const isMoved = drag.isMoved || isDragged(drag.press, pointer, readView(document).scale);

  SELECTION_DRAGS.set(document, {
    ...drag,
    isAlt: event.altKey,
    isMoved,
    isShift: event.shiftKey,
    pointer,
  });
}
