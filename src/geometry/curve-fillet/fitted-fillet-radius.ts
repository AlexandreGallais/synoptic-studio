import { bisectionBracket, regularPartition } from "../../math";
import { cyclicItem } from "../contour";
import { FILLET_STEPS } from "../fillet-side";

import { filletTangency } from "./fillet-tangency";

import type { FilletSide } from "../fillet-side";

/** Radii tried below the one asked, from the largest: a coarse table before the bisection. */
const STEPS = 16;

/**
 * Whether a fillet of a radius is drawn at a corner: a radius 0 always, any other when a valid
 * tangency exists (`filletTangency`).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius tried, ≥ 0
 * @returns `true` when that radius can be drawn
 * @see ADR-0047
 */
function isFitting(sides: readonly [FilletSide, FilletSide], radius: number): boolean {
  return radius === 0 || filletTangency(sides, radius) !== undefined;
}

/**
 * The largest radius below one asked that fits: the first that fits in a table descending from
 * it, then a bisection between it and the one above, keeping the lower end, seen to fit
 * (`REF-MATHWORLD-BISECTION`, ADR-0047).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius asked, which does not fit
 * @returns a radius that fits, from 0
 * @see DERIV-fillet-beside-curve
 */
function largestFitting(sides: readonly [FilletSide, FilletSide], radius: number): number {
  // The radii tried, from the radius asked down to 0.
  const table = regularPartition(radius, STEPS).toReversed();
  const first = table.findIndex((tried) => isFitting(sides, tried));

  return bisectionBracket(
    (tried) => isFitting(sides, tried),
    [cyclicItem(table, first, 0), cyclicItem(table, first - 1, radius)],
    FILLET_STEPS,
  )[0];
}

/**
 * The radius a corner beside a curve is drawn with: the one asked when it fits, else the largest
 * that fits, never a sharp corner (ADR-0047, `DERIV-fillet-beside-curve` step 11,
 * `REF-ASTUTE-CORNERS`); 0 when the sides leave along one line.
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius asked, ≥ 0
 * @returns the radius drawn, one that fits, from 0 to the one asked
 * @see ADR-0047
 */
export function fittedFilletRadius(
  sides: readonly [FilletSide, FilletSide],
  radius: number,
): number {
  if (radius <= 0 || sides[0].sign === 0) {
    return 0;
  }

  return isFitting(sides, radius) ? radius : largestFitting(sides, radius);
}
