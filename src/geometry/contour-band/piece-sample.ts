import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * One of the points a band follows a piece of a contour by (`pieceSamples`), computed alone so
 * that a band within a box computes only the ones it needs (`DERIV-band-within-box` step 5): the
 * first its start and the last its end, exactly; between, an arc's start turned about its center
 * by an equal share of its sweep (`regularPartition`'s point, `rotate`), the sweep the signed
 * angle from its start to its end (`signedAngle`), a cubic's point at an equal step of t
 * (`cubicPoint`).
 *
 * Formula: Pᵢ = C + R((i · θ) / n)(S − C) for an arc, B(i / n) for a cubic; P₀ = S, Pₙ = E.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @param count - its number of chords n, ≥ 1 (`pieceStepCount`), 1 for a segment
 * @param index - the point's index i, from 0 to n
 * @returns the point
 * @see DERIV-band-within-box
 */
export function pieceSample(piece: ContourPiece, count: number, index: number): Point {
  return index === count ? piece.end : piece.start;
}
