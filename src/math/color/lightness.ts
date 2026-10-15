import { average } from "../arithmetic";

import { brightness } from "./brightness";

import type { Rgb } from "./rgb";

/**
 * The lightness of a color in HSL, the average of its largest and smallest components
 * (REF-CSS-COLOR-4, §7.2: `light = (min + max) / 2`; REF-WIKI-HSL-HSV, `L = (M + m) / 2`;
 * `DERIV-color-spaces`, step 4).
 *
 * Formula: L = (max(r, g, b) + min(r, g, b)) / 2.
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the lightness, from 0 (black) to 1 (white)
 * @see REF-CSS-COLOR-4
 */
export function lightness(rgb: Rgb): number {
  return average(brightness(rgb), Math.min(rgb.red, rgb.green, rgb.blue));
}
