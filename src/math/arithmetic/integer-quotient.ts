/**
 * The quotient of a division with remainder: how many whole divisors fit in the dividend, the
 * remainder `dividend mod divisor` left over, from 0 up to the divisor. 13 bytes hold 3 records of
 * 4 bytes.
 *
 * Formula: ⌊x / y⌋, so that x = y · ⌊x / y⌋ + (x mod y).
 *
 * @kind math
 * @param dividend - an integer ≥ 0
 * @param divisor - an integer > 0
 * @returns the quotient, an integer ≥ 0
 * @see REF-WIKI-FLOOR-CEILING
 */
export function integerQuotient(dividend: number, divisor: number): number {
  return Math.floor(dividend / divisor);
}
