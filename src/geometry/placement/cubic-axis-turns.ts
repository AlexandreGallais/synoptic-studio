import { quadraticRoots } from "../../math";

import { derivativeCoefficients } from "./derivative-coefficients";

/**
 * Values of t strictly between 0 and 1 where one axis of a cubic turns: the roots of its
 * derivative there (`DERIV-cubic-sides` steps 2 and 3); a degenerate root (infinite, NaN) fails
 * the test and is left out.
 *
 * Formula: the roots of w′(t) = a t² + b t + c with 0 < t < 1, in the order `quadraticRoots` gives.
 *
 * @kind geometry
 * @param weights - the four values of the axis, from the start to the end
 * @returns those values of t, none, one or two
 * @see DERIV-cubic-sides
 */
export function cubicAxisTurns(
  weights: readonly [number, number, number, number],
): readonly number[] {
  return quadraticRoots(derivativeCoefficients(weights)).filter((t) => t > 0 && t < 1);
}
