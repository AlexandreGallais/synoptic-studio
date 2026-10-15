import { modulo } from "./modulo";

/**
 * The integer nearest the quotient of two integers, a tie rounded up, computed with integers only,
 * so that no binary fraction enters: `255 · 50 / 100 = 127.5` gives 128, while `50 × 2.55` in
 * binary64 is `127.49999999999999` (`DERIV-color-spaces`, step 2).
 *
 * Formula: round(n / d) = ⌊(2n + d) / 2d⌋ for d > 0, the floor taken as (t − t mod 2d) / 2d with
 * t mod 2d in [0, 2d), exact while |2n + d| ≤ Number.MAX_SAFE_INTEGER.
 *
 * @kind math
 * @param dividend - an integer, |2 · dividend + divisor| ≤ Number.MAX_SAFE_INTEGER
 * @param divisor - an integer > 0
 * @returns the nearest integer to dividend / divisor, a tie rounded toward +∞
 * @see DERIV-color-spaces
 */
export function roundedQuotient(dividend: number, divisor: number): number {
  const doubledDividend = 2 * dividend + divisor;
  const doubledDivisor = 2 * divisor;

  return (doubledDividend - modulo(doubledDividend, doubledDivisor)) / doubledDivisor;
}
