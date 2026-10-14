import { discriminant } from "./discriminant";

import type { Quadratic } from "./quadratic";

/**
 * Roots of `a t² + b t + c`, taken without cancellation; a degenerate case (a = 0, D < 0) gives
 * infinite or NaN values, left to the caller to filter (`DERIV-cubic-sides` step 3).
 *
 * Formula: q = −(b + sign(b) √(b² − 4ac)) / 2, t₁ = q / a, t₂ = c / q, sign(0) = +1.
 *
 * @kind math
 * @param quadratic - its coefficients
 * @returns the two values t₁ and t₂, possibly infinite or NaN
 * @see REF-WIKI-QUADRATIC
 */
export function quadraticRoots(quadratic: Quadratic): readonly [number, number] {
  const { a, b, c } = quadratic;
  const q = -(b + (b < 0 ? -1 : 1) * Math.sqrt(discriminant(quadratic))) / 2;

  return [q / a, c / q];
}
