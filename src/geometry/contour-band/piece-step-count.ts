import { ceiling } from "../../math";

import { pieceLengthBound } from "./piece-length-bound";

import type { ContourPiece } from "../contour";

/**
 * How many chords a band follows a piece of a contour by (`contourBand`), so that none is longer
 * than `step`: a segment one, itself; a curve its length bound over the step, rounded up
 * (`pieceLengthBound`), at least one.
 *
 * Formula: 1 for a segment; n = max(⌈Λ / s⌉, 1) for an arc or a cubic.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @param step - the longest chord wanted along a curve, in user units, > 0
 * @returns the number of chords, ≥ 1
 * @see DERIV-band-within-box
 */
export function pieceStepCount(piece: ContourPiece, step: number): number {
  return piece.kind === "segment" ? 1 : Math.max(ceiling(pieceLengthBound(piece) / step), 1);
}
