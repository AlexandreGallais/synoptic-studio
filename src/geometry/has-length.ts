import { EPSILON, norm, subtract } from "../math";

import type { ContourPiece } from "./contour-piece";

/**
 * Whether a contour piece is drawn: its ends are at least `EPSILON` apart (Q10).
 *
 * A segment between two tangent points that meet, or the arc of a corner that keeps no fillet,
 * has no length and is dropped from a rounded contour.
 *
 * @kind geometry
 * @param piece - segment or arc
 * @returns `true` when the distance between its ends is at least `EPSILON`
 * @see DERIV-fillet-arc
 */
export function hasLength(piece: ContourPiece): boolean {
  return norm(subtract(piece.end, piece.start)) >= EPSILON;
}
