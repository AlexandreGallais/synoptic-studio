import { cyclicItem } from "../contour";
import { hasLength } from "../corner-radius";
import { cubicSide, isCurvedSide } from "../curve";
import { NO_CURVE_CORNER } from "../handles";

import { cubicBetween } from "./cubic-between";
import { curveFillets } from "./curve-fillets";

import type { ContourPiece } from "../contour";
import type { CurveCorner } from "../handles";

/**
 * Contour of a free shape with handles: as `roundedContour`, each corner's fillet arc and a piece
 * per side between them — a cubic for a curved side, cut where the fillets touch it
 * (`DERIV-fillet-beside-curve` step 9), its ends those of the arcs so that pieces join exactly, a
 * segment otherwise; pieces without length dropped. With
 * every handle off it is the rounded contour, piece for piece.
 *
 * @kind geometry
 * @param corners - corners of the shape, in contour order
 * @returns the pieces, clockwise, each starting where the previous one ends
 * @see ADR-0047
 */
export function curveContour(corners: readonly CurveCorner[]): readonly ContourPiece[] {
  const fillets = curveFillets(corners);

  return fillets
    .flatMap((fillet, index): ContourPiece[] => {
      const next = cyclicItem(fillets, index + 1, fillet);
      const start = cyclicItem(corners, index, NO_CURVE_CORNER);
      const end = cyclicItem(corners, index + 1, start);
      const side: ContourPiece = isCurvedSide(start, end)
        ? {
            ...cubicBetween(cubicSide(start, end), fillet.after, 1 - next.before),
            end: next.arc.start,
            start: fillet.arc.end,
          }
        : { end: next.arc.start, kind: "segment", start: fillet.arc.end };

      return [side, next.arc];
    })
    .filter((piece) => hasLength(piece));
}
