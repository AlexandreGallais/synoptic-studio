import { nearestInteger } from "../arithmetic";

import type { IntegerRatio } from "./integer-ratio";

/**
 * An exact fraction from 0 to 1 as a whole number of a scale, the nearest, a tie up: a byte on the
 * scale 255, a percent on the scale 100. `k · N` stays an integer below 2⁵³ and the quotient is
 * correctly rounded, so a half is seen as a half and any other value lies at least `1 / (2D)`
 * from one (`DERIV-color-spaces`, steps 11 and 12): 1 / 10 on 255 is 25.5, which gives 26.
 *
 * Formula: round(k · N / D).
 *
 * @kind math
 * @param ratio - the value, from 0 to 1, as a numerator and a denominator
 * @param scale - the whole number a ratio of 1 gives, k
 * @returns the nearest whole number of the scale
 * @see DERIV-color-spaces
 */
export function ratioOnScale(ratio: IntegerRatio, scale: number): number {
  return nearestInteger((scale * ratio.numerator) / ratio.denominator);
}
