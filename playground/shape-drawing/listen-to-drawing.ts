import { listenToPlacing } from "../placing";
import { listenToPicking } from "../selection";
import { listenToSelectionDrag } from "../selection-drag";
import { listenToNudgeKeys, listenToSelectionKeys, listenToSpace } from "../selection-keys";
import { listenToView } from "../view";
import { listenToZoomKeys, listenToZoomMenu } from "../zoom";

import { listenToShapeDrawing } from "./listen-to-shape-drawing";

/**
 * Listens to the pointer on the drawing: first to what changes the view, the wheel, a pinch and a
 * drag that pans, which stops the listeners after it (`listenToView`); with a tool armed, a
 * click places the tool's shape (`listenToPlacing`), a drag draws its box
 * (`listenToShapeDrawing`); with the selector, a click picks the shape under it
 * (`listenToPicking`), Shift adding it or taking it out, and a drag from an empty place
 * selects the shapes its rectangle takes (`listenToSelectionDrag`); and to the keys of the
 * selection, Ctrl + A, Escape and Delete (`listenToSelectionKeys`), the arrows, which nudge the
 * shapes selected (`listenToNudgeKeys`), `Space` held, which makes a drag pan
 * (`listenToSpace`), and the zoom's keys and the percentage's menu (`listenToZoomKeys`,
 * `listenToZoomMenu`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 */
export function listenToDrawing(document: Document, update: () => void): void {
  listenToView(document, update);
  listenToShapeDrawing(document, update);
  listenToPlacing(document, update);
  listenToPicking(document, update);
  listenToSelectionDrag(document, update);
  listenToSelectionKeys(document, update);
  listenToNudgeKeys(document, update);
  listenToSpace(document);
  listenToZoomKeys(document, update);
  listenToZoomMenu(document, update);
}
