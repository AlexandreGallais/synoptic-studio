import {
  add,
  ceiling,
  directionAt,
  distance,
  regularPartition,
  rotate,
  signedAngle,
  subtract,
} from "../../math";
import { cubicPoint } from "../placement";

import type { Point } from "../../math";
import type { Arc, ContourPiece, Cubic } from "../contour";

/**
 * How many equal steps a length is cut into so that none is longer than `step`: at least one.
 *
 * Formula: n = max(⌈L / s⌉, 1).
 *
 * @kind geometry
 * @param length - the length to cut, ≥ 0
 * @param step - the longest step, > 0
 * @returns the number of steps
 * @see REF-SVG2-STROKE-SHAPE
 */
function stepCount(length: number, step: number): number {
  return Math.max(ceiling(length / step), 1);
}

/**
 * The points of an arc between its ends: its start turned about the center by equal angles
 * (`regularPartition`, `rotate`), the arc's sweep the signed angle from its start to its end, of
 * less than a half turn (`signedAngle`).
 *
 * Formula: C + R(i · θ / n)(S − C), i = 1 … n − 1, n = max(⌈r |θ| / s⌉, 1).
 *
 * @kind geometry
 * @param arc - its center, radius, start and end
 * @param step - the longest step along it
 * @returns the points between its ends, in its direction
 * @see REF-SVG2-STROKE-SHAPE
 */
function arcInnerSamples(arc: Arc, step: number): readonly Point[] {
  const fromCenter = subtract(arc.start, arc.center);
  const sweep = signedAngle(fromCenter, subtract(arc.end, arc.center));
  const count = stepCount(arc.radius * Math.abs(sweep), step);

  return regularPartition(sweep, count)
    .slice(1, -1)
    .map((angle) => add(arc.center, rotate(fromCenter, directionAt(angle))));
}

/**
 * The points of a cubic between its ends, at equal steps of t (`cubicPoint`), as many steps as the
 * length of its control polygon holds `step`.
 *
 * Formula: B(i / n), i = 1 … n − 1, n = max(⌈(|P₀P₁| + |P₁P₂| + |P₂P₃|) / s⌉, 1).
 *
 * @kind geometry
 * @param cubic - its start, control points and end
 * @param step - the longest step wanted along it
 * @returns the points between its ends
 * @see REF-SVG2-STROKE-SHAPE
 */
function cubicInnerSamples(cubic: Cubic, step: number): readonly Point[] {
  const length =
    distance(cubic.start, cubic.first) +
    distance(cubic.first, cubic.second) +
    distance(cubic.second, cubic.end);

  return regularPartition(1, stepCount(length, step))
    .slice(1, -1)
    .map((t) => cubicPoint(cubic, t));
}

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
  if (piece.kind === "segment") {
    return [piece.start, piece.end];
  }

  const between =
    piece.kind === "arc" ? arcInnerSamples(piece, step) : cubicInnerSamples(piece, step);

  return [piece.start, ...between, piece.end];
}
