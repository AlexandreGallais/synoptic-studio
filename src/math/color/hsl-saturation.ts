import { brightness } from "./brightness";
import { largestHalfChroma } from "./largest-half-chroma";
import { lightness } from "./lightness";

import type { Rgb } from "./rgb";

/**
 * The saturation of a color in HSL: how far its largest component lies above its lightness, out
 * of the most the lightness allows; 0 for black and white, where that most is 0 (REF-CSS-COLOR-4,
 * §7.2: `sat = (max − light) / min(light, 1 − light)`, 0 when `light` is 0 or 1; REF-WIKI-HSL-HSV,
 * `S_L = (V − L) / min(L, 1 − L)`;
 * `DERIV-color-spaces`, step 4). Never negative: `max − light` is half the chroma.
 *
 * Formula: S_L = (M − L) / min(L, 1 − L), 0 when min(L, 1 − L) = 0.
 *
 * @kind math
 * @param rgb - the color's components, from 0 to 1
 * @returns the saturation, from 0 to 1
 * @see REF-CSS-COLOR-4
 */
export function hslSaturation(rgb: Rgb): number {
  const lightnessOfColor = lightness(rgb);
  const mostAboveLightness = largestHalfChroma(lightnessOfColor);

  return mostAboveLightness === 0 ? 0 : (brightness(rgb) - lightnessOfColor) / mostAboveLightness;
}
