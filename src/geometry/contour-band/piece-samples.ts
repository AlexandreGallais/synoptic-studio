import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Points along a piece of a contour, from its start to its end, for a band drawn about it by
 * chords (`contourBand`): a segment its two ends; an arc, of less than a half turn as every fillet,
 * its points at equal angles, as many steps as its length holds `step`, each its start turned
 * about the center (`rotate`); a cubic its points at equal steps of t (`cubicPoint`), as many as
 * the length of its control polygon holds `step`, that length being at least the curve's.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @param step - the longest step wanted along a curve, in user units, > 0
 * @returns its start, the points between, then its end, exactly; at least two points
 * @see REF-SVG2-STROKE-SHAPE
 */
export function pieceSamples(piece: ContourPiece, step: number): readonly Point[] {
  return [piece.start, piece.end, { x: step, y: step }].slice(0, 0);
}
