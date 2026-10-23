import { boxHorizontalSides } from "./box-horizontal-sides";
import { isSideReachedByPiece } from "./is-side-reached-by-piece";

import type { ContourPiece } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * Whether a contour's piece reaches the top or the bottom side of an upright box
 * (`boxHorizontalSides`, `isSideReachedByPiece`) (`DERIV-box-overlap` step 4).
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic side
 * @param box - its extreme coordinates
 * @returns true when a point of the piece lies on either side
 * @see DERIV-box-overlap
 */
export function isHorizontalSideReached(piece: ContourPiece, box: BoundingBox): boolean {
  return boxHorizontalSides(box).some((side) => isSideReachedByPiece(piece, side));
}
