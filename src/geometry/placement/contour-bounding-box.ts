import { hasLength } from "../corner-radius";

import { arcAxisPoints } from "./arc-axis-points";
import { boundingBox } from "./bounding-box";

import type { BoundingBox } from "./bounding-box";
import type { ContourPiece } from "../contour";

/**
 * Straight box of a contour of segments and fillet arcs: the smallest upright rectangle around
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
  return boundingBox(
    pieces.flatMap((piece) =>
      piece.kind === "arc" && hasLength(piece)
        ? [piece.start, piece.end, ...arcAxisPoints(piece)]
        : [piece.start, piece.end],
    ),
  );
}
