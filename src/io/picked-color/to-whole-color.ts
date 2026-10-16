import { PERCENT, ratioOnScale, ratioValue } from "../../math";
import { toShownHue } from "../color";

import type { ExactColor } from "../../math";
import type { PickedColor } from "../../model";

/**
 * A color's HSB or HSL as shown and typed, whole numbers: each exact value rounded to the nearest
 * integer, a tie up, a hue that rounds to 360 shown 0 (`toShownHue`, its quotient correctly
 * rounded, `ratioValue`), the saturation and the level on a scale of 100 (`ratioOnScale`), the
 * decimals kept inside the exact color only. `#ff8000` is shown HSB 30°, 100 %, 100 %, `#336699`
 * 210°, 67 %, 60 % (ADR-0058, decision 1; `DERIV-color-spaces`, step 9).
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
    hue: toShownHue(ratioValue(color.hue)),
    level: ratioOnScale(color.level, PERCENT),
    saturation: ratioOnScale(color.saturation, PERCENT),
  };
}
