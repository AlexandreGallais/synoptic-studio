import type { IntegerRatio } from "./integer-ratio";

/**
 * The value of an exact fraction, its numerator divided by its denominator, correctly rounded to
 * the nearest double: a hue of 630 / 3 is 210 (`DERIV-color-spaces`, step 9).
 *
 * Formula: N / D.
 *
 * @kind math
 * @param ratio - a fraction of integers
 * @returns its value
 * @see DERIV-color-spaces
 */
export function ratioValue(ratio: IntegerRatio): number {
  return ratio.numerator / ratio.denominator;
}
