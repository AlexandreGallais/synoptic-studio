/**
 * The remainder of a non-negative dividend by a positive divisor: its position on a cycle of that
 * length, as `modulo` gives it, for "two values of the same sign, the two are equivalent"
 * (REF-MDN-REMAINDER), but exact: `%` takes the real remainder `n − d · trunc(n / d)`
 * (REF-ECMA-262-NUMBER-REMAINDER), so that 30.1 on a cycle of 360 stays 30.1, where `modulo`'s
 * added divisor gives 30.100000000000023.
 *
 * Formula: n − d · ⌊n / d⌋, for n ≥ 0 and d > 0.
 *
 * @kind math
 * @param dividend - a number ≥ 0
 * @param divisor - length of the cycle, > 0
 * @returns the remainder, from 0 included to the divisor excluded
 * @see REF-ECMA-262-NUMBER-REMAINDER
 */
export function nonNegativeRemainder(dividend: number, divisor: number): number {
  return dividend % divisor;
}
