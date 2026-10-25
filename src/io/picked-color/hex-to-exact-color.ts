import { chroma, exactHsb, exactHsl } from "../../math";
import { hexToBytes } from "../color";

import type { ExactColor } from "../../math";
import type { ColorModel } from "../../model";

/**
 * A stored color's exact values in a model, read from its bytes (`exactHsb`, `exactHsl`); a grey,
 * which has no hue of its own, takes the hue the picker holds (ADR-0056, decision 3; ADR-0058,
 * decision 2). `#336699` is HSB 210°, 102 / 153, 153 / 255; `#808080` with the hue 200° held is
 * 200°, 0, 128 / 255.
 *
 * @kind format
 * @param color - the stored color, already checked (`isColor`)
 * @param model - the model to read it in
 * @param keptHue - the hue the picker holds, an integer in degrees, 0 when it opens
 * @returns its hue, saturation and lightness or brightness as exact fractions
 */
export function hexToExactColor(color: string, model: ColorModel, keptHue: number): ExactColor {
  const bytes = hexToBytes(color);
  const exact = model === "hsb" ? exactHsb(bytes) : exactHsl(bytes);

  return chroma(bytes) === 0 ? { ...exact, hue: { denominator: 1, numerator: keptHue } } : exact;
}
