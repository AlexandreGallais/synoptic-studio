/** A whole, in percent. */
const PERCENT = 100;

/**
 * A percentage as a fraction, as CSS's `hslToRgb` divides the saturation and lightness by 100
 * (REF-CSS-COLOR-4, §7.1).
 *
 * Formula: x / 100.
 *
 * @kind math
 * @param percent - an amount in percent
 * @returns the same amount as a fraction, 1 being the whole
 * @see REF-CSS-COLOR-4
 */
export function fractionFromPercent(percent: number): number {
  return percent / PERCENT;
}
