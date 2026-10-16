import { PERCENT } from "./percent";

/**
 * A fraction written in percent, as CSS's `rgbToHsl` returns the saturation and lightness times
 * 100 (REF-CSS-COLOR-4, §7.2).
 *
 * Formula: 100 · x.
 *
 * @kind math
 * @param fraction - a fraction, 1 being the whole
 * @returns the same amount in percent
 * @see REF-CSS-COLOR-4
 */
export function percentFromFraction(fraction: number): number {
  return PERCENT * fraction;
}
