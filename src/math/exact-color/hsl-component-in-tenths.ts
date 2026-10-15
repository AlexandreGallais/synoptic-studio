import { clamp, modulo } from "../arithmetic";

import type { IntegerRatio } from "./integer-ratio";
import type { TenthsColor } from "./tenths-color";

/** A twelfth of a turn, 30°, in tenths of a degree: the step of HSL's `k`. */
const TWELFTH = 300;

/** A whole turn, in tenths of a degree. */
const TURN = 3600;

/** Where the component starts to fall from its highest, `k − 3`, in tenths of a degree: 90°. */
const RISE_END = 900;

/** Where the component is back at its highest, `9 − k`, in tenths of a degree: 270°. */
const FALL_START = 2700;

/** A whole lightness, 100 %, in tenths of a percent. */
const WHOLE_LEVEL = 1000;

/** The denominator: 1000 for L, times 1000 for S, times 300 for `k`. */
const DENOMINATOR = 300_000_000;

/** The lightness's factor over that denominator: 1000 · 300. */
const LEVEL_FACTOR = 300_000;

/**
 * One component of an HSL color given to one decimal, as an exact fraction of integers: CSS's
 * `f(n)` with each value in tenths, so that a byte that lies on a half is seen exactly
 * (`DERIV-color-spaces`, step 11; step 6 for `f(n)`). HSL 0°, 75 %, 40 % has the green and blue
 * (120 000 000 − 750 · 400 · 300) / 300 000 000 = 0.1.
 *
 * Formula: K = (300 n + h) mod 3600, q = clamp(min(K − 900, 2700 − K), −300, 300),
 * f(n) = (300 000 · l − s · min(l, 1000 − l) · q) / (3 · 10⁸).
 *
 * @kind math
 * @param color - the hue, saturation and lightness in tenths, integers
 * @param n - 0 for red, 8 for green, 4 for blue
 * @returns the component as a numerator over 3 · 10⁸
 * @see DERIV-color-spaces
 */
export function hslComponentInTenths(color: TenthsColor, n: number): IntegerRatio {
  const k = modulo(TWELFTH * n + color.hue, TURN);
  const q = clamp(Math.min(k - RISE_END, FALL_START - k), -TWELFTH, TWELFTH);
  // a = S · min(L, 1 − L), in tenths: s · min(l, 1000 − l).
  const a = color.saturation * Math.min(color.level, WHOLE_LEVEL - color.level);

  return { denominator: DENOMINATOR, numerator: LEVEL_FACTOR * color.level - a * q };
}
