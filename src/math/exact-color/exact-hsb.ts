import { brightness, chroma } from "../color";

import { exactHue } from "./exact-hue";
import { saturationRatio } from "./saturation-ratio";

import type { ByteRgb } from "./byte-rgb";
import type { ExactColor } from "./exact-color";

/** The largest byte, `ff`: a component of 1. */
const BYTE_MAX = 255;

/**
 * A color's HSB read from its bytes as exact fractions: the hue (`exactHue`), the saturation
 * `C / M`, 0 for black (`saturationRatio`), and the brightness `M / 255`, so that `#336699` is 210°, 102 / 153 and
 * 153 / 255, that is 66.66667 % and 60 %, nothing rounded (REF-WIKI-HSL-HSV, From RGB:
 * `V = max(R, G, B)`, `S_V = C / V`; `DERIV-color-spaces`, steps 5 and 12).
 *
 * Formula: H = exactHue, S = C / M (0 / 1 when M = 0), V = M / 255.
 *
 * @kind math
 * @param bytes - the color's bytes, integers from 0 to 255
 * @returns its hue in degrees, its saturation and brightness from 0 to 1
 * @see DERIV-color-spaces
 */
export function exactHsb(bytes: ByteRgb): ExactColor {
  const largest = brightness(bytes);

  return {
    hue: exactHue(bytes),
    level: { denominator: BYTE_MAX, numerator: largest },
    saturation: saturationRatio(chroma(bytes), largest),
  };
}
