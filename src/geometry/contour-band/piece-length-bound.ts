import { distance, signedAngle, subtract } from "../../math";
import { CUBIC_DEGREE } from "../placement";

import type { ContourPiece } from "../contour";

/**
 * A bound on how far a piece of a contour runs, from which its chords are counted
 * (`pieceStepCount`): a segment its length; an arc its length, its radius times the signed angle
 * from its start to its end (`signedAngle`), of less than a half turn; a cubic three times the
 * longest side of its control polygon, at least the curve's speed (`REF-POMAX-BEZIER`, the
 * derivative's weights), so that n chords at equal steps of t are each at most the bound over n
 * (`DERIV-band-within-box` step 4).
 *
 * Formula: |E − S|; r · |θ|; 3 · max(|P₀P₁|, |P₁P₂|, |P₂P₃|).
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @returns the bound, ≥ 0, in user units
 * @see DERIV-band-within-box
 */
export function pieceLengthBound(piece: ContourPiece): number {
  if (piece.kind === "segment") {
    return distance(piece.start, piece.end);
  }

  if (piece.kind === "arc") {
    const sweep = signedAngle(
      subtract(piece.start, piece.center),
      subtract(piece.end, piece.center),
    );

    return piece.radius * Math.abs(sweep);
  }

  return (
    CUBIC_DEGREE *
    Math.max(
      distance(piece.start, piece.first),
      distance(piece.first, piece.second),
      distance(piece.second, piece.end),
    )
  );
}
