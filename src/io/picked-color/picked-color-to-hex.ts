import { exactColorToHex } from "./exact-color-to-hex";
import { toTenths } from "./to-tenths";

import type { ColorModel, PickedColor } from "../../model";

/** Tenths in a unit: a hue in degrees, given to one decimal, is its tenths over 10. */
const TENTHS = 10;

/** Tenths of a percent in a whole: a percent to one decimal is its tenths over 1000. */
const TENTHS_OF_PERCENT = 1000;

/**
 * The stored color of the values an open picker holds, read in its model: each value, given to
 * one decimal, counted in tenths (`toTenths`) as an exact fraction, then turned into bytes
 * exactly (`exactColorToHex`), so that a byte lying on a half rounds up as the domain says
 * (ADR-0056, decision 3; `DERIV-color-spaces`, step 11). HSB 210°, 100 %, 50 % is `#004080`;
 * HSB 30.1°, 90 %, 100 % is `#ff8d1a`.
 *
 * @kind format
 * @param picked - the hue, the saturation and the lightness or brightness, to one decimal
 * @param model - the model the values are read in
 * @returns the color, `#` and six lower-case digits
 * @see DERIV-color-spaces
 */
export function pickedColorToHex(picked: PickedColor, model: ColorModel): string {
  return exactColorToHex(
    {
      hue: { denominator: TENTHS, numerator: toTenths(picked.hue) },
      level: { denominator: TENTHS_OF_PERCENT, numerator: toTenths(picked.level) },
      saturation: { denominator: TENTHS_OF_PERCENT, numerator: toTenths(picked.saturation) },
    },
    model,
  );
}
