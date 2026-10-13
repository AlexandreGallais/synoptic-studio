import { bisection } from "../../math";

import { isFilletFitting } from "./is-fillet-fitting";

import type { FilletSide } from "./fillet-side";

/**
 * The radius a corner beside a curve is drawn with: the one asked when it fits, else the largest
 * that fits, never a sharp corner (ADR-0047; `DERIV-fillet-beside-curve` step 11;
 * `REF-ASTUTE-CORNERS`); 0 when the sides leave along one line.
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius asked, ≥ 0
 * @returns the radius drawn, from 0 to the one asked
 * @see ADR-0047
 */
export function fittedFilletRadius(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
): number {
  if (radius <= 0 || sides[0].sign === 0) {
    return 0;
  }

  return isFilletFitting(sides, radius)
    ? radius
    : bisection((tried) => isFilletFitting(sides, tried), 0, radius);
}
