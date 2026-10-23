import { listenToPlacing } from "../placing";
import { listenToPicking } from "../selection";
import { listenToSelectionDrag } from "../selection-drag";
import { listenToNudgeKeys, listenToSelectionKeys } from "../selection-keys";

import { listenToShapeDrawing } from "./listen-to-shape-drawing";

/**
 * Listens to the pointer on the drawing: with a tool armed, a click places the tool's shape
 * (`listenToPlacing`, F35), a drag draws its box (`listenToShapeDrawing`, F36); with the
 * selector, a click picks the shape under it (`listenToPicking`, F36), Shift adding it or taking
 * it out (F37), and a drag from an empty place selects the shapes its rectangle takes
 * (`listenToSelectionDrag`, F37); and to the keys of the selection, Ctrl + A, Escape and Delete (`listenToSelectionKeys`), and
 * the arrows, which nudge the shapes selected (`listenToNudgeKeys`, F39).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToDrawing(document: Document, update: () => void): void {
  listenToShapeDrawing(document, update);
  listenToPlacing(document, update);
  listenToPicking(document, update);
  listenToSelectionDrag(document, update);
  listenToSelectionKeys(document, update);
  listenToNudgeKeys(document, update);
}
