import { hsbComponentInTenths, hslComponentInTenths } from "../../math";
import { bytesToHex } from "../color";

import { ratioToByte } from "./ratio-to-byte";
import { toTenths } from "./to-tenths";

import type { IntegerRatio, TenthsColor } from "../../math";
import type { ColorModel, PickedColor } from "../../model";

/** The `n` of HSB's red in `f(n)`. */
const HSB_RED = 5;

/** The `n` of HSB's green. */
const HSB_GREEN = 3;

/** The `n` of HSB's blue. */
const HSB_BLUE = 1;

/** The `n` of HSL's red in `f(n)`. */
const HSL_RED = 0;

/** The `n` of HSL's green. */
const HSL_GREEN = 8;

/** The `n` of HSL's blue. */
const HSL_BLUE = 4;

/** Each model's exact component and the `n` of its red, green and blue in `f(n)`. */
const MODELS: Readonly<
  Record<
    ColorModel,
    {
      /** The component as an exact fraction. */
      readonly component: (color: TenthsColor, n: number) => IntegerRatio;
      /** The `n` of red, green and blue. */
      readonly components: readonly number[];
    }
  >
> = {
  hsb: { component: hsbComponentInTenths, components: [HSB_RED, HSB_GREEN, HSB_BLUE] },
  hsl: { component: hslComponentInTenths, components: [HSL_RED, HSL_GREEN, HSL_BLUE] },
};

/**
 * The stored color of the values an open picker holds, read in its model: each value, given to
 * one decimal, counted in tenths (`toTenths`), each component an exact fraction
 * (`hsbComponentInTenths`, `hslComponentInTenths`) and its byte rounded to the closest, a tie up
 * (`ratioToByte`), so that a byte lying on a half rounds up as the domain says (ADR-0056, decision
 * 3; `DERIV-color-spaces`, step 11). HSB 210°, 100 %, 50 % is `#004080`; HSB 30.1°, 90 %, 100 %
 * is `#ff8d1a`.
 *
 * @kind format
 * @param picked - the hue, the saturation and the lightness or brightness, to one decimal
 * @param model - the model the values are read in
 * @returns the color, `#` and six lower-case digits
 * @see DERIV-color-spaces
 */
export function pickedColorToHex(picked: PickedColor, model: ColorModel): string {
  const { component, components } = MODELS[model];
  const tenths = {
    hue: toTenths(picked.hue),
    level: toTenths(picked.level),
    saturation: toTenths(picked.saturation),
  };

  return bytesToHex(components.map((n) => ratioToByte(component(tenths, n))));
}
