import { toOneDecimal } from "./to-one-decimal";
import { toShownHue } from "./to-shown-hue";

import type { Hsl } from "../../math";

/**
 * A color's HSL as shown: each value to one decimal, the hue 360.0 shown 0.0; `#ff8000` is shown
 * 30.1°, 100 %, 50 %. Typed back, every color gives its hexadecimal again (`DERIV-color-spaces`,
 * step 9; ADR-0056).
 *
 * @kind format
 * @param hsl - the color's HSL, unrounded
 * @returns its HSL to one decimal
 * @see DERIV-color-spaces
 */
export function toShownHsl(hsl: Hsl): Hsl {
  return {
    hue: toShownHue(hsl.hue),
    lightness: toOneDecimal(hsl.lightness),
    saturation: toOneDecimal(hsl.saturation),
  };
}
