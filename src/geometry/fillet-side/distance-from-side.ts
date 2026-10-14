import { scalarProjection } from "../../math";
import { cubicPoint } from "../placement";

import { footOnSide } from "./foot-on-side";
import { sideNormal } from "./side-normal";

import type { FilletSide } from "./fillet-side";
import type { Point } from "../../math";

/**
 * Signed distance from a point to a side near the corner, positive on its fillet's side: along
 * the normal at the foot of the perpendicular (`DERIV-fillet-beside-curve` step 10,
 * `REF-MATHWORLD-DOT`); from a straight side, along its one normal (step 3).
 *
 * Formula: (P − A(tₐ)) · n_A(tₐ), tₐ the foot.
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param point - the point measured, a fillet's centre
 * @param end - the largest parameter looked at
 * @returns the signed distance
 * @see DERIV-fillet-beside-curve
 */
export function distanceFromSide(side: FilletSide, point: Point, end: number): number {
  // A straight side's normal is the same everywhere: no foot to look for.
  const foot = side.isStraight ? 0 : footOnSide(side, point, end);

  return scalarProjection(point, cubicPoint(side.cubic, foot), sideNormal(side, foot));
}
