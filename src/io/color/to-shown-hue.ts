import { DEGREES_PER_TURN, nonNegativeRemainder } from "../../math";

import { toOneDecimal } from "./to-one-decimal";

/**
 * A hue as shown, to one decimal, from 0.0 to 359.9: a hue that rounds to 360.0 is shown 0.0, the
 * same direction (`DERIV-color-spaces`, step 9). No color read from a hexadecimal has such a hue,
 * the largest being 359.76471° (`#ff0001`); a hue typed or picked may.
 *
 * Formula: round(10 · H) / 10 mod 360.
 *
 * @kind format
 * @param hue - the hue, in degrees, from 0 to 360
 * @returns the hue to one decimal, from 0 to 359.9
 * @see DERIV-color-spaces
 */
export function toShownHue(hue: number): number {
  return nonNegativeRemainder(toOneDecimal(hue), DEGREES_PER_TURN);
}
