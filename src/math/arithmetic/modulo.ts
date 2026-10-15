/**
 * Remainder of a division taking the sign of the divisor: the position of a number on a cycle of
 * that length, so that −1 on a cycle of 4 is 3 (`%` takes the sign of the dividend).
 *
 * Formula: n mod d = ((n % d) + d) % d.
 *
 * @kind math
 * @param dividend - any number
 * @param divisor - length of the cycle, > 0; 0 gives NaN
 * @returns the remainder, from 0 included to the divisor excluded
 * @see REF-MDN-REMAINDER
 */
export function modulo(dividend: number, divisor: number): number {
  return ((dividend % divisor) + divisor) % divisor;
}
