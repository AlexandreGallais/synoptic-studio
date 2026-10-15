import type { Rgb } from "./rgb";

/**
 * The smallest component of a color, `m` beside the largest `M` (REF-WIKI-HSL-HSV, Hue and chroma:
 * `m = min(R, G, B)`; `DERIV-color-spaces`, notation). The chroma and HSL's lightness start from
 * it.
 *
 * Formula: m = min(r, g, b).
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the smallest, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function smallestComponent(rgb: Rgb): number {
  return Math.min(rgb.red, rgb.green, rgb.blue);
}
