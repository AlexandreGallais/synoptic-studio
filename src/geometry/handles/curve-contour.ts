import { cyclicItem } from "../contour";
import { roundedContour } from "../corner-radius";

import { cubicSide } from "./cubic-side";
import { isCurvedSide } from "./is-curved-side";

import type { CurveCorner } from "./curve-corner";
import type { ContourPiece, Cubic } from "../contour";

/**
 * Corners as filleted next to curves: a corner in handles mode, or next to a curved side, keeps
 * no fillet (ADR-0046: a fillet needs two straight sides, ADR-0007); its radius is kept in the
 * model, only not drawn.
 *
 * @kind geometry
 * @param corners - corners of the shape
 * @returns the same corners, the radius of those 0
 * @see ADR-0046
 */
function filletableCorners(corners: readonly CurveCorner[]): readonly CurveCorner[] {
  return corners.map((corner, index) => {
    const previous = cyclicItem(corners, index - 1, corner);
    const next = cyclicItem(corners, index + 1, corner);
    const isSharp =
      corner.handles.isOn || isCurvedSide(previous, corner) || isCurvedSide(corner, next);

    return isSharp ? { ...corner, radius: 0 } : corner;
  });
}

/**
 * Whether a cubic and a straight piece join the same two points, in the same order.
 *
 * @kind geometry
 * @param cubic - a curved side
 * @param piece - a straight side of the rounded contour
 * @returns `true` when the cubic replaces that straight side
 * @see ADR-0046
 */
function isSameSide(cubic: Cubic, piece: ContourPiece): boolean {
  const ends = [cubic.start.x, cubic.start.y, cubic.end.x, cubic.end.y];
  const sides = [piece.start.x, piece.start.y, piece.end.x, piece.end.y];

  return ends.every((value, index) => value === sides[index]);
}

/**
 * Contour of a free shape with handles: the rounded contour of its corners (`roundedContour`),
 * each curved side then drawn as a cubic. A curved side's corners keep no fillet, so its segment
 * runs from its start point to its end point, and the cubic takes its place. With every handle
 * off it is the rounded contour, piece for piece.
 *
 * @kind geometry
 * @param corners - points, radii and handles in contour order (Q11), clockwise
 * @returns the pieces of the contour, those without length dropped
 * @see ADR-0046
 */
export function curveContour(corners: readonly CurveCorner[]): readonly ContourPiece[] {
  const kept = filletableCorners(corners);
  const curved = kept.flatMap((corner, index) => {
    const next = cyclicItem(kept, index + 1, corner);

    return isCurvedSide(corner, next) ? [cubicSide(corner, next)] : [];
  });

  return roundedContour(kept).map(
    (piece) =>
      curved.find((cubic) => piece.kind === "segment" && isSameSide(cubic, piece)) ?? piece,
  );
}
