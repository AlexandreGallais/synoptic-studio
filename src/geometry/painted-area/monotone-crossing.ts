import { cubicPoint } from "../placement";

import { cubicLevelParameter } from "./cubic-level-parameter";
import { isLevelCrossed } from "./is-level-crossed";

import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * Signed crossing of a part of a cubic side monotone in y with the horizontal ray from a point,
 * counted as a segment's from the part's ends (`isLevelCrossed`): where it meets the ray's line
 * (`cubicLevelParameter`), it counts its direction when that point is strictly right of the
 * point asked (`DERIV-painted-area` step 4).
 *
 * Formula: d = sign(y(b) − y(a)); d when min(y(a), y(b)) ≤ p_y < max(y(a), y(b)) and
 * x(t*) > pₓ, else 0.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param span - the part's first and last t, y monotone between them
 * @param point - where the ray starts
 * @returns +1, −1 or 0
 * @see DERIV-painted-area
 */
export function monotoneCrossing(
  cubic: Cubic,
  span: readonly [number, number],
  point: Point,
): number {
  const from = cubicPoint(cubic, span[0]).y;
  const to = cubicPoint(cubic, span[1]).y;

  if (!isLevelCrossed(from, to, point.y)) {
    return 0;
  }

  const crossing = cubicPoint(cubic, cubicLevelParameter(cubic, span, point.y));

  return crossing.x > point.x ? Math.sign(to - from) : 0;
}
