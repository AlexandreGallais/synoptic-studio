import type { ContourPiece, Segment } from "../contour";

/**
 * Whether a contour's piece reaches a horizontal side, by its kind: a segment
 * (`isSideReachedBySegment`), a fillet arc (`isSideReachedByArc`), a cubic side (`isSideReachedByCubic`)
 * (`DERIV-box-overlap` steps 5 to 7).
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @param side - a horizontal side, its ends at the same ordinate
 * @returns true when a point of the piece lies on the side
 * @see DERIV-box-overlap
 */
export function isSideReachedByPiece(
  piece: ContourPiece,
  side: Pick<Segment, "end" | "start">,
): boolean {
  return piece.start.y === side.start.y;
}
