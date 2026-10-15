import { hsbToRgb, hslToRgb } from "../../math";

import { rgbToHex } from "./rgb-to-hex";

import type { ColorModel, PickedColor } from "../../model";

/**
 * The stored color of the values an open picker holds, read in its model: HSB's brightness or
 * HSL's lightness, each byte rounded to the closest, a tie up (ADR-0056, decision 3;
 * `DERIV-color-spaces`, steps 6 and 8). HSB 210°, 100 %, 50 % is `#004080`; HSB 210°, 66.7 %,
 * 60 % is `#336699`.
 *
 * @kind format
 * @param picked - the hue, the saturation and the lightness or brightness
 * @param model - the model the values are read in
 * @returns the color, `#` and six lower-case digits
 * @see DERIV-color-spaces
 */
export function pickedColorToHex(picked: PickedColor, model: ColorModel): string {
  const { hue, level, saturation } = picked;

  return rgbToHex(
    model === "hsb"
      ? hsbToRgb({ brightness: level, hue, saturation })
      : hslToRgb({ hue, lightness: level, saturation }),
  );
}
