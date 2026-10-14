/**
 * Midpoint of an interval, the mean of its two ends, as a bisection halves it. Halving is exact in
 * binary floating point unless the sum is subnormal: the only rounding is the sum's.
 *
 * Formula: (a + b) / 2.
 *
 * @kind math
 * @param first - one number
 * @param second - the other
 * @returns their mean
 * @see REF-MATHWORLD-MIDPOINT
 */
export function average(first: number, second: number): number {
  return (first + second) / 2;
}
