import { brightness, chroma, smallestComponent } from "../color";

import { BYTE_MAXIMUM } from "./byte-maximum";
import { exactHue } from "./exact-hue";
import { exactLargestHalfChroma } from "./exact-largest-half-chroma";
import { saturationRatio } from "./saturation-ratio";

import type { ByteRgb } from "./byte-rgb";
import type { ExactColor } from "./exact-color";

/** Twice the largest byte: the lightness `(M + m) / 2` is `(M + m) / 510` in bytes. */
const TWO_BYTES = 2 * BYTE_MAXIMUM;

/**
 * A color's HSL read from its bytes as exact fractions: the hue (`exactHue`), the lightness
 * `(M + m) / 510` and the saturation, the half chroma out of the largest half chroma the
 * lightness allows (`exactLargestHalfChroma`), `C / min(M + m, 510 − M − m)` in bytes, 0 for black
 * and white (`saturationRatio`); `#336699` is 210°, 102 / 204 and 204 / 510, that is 50 % and 40 %
 * (REF-CSS-COLOR-4, §7.2: `light = (min + max) / 2`, `sat = (max − light) / min(light, 1 − light)`;
 * `DERIV-color-spaces`, steps 4 and 12).
 *
 * Formula: H = exactHue, L = (M + m) / 510, S = C / min(M + m, 510 − (M + m)), 0 / 1 when that
 * is 0.
 *
 * @kind math
 * @param bytes - the color's bytes, integers from 0 to 255
 * @returns its hue in degrees, its saturation and lightness from 0 to 1
 * @see DERIV-color-spaces
 */
export function exactHsl(bytes: ByteRgb): ExactColor {
  const level = { denominator: TWO_BYTES, numerator: brightness(bytes) + smallestComponent(bytes) };

  return {
    hue: exactHue(bytes),
    level,
    // C / 510 out of min(L, 1 − L) over 510: the denominators cancel.
    saturation: saturationRatio(chroma(bytes), exactLargestHalfChroma(level).numerator),
  };
}
