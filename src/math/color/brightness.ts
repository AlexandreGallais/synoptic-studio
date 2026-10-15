import { maximum } from "../arithmetic";

import type { Rgb } from "./rgb";

/**
 * The brightness of a color in HSB, HSV's value: its largest component (REF-WIKI-HSL-HSV, From
 * RGB: `V = max(R, G, B)`; "HSV … is also often called HSB (B for brightness)";
 * `DERIV-color-spaces`, step 5).
 *
 * Formula: V = max(r, g, b).
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the brightness, from 0 (black) to 1
 * @see REF-WIKI-HSL-HSV
 */
export function brightness(rgb: Rgb): number {
  return maximum(Object.values(rgb), 0);
}
