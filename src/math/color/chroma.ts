import { maximum, minimum } from "../arithmetic";

import type { Rgb } from "./rgb";

/**
 * The chroma of a color, the range of its components: its largest less its smallest, 0 for a
 * grey (REF-WIKI-HSL-HSV, Hue and chroma: `C = range(R, G, B) = M − m`; `DERIV-color-spaces`,
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
  const components = Object.values(rgb);

  return maximum(components, 0) - minimum(components, 1);
}
