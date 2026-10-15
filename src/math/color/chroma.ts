import { brightness } from "./brightness";

import type { Rgb } from "./rgb";

/**
 * The chroma of a color, the range of its components: its largest less its smallest, 0 for a
 * grey; its largest is HSB's brightness (REF-WIKI-HSL-HSV, Hue and chroma: `C = range(R, G, B) =
 * M − m`, From RGB: `C = V − min(R, G, B)`; `DERIV-color-spaces`,
 * step 3).
 *
 * Formula: C = max(r, g, b) − min(r, g, b).
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the chroma, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function chroma(rgb: Rgb): number {
  return brightness(rgb) - Math.min(rgb.red, rgb.green, rgb.blue);
}
