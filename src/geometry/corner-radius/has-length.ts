import { EPSILON, distance } from "../../math";

import type { ContourPiece } from "../contour";

/**
 * Whether a contour piece is drawn: its ends are at least `EPSILON` apart (Q10), or, for a cubic,
 * one of its control points is away from its start (a closed loop still has length).
 *
 * A segment between two tangent points that meet, or the arc of a corner that keeps no fillet,
 * has no length and is dropped from a rounded contour.
 *
 * @kind geometry
 * @param piece - segment, arc or cubic
 * @returns `true` when a point of the piece is at least `EPSILON` from its start
 * @see DERIV-fillet-arc
 */
export function hasLength(piece: ContourPiece): boolean {
  const points = piece.kind === "cubic" ? [piece.first, piece.second, piece.end] : [piece.end];

  return points.some((point) => distance(piece.start, point) >= EPSILON);
}
