import { add, scale, withoutNegativeZero } from "../../math";

import { ELLIPSE_QUARTERS } from "./ellipse-quarters";
import { QUARTER_CIRCLE_HANDLE } from "./quarter-circle-handle";

import type { Vector } from "../../math";
import type { CurveCorner } from "../handles";
import type { Size } from "../layout-box";

/**
 * A vector of the unit circle's frame stretched to the ellipse's half axes: the affine map that
 * makes the circle the ellipse (`DERIV-round-shape-curve` step 2).
 *
 * Formula: (a · vₓ, b · v_y).
 *
 * @kind geometry
 * @param vector - vector in the frame of the unit circle
 * @param half - half axes `a` and `b` of the ellipse
 * @returns the vector in the frame of the ellipse
 * @see REF-WIKI-BEZIER
 */
function stretchedToEllipse(vector: Vector, half: Vector): Vector {
  return { x: half.x * vector.x, y: half.y * vector.y };
}

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
  const half = { x: size.width / 2, y: size.height / 2 };

  return ELLIPSE_QUARTERS.map(({ point, tangent }) => {
    const after = scale(stretchedToEllipse(tangent, half), QUARTER_CIRCLE_HANDLE);

    return {
      handles: {
        // Never −0, so that a path never writes "-0" (a box of width or height 0).
        after: withoutNegativeZero(after),
        before: withoutNegativeZero(scale(after, -1)),
        isOn: true,
        mode: "angle-and-length",
      },
      point: add(half, stretchedToEllipse(point, half)),
      radius: 0,
    };
  });
}
