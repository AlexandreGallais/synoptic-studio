import { BYTE_MAXIMUM, PERCENT, ratioOnScale } from "../../math";

/**
 * The opacity, in whole percent, of the alpha byte of an 8- or 4-digit hex color, rounded to the
 * nearest percent, a tie rounded up: `cc` (204) is 80 %, `ec` (236) 93 %. The 256 bytes give the
 * 101 percents, so a byte that no percent gives is rounded (`DERIV-color-spaces`, step 2).
 *
 * Formula: p = round(100 · A / 255) (`ratioOnScale`), a quotient of integers, whose tie binary64
 * keeps exact.
 *
 * @kind format
 * @param alpha - the alpha byte, an integer from 0 to 255
 * @returns the opacity, an integer from 0 to 100
 * @see DERIV-color-spaces
 */
export function alphaToPercent(alpha: number): number {
  return ratioOnScale({ denominator: BYTE_MAXIMUM, numerator: alpha }, PERCENT);
}
