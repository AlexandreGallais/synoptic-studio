import { percentFromFraction } from "../arithmetic";

import { brightness } from "./brightness";
import { hsbSaturation } from "./hsb-saturation";
import { hue } from "./hue";

import type { Rgb } from "./rgb";

/** A color in HSB (HSV): the hue in degrees, the saturation and brightness in percent. */
export type Hsb = {
  /** The hue, in degrees, from 0 included to 360 excluded; 0 for a grey. */
  readonly hue: number;
  /** The saturation, in percent, from 0 to 100. */
  readonly saturation: number;
  /** The brightness, HSV's value, in percent, from 0 (black) to 100. */
  readonly brightness: number;
};

/**
 * A color in HSB from its red, green and blue, unrounded: `#336699` is 210°, 66.66667 %, 60 %
 * (REF-WIKI-HSL-HSV, From RGB; `DERIV-color-spaces`, steps 3 and 5). Its hue is HSL's; its
 * saturation and brightness are given in percent, as HSL's are. Computed, never stored (ADR-0056).
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the hue in degrees, the saturation and brightness in percent
 * @see REF-WIKI-HSL-HSV
 */
export function rgbToHsb(rgb: Rgb): Hsb {
  return {
    brightness: percentFromFraction(brightness(rgb)),
    hue: hue(rgb),
    saturation: percentFromFraction(hsbSaturation(rgb)),
  };
}
