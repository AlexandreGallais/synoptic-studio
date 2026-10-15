import { nearestInteger } from "../../math";

/** The largest byte, `ff`: a component of 1. */
const BYTE_MAX = 255;

/**
 * The byte of a color component given as a fraction, `255 · x` rounded to the closest integer, a
 * tie rounded up: an sRGB component "must be rounded towards +∞, not truncated" (REF-CSS-COLOR-4,
 * §16.2.2), a tie toward +∞ (REF-CSS-VALUES-4, §5.2; `DERIV-color-spaces`, step 8).
 * HSL(30°, 100 %, 50 %)'s green, 0.5, is 127.5 and gives 128.
 *
 * Formula: round(255 · x).
 *
 * @kind format
 * @param fraction - the component, from 0 to 1
 * @returns the byte, an integer from 0 to 255
 * @see REF-CSS-COLOR-4
 */
export function fractionToByte(fraction: number): number {
  return nearestInteger(BYTE_MAX * fraction);
}
