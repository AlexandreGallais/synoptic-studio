import { hsbComponent } from "./hsb-component";

import type { Rgb } from "./rgb";
import type { Hsb } from "./rgb-to-hsb";

/** The `n` of the red component in HSB's `f(n)`. */
const RED = 5;

/** The `n` of the green component. */
const GREEN = 3;

/** The `n` of the blue component. */
const BLUE = 1;

/**
 * A color's red, green and blue from its HSB, unrounded: `(f(5), f(3), f(1))` (REF-WIKI-HSL-HSV,
 * HSV to RGB alternative; `DERIV-color-spaces`, step 6). HSB(210°, 67 %, 60 %) is 0.198, 0.399,
 * 0.6.
 *
 * @kind math
 * @param hsb - the color, its saturation and brightness in percent, from 0 to 100
 * @returns its components, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function hsbToRgb(hsb: Hsb): Rgb {
  return {
    blue: hsbComponent(hsb, BLUE),
    green: hsbComponent(hsb, GREEN),
    red: hsbComponent(hsb, RED),
  };
}
