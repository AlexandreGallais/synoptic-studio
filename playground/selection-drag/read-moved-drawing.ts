import { shapeMovedBy } from "../../src";
import { writeKeptMetrics } from "../kept-shapes";

import type { Point } from "../../src";
import type { ShapesMove } from "../hover";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the drawing once moved: the drawing at the press, each shape moved going by the
 * shift (`shapeMovedBy`), with the kind the panel showed it as, a text keeping the metrics it was
 * last drawn with (`writeKeptMetrics`); the others as they are, in the order drawn.
 *
 * @kind procedure
 * @param move - the drawing at the press and the shapes moved
 * @param shift - how far they go
 * @returns every shape in the order drawn, those moved at their new place
 */
export function readMovedDrawing(move: ShapesMove, shift: Point): readonly KeptShape[] {
  return move.drawing.map((kept, index) => {
    if (!move.moved.includes(index)) {
      return kept;
    }

    const shape = shapeMovedBy(kept.shape, shift);

    writeKeptMetrics(kept.shape, shape);

    return { kind: kept.kind, shape };
  });
}
