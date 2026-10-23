import { selectElement } from "../page";
import { CLICKED_PRESSES, readDrawingPoint } from "../placing";
import { readArmedTool } from "../toolbar";

import { SHAPE_DRAWINGS } from "./shape-drawings";

/**
 * Starts a drag on the drawing (F36) when the main button is pressed with a tool armed, the point
 * of a press before dropped: the canvas takes the pointer, so that the drag goes on beyond its
 * edges; nothing is drawn until the pointer goes further than 3 pixels from the press
 * (`showShapeDrawing`, SP-030's answer 8).
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press on the canvas
 * @see docs/domain/interaction.md#_1-tools
 */
export function startShapeDrawing(document: Document, event: PointerEvent): void {
  const tool = readArmedTool(document);
  const press = readDrawingPoint(document, event);

  CLICKED_PRESSES.delete(document);

  if (tool === "selector" || event.button !== 0) {
    return;
  }

  selectElement(document, "canvas").setPointerCapture(event.pointerId);

  SHAPE_DRAWINGS.set(document, {
    isDrawn: false,
    isShift: event.shiftKey,
    pointer: press,
    press,
    tool,
  });
}
