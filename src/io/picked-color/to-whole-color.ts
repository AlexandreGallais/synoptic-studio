import { nearestInteger } from "../../math";
import { toShownHue } from "../color";

import type { ExactColor, IntegerRatio } from "../../math";
import type { PickedColor } from "../../model";

/** A whole saturation, lightness or brightness, in percent. */
const PERCENT = 100;

/**
 * An exact fraction from 0 to 1 as a whole percent, the nearest, a tie up: `100 · N` is an
 * integer below 2⁵³ and the quotient is correctly rounded, so a half is seen as a half
 * (`DERIV-color-spaces`, steps 11 and 12).
 *
 * Formula: round(100 · N / D).
 *
 * @kind format
 * @param ratio - the value as a fraction of integers
 * @returns the nearest whole percent
 * @see DERIV-color-spaces
 */
function ratioToPercent(ratio: IntegerRatio): number {
  return nearestInteger((PERCENT * ratio.numerator) / ratio.denominator);
}

/**
 * A color's HSB or HSL as shown and typed, whole numbers: each exact value rounded to the nearest
 * integer, a tie up, a hue that rounds to 360 shown 0 (`toShownHue`, its quotient correctly
 * rounded as `ratioToPercent`'s), the decimals kept inside the exact color only. `#ff8000` is shown HSB 30°, 100 %, 100 %, `#336699` 210°, 67 %,
 * 60 % (ADR-0058, decision 1; `DERIV-color-spaces`, step 9).
 *
 * Formula: H = round(H) mod 360, S = round(100 · S), L or V = round(100 · L or V).
 *
 * @kind format
 * @param color - the color's exact values (`hexToExactColor`)
 * @returns its hue from 0 to 359, its saturation and lightness or brightness from 0 to 100
 * @see DERIV-color-spaces
 */
export function toWholeColor(color: ExactColor): PickedColor {
  return {
    hue: toShownHue(color.hue.numerator / color.hue.denominator),
    level: ratioToPercent(color.level),
    saturation: ratioToPercent(color.saturation),
  };
}
