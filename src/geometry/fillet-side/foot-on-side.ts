import { dot, subtract } from "../../math";
import { cubicPoint } from "../placement";

import { crossings } from "./crossings";
import { sideTangent } from "./side-tangent";

import type { FilletSide } from "./fillet-side";
import type { Point } from "../../math";

/**
 * Whether a point lies ahead of a side's point along its unit tangent — at a corner whose handle
 * is off the velocity vanishes, not the tangent (`DERIV-fillet-beside-curve` step 1,
 * `REF-MATHWORLD-DOT`).
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param point - the point projected
 * @param t - the side's parameter
 * @returns `true` when (P − B(t)) · T(t) > 0
 * @see DERIV-fillet-beside-curve
 */
function isAheadAt(side: FilletSide, point: Point, t: number): boolean {
  return dot(subtract(point, cubicPoint(side.cubic, t)), sideTangent(side.cubic, t)) > 0;
}

/**
 * The parameter of the foot of the perpendicular from a point to a side, near the corner: the
 * first place where the point stops lying ahead along the side's tangent, found in a coarse table
 * then by bisection, a curve turning back being allowed (Euclid III.18, `REF-EUCLID-III18`;
 * `REF-POMAX-BEZIER`, "Projecting a point onto a Bézier curve").
 *
 * Formula: the first root of (P − B(t)) · T(t) on [0, end], T the unit tangent; 0 when the point
 * lies behind the corner.
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param point - the point projected
 * @param end - the largest parameter looked at
 * @returns the foot's parameter, 0 when the point lies behind the corner, `end` when beyond
 * @see DERIV-fillet-beside-curve
 */
export function footOnSide(side: FilletSide, point: Point, end: number): number {
  if (!isAheadAt(side, point, 0)) {
    return 0;
  }

  const [first] = crossings((t) => isAheadAt(side, point, t), end);

  return first ?? end;
}
