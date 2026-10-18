import { clamp, fractionFromPercent, modulo } from "../arithmetic";

import type { Hsb } from "./rgb-to-hsb";

/** The degrees of one of the six steps `k` counts around the hue: 60°. */
const DEGREES_PER_STEP = 60;

/** The steps of a whole turn of hue. */
const STEPS_PER_TURN = 6;

/** Where the component is back at its highest: `4 − k`. */
const FALL_END = 4;

/**
 * One component of an HSB color, as a fraction: `f(n)`, red at `n = 5`, green at 3, blue at 1
 * (REF-WIKI-HSL-HSV, HSV to RGB alternative; `DERIV-color-spaces`, step 6). Step 7 shows it gives
 * back the components HSB was read from.
 *
 * Formula: k = (n + H / 60°) mod 6, f(n) = V − V · S · max(0, min(k, 4 − k, 1)).
 *
 * @kind math
 * @param hsb - the color, its saturation and brightness in percent, from 0 to 100
 * @param n - 5 for red, 3 for green, 1 for blue
 * @returns the component, from 0 to 1
 * @see REF-WIKI-HSL-HSV
 */
export function hsbComponent(hsb: Hsb, n: number): number {
  const k = modulo(n + hsb.hue / DEGREES_PER_STEP, STEPS_PER_TURN);
  const value = fractionFromPercent(hsb.brightness);

  return (
    value - value * fractionFromPercent(hsb.saturation) * clamp(Math.min(k, FALL_END - k), 0, 1)
  );
}
