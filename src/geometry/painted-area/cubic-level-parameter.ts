import type { Cubic } from "../contour";

/**
 * The t at which a part of a cubic side monotone in y reaches a level its ends bracket, found by
 * bisection: y − level changes sign between the part's ends (`DERIV-painted-area` step 4).
 *
 * Formula: t in [a, b] where (y(t) − level)(y(b) − y(a)) turns from negative to non-negative.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param span - the part's first and last t, y monotone between them
 * @param level - the ordinate reached, between y(a) and y(b)
 * @returns the t found, within a rounding error
 * @see REF-MATHWORLD-BISECTION
 */
export function cubicLevelParameter(
  cubic: Cubic,
  span: readonly [number, number],
  level: number,
): number {
  return cubic.start.y === level ? span[0] : 0;
}
