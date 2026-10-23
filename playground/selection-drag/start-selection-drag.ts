import { PICK_TOLERANCE, screenToUser, shapeAtPoint } from "../../src";
import { SELECTION_DRAGS, readPickAreas } from "../hover";
import { VIEW_SCALE } from "../kept-shapes";
import { readMode, selectElement } from "../page";
import { readDrawingPoint } from "../placing";
import { readArmedTool } from "../toolbar";

import type { Point } from "../../src";
import type { SelectionDrag } from "../hover";

/**
 * Reads the point of a press the Selector takes: made with the Selector armed and the main button.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press on the canvas
 * @returns the point pressed, in the coordinates of the drawing's SVG; undefined for another press
 * @see docs/domain/interaction.md#_2-selector
 */
function readSelectorPress(document: Document, event: PointerEvent): Point | undefined {
  return readArmedTool(document) === "selector" && event.button === 0
    ? readDrawingPoint(document, event)
    : undefined;
}

/**
 * Reads the drag a press with the Selector starts: it draws a selection rectangle in Shape mode
 * from an empty place of the drawing, no shape under it as a click reaches one (`shapeAtPoint`,
 * `PICK_TOLERANCE` in user units).
 *
 * @kind procedure
 * @param document - page document
 * @param press - the point pressed, in the coordinates of the drawing's SVG
 * @returns the drag, the pointer on the press, not moved yet
 * @see docs/domain/interaction.md#_2-selector
 */
function readPressedDrag(document: Document, press: Point): SelectionDrag {
  const reach = screenToUser(PICK_TOLERANCE, VIEW_SCALE);
  const isRectangle =
    readMode(document) === "shape" && shapeAtPoint(readPickAreas(document), press, reach) < 0;

  return { isMoved: false, isRectangle, isShift: false, move: undefined, pointer: press, press };
}

/**
 * Starts a press with the Selector on the drawing (F37) when the main button is pressed, a drag
 * left before dropped: kept from the press to the release (`SELECTION_DRAGS`), so that a release
 * off the press's integer point is no click; pressed on an empty place, in Shape mode, it draws a
 * selection rectangle (`readPressedDrag`), the canvas taking the pointer so that the drag goes on
 * beyond its edges; a press on a shape is left for moving it, E02's next feature.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the press on the canvas
 * @see docs/domain/interaction.md#_2-selector
 */
export function startSelectionDrag(document: Document, event: PointerEvent): void {
  SELECTION_DRAGS.delete(document);

  const press = readSelectorPress(document, event);

  if (press === undefined) {
    return;
  }

  const drag = readPressedDrag(document, press);

  if (drag.isRectangle) {
    selectElement(document, "canvas").setPointerCapture(event.pointerId);
  }

  SELECTION_DRAGS.set(document, drag);
}
