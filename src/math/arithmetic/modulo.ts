/**
 * Remainder of a division taking the sign of the divisor: the position of a number on a cycle of
 * that length, so that −1 on a cycle of 4 is 3 (`%` takes the sign of the dividend). A remainder
 * below 0 gets the divisor added once; one from 0 is kept as it is, `%` being exact in binary64,
 * so that 30.1 mod 360 is 30.1 (MDN's `((n % d) + d) % d` gives 30.100000000000023); adding +0
 * turns −0 to 0 (REF-ECMA-262-NUMBER-ADD).
 *
 * Formula: n mod d = r + d when r < 0, else r + 0, with r = n % d.
 *
 * @kind math
 * @param dividend - any number
 * @param divisor - length of the cycle, > 0; 0 gives NaN
 * @returns the remainder, from 0 included to the divisor excluded; the divisor itself only for a
 * dividend below 0 by less than half a unit in the last place of the divisor
 * @see REF-MDN-REMAINDER
 */
export function modulo(dividend: number, divisor: number): number {
  const remainder = dividend % divisor;

  return remainder + (remainder < 0 ? divisor : 0);
}
