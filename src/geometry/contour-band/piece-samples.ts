import { pieceSample } from "./piece-sample";
import { pieceStepCount } from "./piece-step-count";

import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Points along a piece of a contour, from its start to its end, for a band drawn about it by
 * chords (`contourBand`): as many chords as `pieceStepCount` gives, each point computed alone
 * (`pieceSample`). A segment gives its two ends; an arc, of less than a half turn as every fillet,
 * its points at equal angles, as many steps as its length holds `step`, each its start turned
 * about the center (`rotate`); a cubic its points at equal steps of t (`cubicPoint`), as many as
 * three times the longest side of its control polygon holds `step`, that bound being at least the
 * curve's speed: B′ is the quadratic Bézier of the weights 3(Pᵢ₊₁ − Pᵢ) (`REF-POMAX-BEZIER`, the
 * derivative's weights), never outside their hull, so a chord over 1/n of t is at most that bound
 * over n, and no chord is longer than `step`.
 *
 * Formula: Pᵢ, i = 0 … n, n = pieceStepCount(piece, s).
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @param step - the longest step wanted along a curve, in user units, > 0
 * @returns its start, the points between, then its end, exactly; at least two points
 * @see REF-SVG2-PAINT
 */
export function pieceSamples(piece: ContourPiece, step: number): readonly Point[] {
  const count = pieceStepCount(piece, step);

  return Array.from({ length: count + 1 }, (unused, index) => pieceSample(piece, count, index));
}
