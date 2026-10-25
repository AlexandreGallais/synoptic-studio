import { hexToExactColor } from "./hex-to-exact-color";
import { toWholeColor } from "./to-whole-color";

import type { ColorModel, PickedColor } from "../../model";

/**
 * The values a picker shows for a stored color in a model, whole numbers read from its exact
 * values (`hexToExactColor`, `toWholeColor`); a grey, which has no hue of its own, keeps the hue
 * the picker holds, so that a grey made lighter or darker keeps it while the picker is open
 * (ADR-0056, decision 3; ADR-0058, decision 1). `#ff8000` is 30°, 100 %, 100 % in HSB and 30°,
 * 100 %, 50 % in HSL; `#808080` with the hue 200° held is 200°, 0 %, 50 % in HSB.
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @param model - the model to show it in
 * @param keptHue - the hue the picker holds, an integer, 0 when it opens
 * @returns the hue, the saturation and the lightness or brightness, integers
 */
export function hexToPickedColor(color: string, model: ColorModel, keptHue: number): PickedColor {
  return toWholeColor(hexToExactColor(color, model, keptHue));
}
