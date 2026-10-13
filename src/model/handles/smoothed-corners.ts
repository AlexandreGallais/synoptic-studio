import { cyclicItem, tensionHandle } from "../../geometry";
import { scale, withoutNegativeZero } from "../../math";

import type { CurveCorner } from "../../geometry";

/**
 * Corners with the handles of each point in mode `auto` computed from its neighbours by its
 * tension, the handle before opposite the handle after (`DERIV-handle-tension`); the other
 * corners as they are. Computed whenever the shape is drawn or stored, so that they follow the
 * neighbours as they move.
 *
 * @kind domain
 * @param corners - corners of a free shape, in contour order
 * @returns the same corners, the automatic handles computed
 * @see DERIV-handle-tension
 */
export function smoothedCorners(corners: readonly CurveCorner[]): readonly CurveCorner[] {
  return corners.map((corner, index) => {
    if (!corner.handles.isOn || corner.handles.mode !== "auto") {
      return corner;
    }

    const previous = cyclicItem(corners, index - 1, corner).point;
    const after = tensionHandle(
      previous,
      cyclicItem(corners, index + 1, corner).point,
      corner.handles.tension,
    );

    return {
      ...corner,
      handles: { ...corner.handles, after, before: withoutNegativeZero(scale(after, -1)) },
    };
  });
}
