import { exactHsbComponent, exactHslComponent } from "../../math";
import { bytesToHex } from "../color";

import { ratioToByte } from "./ratio-to-byte";

import type { ExactColor, IntegerRatio } from "../../math";
import type { ColorModel } from "../../model";

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
      readonly component: (color: ExactColor, n: number) => IntegerRatio;
      /** The `n` of red, green and blue. */
      readonly components: readonly number[];
    }
  >
> = {
  hsb: { component: exactHsbComponent, components: [HSB_RED, HSB_GREEN, HSB_BLUE] },
  hsl: { component: exactHslComponent, components: [HSL_RED, HSL_GREEN, HSL_BLUE] },
};

/**
 * The stored color of exact HSB or HSL values: each component an exact fraction
 * (`exactHsbComponent`, `exactHslComponent`) and its byte rounded to the closest, a tie up
 * (`ratioToByte`), so that a byte lying on a half rounds up as the domain says (ADR-0056;
 * `DERIV-color-spaces`, steps 11 and 12). HSB 210°, 100 %, 50 % is `#004080`; `#336699`'s exact
 * 210° and 102 / 153 with the brightness 61 / 100 is `#34689c`.
 *
 * @kind format
 * @param color - the hue, the saturation and the lightness or brightness, as exact fractions
 * @param model - the model the values are read in
 * @returns the color, `#` and six lower-case digits
 * @see DERIV-color-spaces
 */
export function exactColorToHex(color: ExactColor, model: ColorModel): string {
  const { component, components } = MODELS[model];

  return bytesToHex(components.map((n) => ratioToByte(component(color, n))));
}
