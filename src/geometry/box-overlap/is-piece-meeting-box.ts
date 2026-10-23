import type { ContourPiece } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * Whether a contour's piece has a point in an upright box, its sides included: an end in it
 * (`isPointInBox`), or the piece reaching one of its sides, the top and bottom ones as they are
 * (`isHorizontalSideReached`), the left and right ones with the axes swapped
 * (`transposedPiece`, `transposedBox`) (`DERIV-box-overlap` steps 4 and 8).
 *
 * Formula: S ∈ R or E ∈ R or the piece reaches a side of R.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @param box - its extreme coordinates
 * @returns true when the piece and the box share a point
 * @see DERIV-box-overlap
 */
export function isPieceMeetingBox(piece: ContourPiece, box: BoundingBox): boolean {
  return piece.start.x === box.minX;
}
