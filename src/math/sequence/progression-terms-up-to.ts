import { EPSILON } from "../precision";

/**
 * How many terms of an increasing arithmetic progression stay at or below a bound: the largest n
 * whose term `a₁ + (n − 1) · d` is not above it, a term above it by a rounding error (`EPSILON`)
 * counted in. From 16 by 20 up to 60, the terms 16, 36 and 56 are three.
 *
 * Formula: max{n ≥ 1 : a₁ + (n − 1) · d ≤ b} = ⌊(b − a₁) / d + ε⌋ + 1, for d > 0 and a₁ ≤ b.
 *
 * @kind math
 * @param first - the first term, a₁, at most the bound
 * @param difference - the common difference, d > 0
 * @param bound - the bound, b
 * @returns the number of terms up to the bound, 1 or more
 * @see REF-WIKI-ARITHMETIC-PROGRESSION
 */
export function progressionTermsUpTo(first: number, difference: number, bound: number): number {
  return Math.floor((bound - first) / difference + EPSILON) + 1;
}
