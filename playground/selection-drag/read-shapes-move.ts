import { shapeSelectionAfterPress } from "../../src";
import { readSelectedShapes } from "../kept-shapes";

import { readSelectionMove } from "./read-selection-move";

import type { ShapesMove } from "../hover";

/**
 * Reads the move a press on a shape with the Selector starts in Shape mode (F39): the shapes
 * selected at the press, and those selected once it moves, those selected when the shape pressed
 * is one of them, otherwise it alone, or with Shift added to them (`shapeSelectionAfterPress`),
 * moved as the shapes selected are (`readSelectionMove`).
 *
 * @kind procedure
 * @param document - page document
 * @param index - the index of the shape pressed (`shapeAtPoint`)
 * @param isShift - whether Shift is held at the press
 * @returns the drawing, the shapes selected and moved, and their anchor
 * @see RULE-interaction-2-selector
 */
export function readShapesMove(document: Document, index: number, isShift: boolean): ShapesMove {
  const before = readSelectedShapes(document);

  return readSelectionMove(document, before, shapeSelectionAfterPress(before, index, isShift));
}
