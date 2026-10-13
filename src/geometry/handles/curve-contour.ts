import { cyclicItem } from "../contour";
import { filletArcs, hasLength } from "../corner-radius";

import { cubicSide } from "./cubic-side";
import { isCurvedSide } from "./is-curved-side";

import type { CurveCorner } from "./curve-corner";
import type { ContourPiece } from "../contour";

/** Corner read when there is none: a contour has 3 corners or more. */
const NO_CORNER: CurveCorner = {
  handles: { after: { x: 0, y: 0 }, before: { x: 0, y: 0 }, isOn: false, mode: "angle-and-length" },
  point: { x: 0, y: 0 },
  radius: 0,
};

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
 * Contour of a free shape with handles: as `roundedContour`, each corner's fillet arc and a piece
 * per side between them, built side by side — a cubic for a curved side (its corners kept sharp,
 * so it runs from point to point), a segment otherwise; pieces without length dropped. With every
 * handle off it is the rounded contour, piece for piece.
 *
 * @kind geometry
 * @param corners - points, radii and handles in contour order (Q11), clockwise
 * @returns the pieces of the contour, those without length dropped
 * @see ADR-0046
 */
export function curveContour(corners: readonly CurveCorner[]): readonly ContourPiece[] {
  const kept = filletableCorners(corners);
  const arcs = filletArcs(kept);

  return arcs
    .flatMap((arc, index): ContourPiece[] => {
      const next = cyclicItem(arcs, index + 1, arc);
      const start = cyclicItem(kept, index, NO_CORNER);
      const end = cyclicItem(kept, index + 1, start);
      const side: ContourPiece = isCurvedSide(start, end)
        ? cubicSide(start, end)
        : { end: next.start, kind: "segment", start: arc.end };

      return [side, next];
    })
    .filter((piece) => hasLength(piece));
}
