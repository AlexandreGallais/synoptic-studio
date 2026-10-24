import { landedShift, landedShiftOnLargerAxis, offsetOnLargerAxis, subtract } from "../../src";

import type { Point } from "../../src";
import type { SelectionDrag, ShapesMove } from "../hover";

/**
 * Reads how far the shapes a move takes follow the pointer while dragged: the pointer's offset
 * from the press (`subtract`), along the axis of the larger displacement while Shift is held
 * (`offsetOnLargerAxis`).
 *
 * @kind procedure
 * @param drag - the press, the pointer and whether Shift is held
 * @returns the offset, possibly fractional
 * @see docs/domain/interaction.md#_2-selector
 */
function readDraggedOffset(drag: SelectionDrag): Point {
  const free = subtract(drag.pointer, drag.press);

  return drag.isShift ? offsetOnLargerAxis(free) : free;
}

/**
 * Reads the shift landing the shapes a move takes at the release: their anchor moved by the
 * pointer's offset and landed on integers (`landedShift`), with Shift held on the axis of the
 * larger displacement alone, the other kept still (`landedShiftOnLargerAxis`).
 *
 * @kind procedure
 * @param drag - the press, the pointer and whether Shift is held
 * @param anchor - the top-left of the straight box of the shapes moved, their dashed box
 * @returns the shift every shape moved goes by
 * @see docs/domain/interaction.md#_2-selector
 */
function readLandedShift(drag: SelectionDrag, anchor: Point): Point {
  const free = subtract(drag.pointer, drag.press);

  return drag.isShift ? landedShiftOnLargerAxis(anchor, free) : landedShift(anchor, free);
}

/**
 * Reads how far a move takes the shapes selected (F39): while dragged the pointer's offset, the
 * shapes following it freely (`readDraggedOffset`); at the release, the shift landing their
 * anchor on integers (`readLandedShift`).
 *
 * @kind procedure
 * @param drag - the press, the pointer and whether Shift is held
 * @param move - the shapes moved and their anchor
 * @param isLanded - whether the pointer is released
 * @returns the shift every shape moved goes by
 * @see docs/domain/interaction.md#_2-selector
 */
export function readMoveShift(drag: SelectionDrag, move: ShapesMove, isLanded: boolean): Point {
  return isLanded ? readLandedShift(drag, move.anchor) : readDraggedOffset(drag);
}
