import { DEGREES_PER_TURN, PERCENT, nonNegativeRemainder } from "../../math";

import { exactColorToHex } from "./exact-color-to-hex";
import { toWholeColor } from "./to-whole-color";

import type { ExactColor, IntegerRatio } from "../../math";
import type { ColorModel, PickedColor } from "../../model";

/** Each value's unit as a fraction's denominator: degrees for the hue, percent for the others. */
const UNITS: Readonly<Record<keyof PickedColor, number>> = {
  hue: 1,
  level: PERCENT,
  saturation: PERCENT,
};

/**
 * One value for the rebuilt color: the exact value when the integer given is the one shown (a
 * hue of 360 being the 0 shown), else that integer over its unit (ADR-0058, decisions 2 and 3).
 *
 * @kind format
 * @param key - the value: hue, saturation, or lightness or brightness
 * @param exact - the color's exact values
 * @param given - the integers given, one per value
 * @returns the value as an exact fraction
 * @see DERIV-color-spaces
 */
function toRebuiltValue(
  key: keyof PickedColor,
  exact: ExactColor,
  given: PickedColor,
): IntegerRatio {
  const compared = key === "hue" ? nonNegativeRemainder(given.hue, DEGREES_PER_TURN) : given[key];

  return compared === toWholeColor(exact)[key]
    ? exact[key]
    : { denominator: UNITS[key], numerator: given[key] };
}

/**
 * The color of integers typed, keyed or dragged in a picker: a value equal to the one shown
 * (`toWholeColor`) stands for its exact value, so that the color is rebuilt from the integer
 * edited and the exact values of the others, and a field left as it is changes nothing; the bytes
 * follow exactly (`exactColorToHex`; ADR-0058, decisions 2 and 3; `DERIV-color-spaces`, step 12).
 * `#336699`, shown HSB 210, 67, 60, its brightness given 61: 210°, 102 / 153, 61 / 100, so
 * `#34689c`, where 210, 67, 61 would give `#33679c`; given 210, 67, 60, it stays `#336699`.
 *
 * @kind format
 * @param exact - the color's exact values (`hexToExactColor`)
 * @param model - the model the values are read in
 * @param given - the hue from 0 to 360, the saturation and lightness or brightness from 0 to 100,
 * integers
 * @returns the color, `#` and six lower-case digits
 * @see DERIV-color-spaces
 */
export function wholeValuesToHex(exact: ExactColor, model: ColorModel, given: PickedColor): string {
  return exactColorToHex(
    {
      hue: toRebuiltValue("hue", exact, given),
      level: toRebuiltValue("level", exact, given),
      saturation: toRebuiltValue("saturation", exact, given),
    },
    model,
  );
}
