import { selectElement } from "../page";
import { readArmedTool } from "../toolbar";

import { addPlacedShape } from "./add-placed-shape";
import { readDrawingPoint } from "./read-drawing-point";

/**
 * Listens to clicks on the drawing: with a tool armed, a click places its shape there
 * (`addPlacedShape`, F35), as does a drag released on the press's integer point (F36); with the
 * selector, it does nothing yet.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function listenToPlacing(document: Document, update: () => void): void {
  selectElement(document, "canvas").addEventListener("click", (event) => {
    const tool = readArmedTool(document);

    if (tool === "selector") {
      return;
    }

    addPlacedShape(document, { point: readDrawingPoint(document, event), tool });
    update();
  });
}
