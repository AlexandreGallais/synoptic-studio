import { shapesTouchedByBox } from "../../src";
import { writeShapeSelection } from "../selection";

import type { BoundingBox } from "../../src";

/**
 * Selects the shapes a selection rectangle takes (F37): those whose painted area it touches
 * (`shapesTouchedByBox`), only those it encloses with Alt held (`shapesEnclosedByBox`); alone, or
 * with Shift added to the shapes selected (`shapeSelectionAfterBox`). They are then selected by
 * their indices (`writeShapeSelection`), one loaded into the panel as a click loads it.
 *
 * @kind procedure
 * @param document - page document
 * @param box - the rectangle, in the coordinates of the drawing's SVG
 * @param event - the release, telling whether Alt and Shift are held
 * @see docs/domain/interaction.md#_2-selector
 */
export function selectShapesInBox(document: Document, box: BoundingBox, event: MouseEvent): void {
  // Placeholder until US-069.T2.
  writeShapeSelection(document, event.altKey ? [] : shapesTouchedByBox([], box));
}
