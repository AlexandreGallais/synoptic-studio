import { arcCrossing } from "./arc-crossing";
import { cubicCrossing } from "./cubic-crossing";
import { segmentCrossing } from "./segment-crossing";

import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Signed crossings of a contour's piece with the horizontal ray from a point, by its kind: a
 * segment (`segmentCrossing`), a fillet arc (`arcCrossing`), a cubic side (`cubicCrossing`)
 * (`DERIV-painted-area` steps 2 to 4).
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @param point - where the ray starts
 * @returns the signed count
 * @see DERIV-painted-area
 */
export function pieceCrossing(piece: ContourPiece, point: Point): number {
  switch (piece.kind) {
    case "arc": {
      return arcCrossing(piece, point);
    }

    case "cubic": {
      return cubicCrossing(piece, point);
    }

    case "segment": {
      return segmentCrossing(piece, point);
    }
  }
}
