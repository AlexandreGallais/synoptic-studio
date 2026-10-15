import { roundedQuotient } from "../../math";

/** The alpha byte of a fully opaque color, `ff`. */
const OPAQUE_ALPHA = 255;

/** The opacity of a fully opaque color, in percent. */
const OPAQUE_PERCENT = 100;

/**
 * The alpha byte of an opacity in whole percent, as CSS maps an alpha percentage `p` to `p × 2.55`
 * rounded to the closest integer, rounding up on a tie, computed with integers: 50 % is 128, not
 * the 127 that `50 × 2.55` gives in binary64. Read back by `alphaToPercent`, each percent comes back
 * (`DERIV-color-spaces`, step 2).
 *
 * Formula: A = round(255 · p / 100) = ⌊(255 · p + 50) / 100⌋.
 *
 * @kind format
 * @param percent - the opacity, an integer from 0 to 100
 * @returns the alpha byte, an integer from 0 to 255
 * @see DERIV-color-spaces
 */
export function percentToAlpha(percent: number): number {
  return roundedQuotient(OPAQUE_ALPHA * percent, OPAQUE_PERCENT);
}
