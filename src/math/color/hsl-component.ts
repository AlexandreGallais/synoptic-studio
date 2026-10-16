import { clamp, fractionFromPercent, modulo } from "../arithmetic";

import { largestHalfChroma } from "./largest-half-chroma";

import type { Hsl } from "./rgb-to-hsl";

/** The degrees of one of the twelve steps `k` counts around the hue: 30°. */
const DEGREES_PER_STEP = 30;

/** The steps of a whole turn of hue. */
const STEPS_PER_TURN = 12;

/** Where the component starts to fall from its highest: `k − 3`. */
const RISE_END = 3;

/** Where the component is back at its highest: `9 − k`. */
const FALL_START = 9;

/**
 * One component of an HSL color, as a fraction: CSS's `f(n)`, red at `n = 0`, green at 8, blue at
 * 4 (REF-CSS-COLOR-4, §7.1, `hslToRgb`, the saturation and lightness divided by 100;
 * `DERIV-color-spaces`, step 6). Step 7 shows it gives back the components HSL was read from.
 *
 * Formula: k = (n + H / 30°) mod 12, a = S · min(L, 1 − L), f(n) = L − a · max(−1, min(k − 3,
 * 9 − k, 1)).
 *
 * @kind math
 * @param hsl - the color, its saturation and lightness in percent, from 0 to 100
 * @param n - 0 for red, 8 for green, 4 for blue
 * @returns the component, from 0 to 1
 * @see REF-CSS-COLOR-4
 */
export function hslComponent(hsl: Hsl, n: number): number {
  const k = modulo(n + hsl.hue / DEGREES_PER_STEP, STEPS_PER_TURN);
  const light = fractionFromPercent(hsl.lightness);
  const a = fractionFromPercent(hsl.saturation) * largestHalfChroma(light);

  return light - a * clamp(Math.min(k - RISE_END, FALL_START - k), -1, 1);
}
