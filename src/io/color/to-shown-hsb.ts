import { toOneDecimal } from "./to-one-decimal";
import { toShownHue } from "./to-shown-hue";

import type { Hsb } from "../../math";

/**
 * A color's HSB as shown: each value to one decimal, the hue 360.0 shown 0.0; `#336699` is shown
 * 210°, 66.7 %, 60 %. Typed back, every color gives its hexadecimal again (`DERIV-color-spaces`,
 * step 9; ADR-0056).
 *
 * @kind format
 * @param hsb - the color's HSB, unrounded
 * @returns its HSB to one decimal
 * @see DERIV-color-spaces
 */
export function toShownHsb(hsb: Hsb): Hsb {
  return {
    brightness: toOneDecimal(hsb.brightness),
    hue: toShownHue(hsb.hue),
    saturation: toOneDecimal(hsb.saturation),
  };
}
