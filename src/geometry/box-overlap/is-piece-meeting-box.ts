import { isPointInBox } from "../placement";

import { isHorizontalSideReached } from "./is-horizontal-side-reached";
import { transposedBox } from "./transposed-box";
import { transposedPiece } from "./transposed-piece";

import type { ContourPiece } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * Whether a contour's piece has a point in an upright box, its sides included: its start in it
 * (`isPointInBox`), its end needing no test since a piece entering the box reaches a side on its
 * way, or the piece reaching one of its sides, the top and bottom ones as they are
 * (`isHorizontalSideReached`), the left and right ones with the axes swapped
 * (`transposedPiece`, `transposedBox`) (`DERIV-box-overlap` steps 4 and 8).
 *
 * Formula: S ∈ R or the piece reaches a side of R.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @param box - its extreme coordinates
 * @returns true when the piece and the box share a point
 * @see DERIV-box-overlap
 */
export function isPieceMeetingBox(piece: ContourPiece, box: BoundingBox): boolean {
  return (
    isPointInBox(box, piece.start) ||
    isHorizontalSideReached(piece, box) ||
    isHorizontalSideReached(transposedPiece(piece), transposedBox(box))
  );
}
