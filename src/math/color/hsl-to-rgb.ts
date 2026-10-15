import { hslComponent } from "./hsl-component";

import type { Rgb } from "./rgb";
import type { Hsl } from "./rgb-to-hsl";

/** The `n` of the red component in HSL's `f(n)`. */
const RED = 0;

/** The `n` of the green component. */
const GREEN = 8;

/** The `n` of the blue component. */
const BLUE = 4;

/**
 * A color's red, green and blue from its HSL, unrounded: `(f(0), f(8), f(4))` (REF-CSS-COLOR-4,
 * §7.1, `hslToRgb`; `DERIV-color-spaces`, step 6). HSL(30°, 100 %, 50 %) is 1, 0.5, 0.
 *
 * @kind math
 * @param hsl - the color, its saturation and lightness in percent, from 0 to 100
 * @returns its components, from 0 to 1
 * @see REF-CSS-COLOR-4
 */
export function hslToRgb(hsl: Hsl): Rgb {
  return {
    blue: hslComponent(hsl, BLUE),
    green: hslComponent(hsl, GREEN),
    red: hslComponent(hsl, RED),
  };
}
