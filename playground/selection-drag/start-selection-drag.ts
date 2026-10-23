import { PICK_TOLERANCE, screenToUser, shapeAtPoint } from "../../src";
import { SELECTION_DRAGS, readPickAreas } from "../hover";
import { VIEW_SCALE } from "../kept-shapes";
import { readMode, selectElement } from "../page";
import { readDrawingPoint } from "../placing";
import { readArmedTool } from "../toolbar";

import { readShapesMove } from "./read-shapes-move";

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
 * Reads the drag a press with the Selector starts in Shape mode, by what is under it as a click
 * reaches it (`shapeAtPoint`, `PICK_TOLERANCE` in user units): from an empty place, a selection
 * rectangle (F37), inside the dashed box of several shapes too; from a shape, a move of the shapes
 * selected (`readShapesMove`, F39). In Points mode, which edits one shape's points, neither.
 *
 * @kind procedure
 * @param document - page document
 * @param press - the point pressed, in the coordinates of the drawing's SVG
 * @param event - the press, telling whether Shift is held
 * @returns the drag, the pointer on the press, not moved yet
 * @see docs/domain/interaction.md#_2-selector
 */
function readPressedDrag(document: Document, press: Point, event: PointerEvent): SelectionDrag {
  const isShape = readMode(document) === "shape";
  const reach = screenToUser(PICK_TOLERANCE, VIEW_SCALE);
  const index = isShape ? shapeAtPoint(readPickAreas(document), press, reach) : -1;
  const drag = { isMoved: false, isShift: event.shiftKey, pointer: press, press };

  return index < 0
    ? { ...drag, isRectangle: isShape, move: undefined }
    : { ...drag, isRectangle: false, move: readShapesMove(document, index, event.shiftKey) };
}

/**
 * Makes the canvas take the pointer for a selection rectangle or a move, so that the drag goes on
 * beyond its edges; a press that does neither leaves the pointer free.
 *
 * @kind procedure
 * @param document - page document
 * @param drag - the drag the press starts
 * @param pointerId - the pointer pressed
 * @see docs/domain/interaction.md#_2-selector
 */
function startPointerCapture(document: Document, drag: SelectionDrag, pointerId: number): void {
  if (drag.isRectangle || drag.move !== undefined) {
    selectElement(document, "canvas").setPointerCapture(pointerId);
  }
}

/**
 * Starts a press with the Selector on the drawing (F37) when the main button is pressed, a drag
 * left before dropped: kept from the press to the release (`SELECTION_DRAGS`), so that a release
 * after the pointer went further than 3 pixels from the press is no click; pressed on an empty place, in Shape mode, it draws a
 * selection rectangle, and on a shape it moves the shapes selected (`readPressedDrag`, F39), the
 * canvas taking the pointer for either so that the drag goes on beyond its edges.
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

  const drag = readPressedDrag(document, press, event);

  startPointerCapture(document, drag, event.pointerId);
  SELECTION_DRAGS.set(document, drag);
}
