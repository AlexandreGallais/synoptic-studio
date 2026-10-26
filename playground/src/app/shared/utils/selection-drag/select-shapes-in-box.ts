import { shapeSelectionAfterBox } from "editor";

import { readSelectedShapes } from "../kept-shapes";
import { readPickAreas, readShapesTaken } from "../picking";
import { writeShapeSelection } from "../selection";

import type { BoundingBox } from "editor";

/**
 * Selects the shapes a selection rectangle takes: those whose painted area it touches, only
 * those it encloses with Alt held (`readShapesTaken`); alone, or with Shift added to the shapes
 * selected (`shapeSelectionAfterBox`). They are then selected by their indices
 * (`writeShapeSelection`), one loaded into the panel as a click loads it.
 *
 * @kind procedure
 * @param document - page document
 * @param box - the rectangle, in the coordinates of the drawing's SVG
 * @param event - the release, telling whether Alt and Shift are held
 */
export function selectShapesInBox(document: Document, box: BoundingBox, event: MouseEvent): void {
  const taken = readShapesTaken(readPickAreas(document), box, event.altKey);

  writeShapeSelection(
    document,
    shapeSelectionAfterBox(readSelectedShapes(document), taken, event.shiftKey),
  );
}
