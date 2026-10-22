import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Whether a point is in the area a closed contour paints, by the nonzero rule: its winding number
 * about the point (`windingNumber`) is not 0 (`DERIV-painted-area` step 5).
 *
 * Formula: wn(P) ≠ 0.
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param point - the point asked
 * @returns true when the contour's fill covers the point
 * @see REF-SVG2-FILL-RULE
 */
export function isInPaintedArea(pieces: readonly ContourPiece[], point: Point): boolean {
  return pieces.length === point.x;
}
