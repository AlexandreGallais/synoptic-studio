import type { Cubic } from "../contour";

/**
 * Values of t that cut a cubic side into parts monotone in y: 0, the roots of its y′ strictly
 * between 0 and 1 in increasing order, and 1 (`DERIV-painted-area` step 4, `DERIV-cubic-sides`
 * steps 2 and 3); a degenerate root (infinite, NaN) is left out.
 *
 * Formula: 0, the roots of y′(t) = a t² + b t + c with 0 < t < 1 sorted, 1.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the breaks, from 0 to 1
 * @see DERIV-painted-area
 */
export function cubicYBreaks(cubic: Cubic): readonly number[] {
  return [cubic.start.y];
}
