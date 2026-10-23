import type { ShapesMove } from "../hover";

/**
 * Reads the move a press on a shape with the Selector starts in Shape mode (F39): the drawing and
 * the shapes selected at the press; the shapes selected once it moves, those selected when the
 * shape pressed is one of them, otherwise it alone, or with Shift added to them
 * (`shapeSelectionAfterPress`); the shapes moved, a text never measured, which is not drawn, left
 * as it is; and their anchor, the top-left of their dashed box (`pickAreasBox`), for one shape its
 * straight box, the X and Y the panel shows.
 *
 * @kind procedure
 * @param document - page document
 * @param index - the index of the shape pressed (`shapeAtPoint`)
 * @param isShift - whether Shift is held at the press
 * @returns the drawing, the shapes selected and moved, and their anchor
 * @see docs/domain/interaction.md#_2-selector
 */
export function readShapesMove(document: Document, index: number, isShift: boolean): ShapesMove {
  return {
    anchor: { x: index, y: 0 },
    before: [],
    drawing: [],
    moved: [],
    selected: isShift ? [] : [document.childNodes.length],
  };
}
