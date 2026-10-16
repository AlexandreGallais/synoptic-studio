import { percentFromFraction } from "../arithmetic";

import { hslSaturation } from "./hsl-saturation";
import { hue } from "./hue";
import { lightness } from "./lightness";

import type { Rgb } from "./rgb";

/** A color in HSL, as CSS writes it: the hue in degrees, the saturation and lightness in percent. */
export type Hsl = {
  /** The hue, in degrees, from 0 included to 360 excluded; 0 for a grey. */
  readonly hue: number;
  /** The saturation, in percent, from 0 to 100. */
  readonly saturation: number;
  /** The lightness, in percent, from 0 (black) to 100 (white). */
  readonly lightness: number;
};

/**
 * A color in HSL from its red, green and blue, unrounded: `#ff8000` is 30.11765°, 100 %, 50 %
 * (REF-CSS-COLOR-4, §7.2, `rgbToHsl`, which returns the saturation and lightness times 100;
 * `DERIV-color-spaces`, steps 3 and 4). Computed, never stored (ADR-0056).
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the hue in degrees, the saturation and lightness in percent
 * @see REF-CSS-COLOR-4
 */
export function rgbToHsl(rgb: Rgb): Hsl {
  return {
    hue: hue(rgb),
    lightness: percentFromFraction(lightness(rgb)),
    saturation: percentFromFraction(hslSaturation(rgb)),
  };
}
