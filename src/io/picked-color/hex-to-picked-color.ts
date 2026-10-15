import { chroma, rgbToHsb, rgbToHsl } from "../../math";
import { hexToRgb, toShownHsb, toShownHsl } from "../color";

import type { Rgb } from "../../math";
import type { ColorModel, PickedColor } from "../../model";

/**
 * A color's HSB or HSL as shown, to one decimal, its brightness or lightness as the picker's
 * level (`toShownHsb`, `toShownHsl`; `DERIV-color-spaces`, step 9).
 *
 * @kind format
 * @param rgb - the color's components, from 0 to 1
 * @param model - the model to show it in
 * @returns its hue, saturation and lightness or brightness, to one decimal
 * @see DERIV-color-spaces
 */
function toShownPickedColor(rgb: Rgb, model: ColorModel): PickedColor {
  if (model === "hsb") {
    const hsb = toShownHsb(rgbToHsb(rgb));

    return { hue: hsb.hue, level: hsb.brightness, saturation: hsb.saturation };
  }

  const hsl = toShownHsl(rgbToHsl(rgb));

  return { hue: hsl.hue, level: hsl.lightness, saturation: hsl.saturation };
}

/**
 * The values a picker shows for a stored color in a model, each to one decimal; a grey, which has
 * no hue of its own, keeps the hue the picker holds, so that a grey made lighter or darker keeps
 * it while the picker is open (ADR-0056, decision 3; `DERIV-color-spaces`, steps 3 and 9).
 * `#ff8000` is 30.1°, 100 %, 100 % in HSB and 30.1°, 100 %, 50 % in HSL; `#808080` with the hue
 * 200° held is 200°, 0 %, 50.2 % in HSB.
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @param model - the model to show it in
 * @param keptHue - the hue the picker holds, 0 when it opens
 * @returns the hue, the saturation and the lightness or brightness
 * @see DERIV-color-spaces
 */
export function hexToPickedColor(color: string, model: ColorModel, keptHue: number): PickedColor {
  const rgb = hexToRgb(color);
  const shown = toShownPickedColor(rgb, model);

  return chroma(rgb) === 0 ? { ...shown, hue: keptHue } : shown;
}
