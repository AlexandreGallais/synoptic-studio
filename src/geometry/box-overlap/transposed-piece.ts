import { transposedPoint } from "./transposed-point";

import type { ContourPiece } from "../contour";

/**
 * A contour's piece with its axes swapped (`transposedPoint`): a segment by its ends, a cubic
 * side by its control points, a linear map of the plane being applied to a Bézier curve through
 * them; a fillet arc by its ends and centre, its radius kept and its direction reversed, the swap
 * reversing every turn (`DERIV-box-overlap` step 8).
 *
 * Formula: every point (x, y) ↦ (y, x); σ ↦ −σ for an arc.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @returns the same kind of piece, swapped
 * @see DERIV-box-overlap
 */
export function transposedPiece(piece: ContourPiece): ContourPiece {
  const ends = { end: transposedPoint(piece.end), start: transposedPoint(piece.start) };

  switch (piece.kind) {
    case "arc": {
      const { center, positive } = piece;

      return { ...piece, ...ends, center: transposedPoint(center), positive: !positive };
    }

    case "cubic": {
      const { first, second } = piece;

      return { ...piece, ...ends, first: transposedPoint(first), second: transposedPoint(second) };
    }

    case "segment": {
      return { ...piece, ...ends };
    }
  }
}
