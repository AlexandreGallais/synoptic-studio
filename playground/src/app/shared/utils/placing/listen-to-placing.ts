import { selectElement } from "../page";
import { readArmedTool } from "../toolbar";

import { addPlacedShape } from "./add-placed-shape";
import { CLICKED_PRESSES } from "./clicked-presses";
import { readDrawingPoint } from "./read-drawing-point";

/**
 * Listens to clicks on the drawing: with a tool armed, a click places its shape there
 * (`addPlacedShape`), centred on the point pressed when the pointer stayed within 3 pixels of
 * it (`CLICKED_PRESSES`, SP-030's answer 8), on the click's point for a click without a press; with
 * the selector, it places nothing, the shape under it picked instead (`listenToPicking`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 */
export function listenToPlacing(document: Document, update: () => void): void {
  selectElement(document, "canvas").addEventListener("click", (event) => {
    const tool = readArmedTool(document);
    const press = CLICKED_PRESSES.get(document);

    CLICKED_PRESSES.delete(document);

    if (tool === "selector") {
      return;
    }

    addPlacedShape(document, { point: press ?? readDrawingPoint(document, event), tool });
    update();
  });
}
