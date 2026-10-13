import { norm, subtract } from "../../math";
import { cyclicItem } from "../contour";
import { cubicSide, isCurvedSide, reversedCubic, straightCubic } from "../curve";

import type { CornerSide } from "../fillet-side";
import type { CurveCorner } from "../handles";

/**
 * One side of a corner written from it: a curved side as drawn (`cubicSide`), reversed when it
 * comes into the corner; a straight side at its thirds (`DERIV-fillet-beside-curve` step 0). A
 * fillet may reach its whole length until ADR-0007's sharing limits it.
 *
 * @kind geometry
 * @param corner - the corner the side belongs to
 * @param other - the side's other corner
 * @param isAfter - whether the side leaves the corner (after it) or comes into it (before it)
 * @returns the side, written from the corner
 * @see DERIV-fillet-beside-curve
 */
function sideFrom(corner: CurveCorner, other: CurveCorner, isAfter: boolean): CornerSide {
  const isStraight = isAfter ? !isCurvedSide(corner, other) : !isCurvedSide(other, corner);
  const curved = isAfter ? cubicSide(corner, other) : reversedCubic(cubicSide(other, corner));

  return {
    cubic: isStraight ? straightCubic(corner.point, other.point) : curved,
    isStraight,
    length: norm(subtract(other.point, corner.point)),
    limit: 1,
  };
}

/**
 * Each corner's two sides written from it (`sideFrom`).
 *
 * @kind geometry
 * @param corners - corners of the shape, in contour order
 * @returns for each corner, its side before then its side after
 * @see DERIV-fillet-beside-curve
 */
export function cornerSides(
  corners: readonly CurveCorner[],
): readonly (readonly [CornerSide, CornerSide])[] {
  return corners.map((corner, index) => [
    sideFrom(corner, cyclicItem(corners, index - 1, corner), false),
    sideFrom(corner, cyclicItem(corners, index + 1, corner), true),
  ]);
}
