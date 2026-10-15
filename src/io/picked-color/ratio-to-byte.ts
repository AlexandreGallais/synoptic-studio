import { nearestInteger } from "../../math";

import type { IntegerRatio } from "../../math";

/** The largest byte, `ff`: a component of 1. */
const BYTE_MAX = 255;

/**
 * The byte of a component given as an exact fraction of integers, `255 · N / D` rounded to the
 * closest, a tie up (REF-CSS-COLOR-4, §5.1): `255 · N` stays an integer below 2⁵³ and the
 * quotient is correctly rounded, so a half is seen as a half and any other value lies at least
 * `1 / (2D)` from one (`DERIV-color-spaces`, step 11). 0.1 is 25.5, which gives 26.
 *
 * Formula: round(255 · N / D).
 *
 * @kind format
 * @param ratio - the component, from 0 to 1, as a numerator and a denominator
 * @returns the byte, an integer from 0 to 255
 * @see DERIV-color-spaces
 */
export function ratioToByte(ratio: IntegerRatio): number {
  return nearestInteger((BYTE_MAX * ratio.numerator) / ratio.denominator);
}
