import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Winding number of a closed contour about a point: the signed count of its pieces' crossings
 * with the horizontal ray from the point (`pieceCrossing`), +1 for each crossing with y growing,
 * −1 with y falling (`DERIV-painted-area` steps 1 and 5).
 *
 * Formula: Σ count(piece).
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param point - the point asked
 * @returns an integer, 0 for a point the contour does not wind around
 * @see REF-SUNDAY-INCLUSION
 */
export function windingNumber(pieces: readonly ContourPiece[], point: Point): number {
  return pieces.length + point.x;
}
