import { pointBetween } from "../../math";
import { CUBIC_DEGREE } from "../placement";

import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * A straight side written as a cubic, its control points at the thirds: then B(t) = A + t(B − A),
 * the parameter proportional to the length (`DERIV-fillet-beside-curve`, checks; `REF-WIKI-BEZIER`).
 *
 * @kind geometry
 * @param start - one end of the side
 * @param end - the other end
 * @returns the cubic tracing the segment at constant speed
 * @see DERIV-fillet-beside-curve
 */
export function straightCubic(start: Point, end: Point): Cubic {
  return {
    end,
    first: pointBetween(start, end, 1 / CUBIC_DEGREE),
    kind: "cubic",
    second: pointBetween(start, end, 2 / CUBIC_DEGREE),
    start,
  };
}
