import { add, hadamardProduct, opposite, scale, withoutNegativeZero } from "../../math";
import { halfSize } from "../layout-box";

import { ELLIPSE_QUARTERS } from "./ellipse-quarters";
import { QUARTER_CIRCLE_HANDLE } from "./quarter-circle-handle";

import type { CurveCorner } from "../handles";
import type { Size } from "../layout-box";

/**
 * The 4 points of the ellipse filling a box, top, right, bottom and left, clockwise from the top,
 * each in handles mode, Mirror angle and length, its handles 4(√2 − 1)/3 of the half axis along
 * the curve; radii 0 (`DERIV-round-shape-curve` steps 2–3). In a square box, a circle.
 *
 * @kind geometry
 * @param size - width and height of the box, ≥ 0, its top-left at the origin
 * @returns the 4 corners drawing the ellipse as four cubic sides
 * @see DERIV-round-shape-curve
 */
export function ellipseCorners(size: Size): readonly CurveCorner[] {
  const half = halfSize(size);

  return ELLIPSE_QUARTERS.map(({ point, tangent }) => {
    const after = scale(hadamardProduct(tangent, half), QUARTER_CIRCLE_HANDLE);

    return {
      handles: {
        // Never −0, so that a path never writes "-0" (a box of width or height 0).
        after: withoutNegativeZero(after),
        before: withoutNegativeZero(opposite(after)),
        isOn: true,
        mode: "angle-and-length",
      },
      point: add(half, hadamardProduct(point, half)),
      radius: 0,
    };
  });
}
