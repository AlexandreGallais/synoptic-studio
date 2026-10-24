import { draggedBox, isDragged } from "../../src";
import { startNewShape, writeClickedShape } from "../placing";
import { readView } from "../view";

import { writeDraggedShape } from "./write-dragged-shape";

import type { ShapeDrawing } from "./shape-drawing";
import type { Point, Size } from "../../src";

/**
 * Reads the box of a drag (`draggedBox`), square with Shift for every tool, a text's box included.
 *
 * @kind procedure
 * @param drawing - the drag, its press, its pointer and Shift
 * @returns the box dragged, on integers
 * @see docs/domain/interaction.md#_1-tools
 */
function readDrawingBox(drawing: ShapeDrawing): Point & Size {
  return draggedBox({ isSquare: drawing.isShift, press: drawing.press, release: drawing.pointer });
}

/**
 * Writes the new shape's box, the shape started first (`startNewShape`) unless it is drawn
 * already: a box 0 × 0, the pointer back on the press's integer point, places it as a click
 * (`writeClickedShape`), any other is the box dragged (`writeDraggedShape`).
 *
 * @kind procedure
 * @param document - page document
 * @param drawing - the drag, its press, its pointer and Shift
 * @param box - the box dragged, on integers
 * @see docs/domain/interaction.md#_1-tools
 */
function writeDrawingBox(document: Document, drawing: ShapeDrawing, box: Point & Size): void {
  if (!drawing.isDrawn) {
    startNewShape(document, drawing.tool);
  }

  // Width and height are integers ≥ 0: their sum is 0 for a box 0 × 0 alone.
  if (box.width + box.height === 0) {
    writeClickedShape(document, { point: drawing.press, tool: drawing.tool });
  } else {
    writeDraggedShape(document, drawing.tool, box);
  }
}

/**
 * Writes the new shape as the drag stands (F36): nothing while the pointer stays within 3 pixels
 * of the press at the view's scale (`isDragged`, SP-030's answer 8), which leaves the release to
 * the click (F35); once it has gone further, the shape started and its box written
 * (`writeDrawingBox`), placed as a click if the pointer comes back to the press's integer point.
 *
 * @kind procedure
 * @param document - page document
 * @param drawing - the drag, its press, its pointer and Shift
 * @returns the drag, its shape drawn or not
 * @see docs/domain/interaction.md#_1-tools
 */
export function showShapeDrawing(document: Document, drawing: ShapeDrawing): ShapeDrawing {
  const box = readDrawingBox(drawing);

  if (!drawing.isDrawn && !isDragged(drawing.press, drawing.pointer, readView(document).scale)) {
    return drawing;
  }

  writeDrawingBox(document, drawing, box);

  return { ...drawing, isDrawn: true };
}
