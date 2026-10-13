import { cyclicItem } from "../contour";

import { isCurvedSide } from "./is-curved-side";

import type { CurveCorner } from "../handles";

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
export function filletableCorners(corners: readonly CurveCorner[]): readonly CurveCorner[] {
  return corners.map((corner, index) => {
    const previous = cyclicItem(corners, index - 1, corner);
    const next = cyclicItem(corners, index + 1, corner);
    const isSharp =
      corner.handles.isOn || isCurvedSide(previous, corner) || isCurvedSide(corner, next);

    return isSharp ? { ...corner, radius: 0 } : corner;
  });
}
