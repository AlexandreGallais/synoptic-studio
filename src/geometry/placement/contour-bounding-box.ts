import { hasLength } from "../corner-radius";

import { arcAxisPoints } from "./arc-axis-points";
import { boundingBox } from "./bounding-box";
import { cubicExtremes } from "./cubic-extremes";

import type { BoundingBox } from "./bounding-box";
import type { Point } from "../../math";
import type { ContourPiece } from "../contour";

/**
 * Points of a piece, other than its ends, where it reaches its extreme x or y: an arc's axis
 * points, a cubic's extremes; none for a segment or a piece without length.
 *
 * @kind geometry
 * @param piece - segment, arc or cubic
 * @returns those points
 * @see DERIV-cubic-sides
 */
function innerExtremes(piece: ContourPiece): readonly Point[] {
  if (!hasLength(piece) || piece.kind === "segment") {
    return [];
  }

  return piece.kind === "arc" ? arcAxisPoints(piece) : cubicExtremes(piece);
}

/**
 * Straight box of a contour of segments, fillet arcs and cubic sides (`DERIV-cubic-sides`): the smallest upright rectangle around
 * the drawn outline, arcs included, as `getBBox` returns it, strokes excluded
 * (`DERIV-arc-bounding-box` step 4).
 *
 * @kind geometry
 * @param pieces - segments and arcs of a contour (`roundedContour`)
 * @returns its extreme coordinates
 * @see DERIV-arc-bounding-box
 */
export function contourBoundingBox(pieces: readonly ContourPiece[]): BoundingBox {
  // A piece without length (a corner that keeps no fillet) adds its ends only: an arc's axis test
  // needs a sweep between 0 and half a turn.
  return boundingBox(pieces.flatMap((piece) => [piece.start, piece.end, ...innerExtremes(piece)]));
}
