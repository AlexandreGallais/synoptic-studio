import { landedShift, offsetOnLargerAxis, subtract } from "../../src";

import type { Point } from "../../src";
import type { SelectionDrag, ShapesMove } from "../hover";

/**
 * Reads how far a move takes the shapes selected (F39): the pointer's offset from the press
 * (`subtract`), along the axis of the larger displacement while Shift is held
 * (`offsetOnLargerAxis`); at the release, the shift landing their anchor on integers
 * (`landedShift`), while dragged the offset itself, the shapes following the pointer freely.
 *
 * @kind procedure
 * @param drag - the press, the pointer and whether Shift is held
 * @param move - the shapes moved and their anchor
 * @param isLanded - whether the pointer is released
 * @returns the shift every shape moved goes by
 * @see docs/domain/interaction.md#_2-selector
 */
export function readMoveShift(drag: SelectionDrag, move: ShapesMove, isLanded: boolean): Point {
  const free = subtract(drag.pointer, drag.press);
  const offset = drag.isShift ? offsetOnLargerAxis(free) : free;

  return isLanded ? landedShift(move.anchor, offset) : offset;
}
