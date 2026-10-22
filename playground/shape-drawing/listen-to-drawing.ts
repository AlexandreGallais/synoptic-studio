import { listenToPlacing } from "../placing";

import { listenToShapeDrawing } from "./listen-to-shape-drawing";

/**
 * Listens to the pointer on the drawing with a tool armed: a click places the tool's shape
 * (`listenToPlacing`, F35), a drag draws its box (`listenToShapeDrawing`, F36).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToDrawing(document: Document, update: () => void): void {
  listenToShapeDrawing(document, update);
  listenToPlacing(document, update);
}
