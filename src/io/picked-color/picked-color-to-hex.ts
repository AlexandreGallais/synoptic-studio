import { PERCENT } from "../../math";

import { exactColorToHex } from "./exact-color-to-hex";

import type { ColorModel, PickedColor } from "../../model";

/**
 * The color of three whole values read in a model, each an exact fraction (a hue `h / 1`, a
 * percent `x / 100`), turned into bytes exactly (`exactColorToHex`), so that a byte lying on a
 * half rounds up as the domain says (ADR-0056; ADR-0058; `DERIV-color-spaces`, steps 11 and 12).
 * HSB 210°, 100 %, 50 % is `#004080`; HSB 210°, 67 %, 61 % is `#33679c`. A picker editing one
 * value of a color rebuilds it with `wholeValuesToHex` instead, the others exact.
 *
 * @kind format
 * @param picked - the hue, the saturation and the lightness or brightness, integers
 * @param model - the model the values are read in
 * @returns the color, `#` and six lower-case digits
 */
export function pickedColorToHex(picked: PickedColor, model: ColorModel): string {
  return exactColorToHex(
    {
      hue: { denominator: 1, numerator: picked.hue },
      level: { denominator: PERCENT, numerator: picked.level },
      saturation: { denominator: PERCENT, numerator: picked.saturation },
    },
    model,
  );
}
