import { isInPaintedArea } from "../painted-area";

import { isPieceMeetingBox } from "./is-piece-meeting-box";

import type { ContourPiece } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * Whether an upright box touches the area a closed contour paints, its outline included: a piece
 * of the contour meets the box (`isPieceMeetingBox`), or, none meeting it, the box lies in one region
 * of the contour, painted when its top-left corner is (`isInPaintedArea`)
 * (`DERIV-box-overlap` step 3).
 *
 * Formula: some piece meets R, or wn(K) ≠ 0 with K = (minX, minY).
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param box - its extreme coordinates
 * @returns true when the box and the painted area share a point
 * @see DERIV-box-overlap
 */
export function isContourTouchingBox(pieces: readonly ContourPiece[], box: BoundingBox): boolean {
  return (
    pieces.some((piece) => isPieceMeetingBox(piece, box)) ||
    isInPaintedArea(pieces, { x: box.minX, y: box.minY })
  );
}
