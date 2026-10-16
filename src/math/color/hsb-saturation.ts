import { brightness } from "./brightness";
import { chroma } from "./chroma";

import type { Rgb } from "./rgb";

/**
 * The saturation of a color in HSB: its chroma out of its brightness; 0 for black, whose
 * brightness is 0 (REF-WIKI-HSL-HSV, From RGB: `S_V = C / V`, 0 when `V = 0`;
 * `DERIV-color-spaces`, step 5).
 *
 * Formula: S_V = C / V, 0 when V = 0.
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the saturation, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function hsbSaturation(rgb: Rgb): number {
  const brightnessOfColor = brightness(rgb);

  return brightnessOfColor === 0 ? 0 : chroma(rgb) / brightnessOfColor;
}
