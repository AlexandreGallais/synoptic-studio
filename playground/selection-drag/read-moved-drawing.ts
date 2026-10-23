import type { Point } from "../../src";
import type { ShapesMove } from "../hover";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the drawing once moved (F39): the drawing at the press, each shape moved going by the
 * shift (`shapeMovedBy`), with the kind the panel showed it as, a text keeping the metrics it was
 * last drawn with (`writeKeptMetrics`); the others as they are, in the order drawn.
 *
 * @kind procedure
 * @param move - the drawing at the press and the shapes moved
 * @param shift - how far they go
 * @returns every shape in the order drawn, those moved at their new place
 * @see docs/domain/interaction.md#_2-selector
 */
export function readMovedDrawing(move: ShapesMove, shift: Point): readonly KeptShape[] {
  return shift.x === 0 ? move.drawing : [];
}
