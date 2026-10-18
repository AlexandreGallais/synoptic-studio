import { clamp, modulo } from "../arithmetic";

import { exactLargestHalfChroma } from "./exact-largest-half-chroma";

import type { ExactColor } from "./exact-color";
import type { IntegerRatio } from "./integer-ratio";

/** The degrees of one step of HSL's `k`, a twelfth of a turn. */
const TWELFTH = 30;

/** The steps of `k` in a turn. */
const STEPS_PER_TURN = 12;

/** Where the component starts to fall from its highest, `k − 3`, in steps of `k`. */
const RISE_END = 3;

/** Where the component is back at its highest, `9 − k`, in steps of `k`. */
const FALL_START = 9;

/**
 * HSL's ramp `clamp(min(k − 3, 9 − k), −1, 1)` as an exact fraction, `k` counted over `30 ·` the
 * hue's denominator (REF-CSS-COLOR-4, §7.1, `f(n)`; `DERIV-color-spaces`, steps 6 and 11).
 *
 * Formula: t = 30 d, K = (t n + h) mod 12t, q = clamp(min(K − 3t, 9t − K), −t, t), ramp = q / t.
 *
 * @kind math
 * @param hue - the hue in degrees, as a fraction `h / d`
 * @param n - 0 for red, 8 for green, 4 for blue
 * @returns the ramp, from −1 to 1, over `30 d`
 * @see DERIV-color-spaces
 */
function exactHslRamp(hue: IntegerRatio, n: number): IntegerRatio {
  const step = TWELFTH * hue.denominator;
  const k = modulo(step * n + hue.numerator, STEPS_PER_TURN * step);

  return {
    denominator: step,
    numerator: clamp(Math.min(k - RISE_END * step, FALL_START * step - k), -step, step),
  };
}

/**
 * The half chroma `a = S · min(L, 1 − L)` of exact values, the saturation times the largest half
 * chroma the lightness allows (`exactLargestHalfChroma`; REF-WIKI-HSL-HSV, HSL to RGB alternative;
 * `DERIV-color-spaces`, steps 6 and 11).
 *
 * Formula: with S = s / e and L = l / w: a = s · min(l, w − l) / (e w).
 *
 * @kind math
 * @param saturation - the saturation, from 0 to 1
 * @param lightness - the lightness, from 0 to 1
 * @returns the half chroma, from 0 to ½
 * @see REF-WIKI-HSL-HSV
 */
function exactHalfChroma(saturation: IntegerRatio, lightness: IntegerRatio): IntegerRatio {
  const largest = exactLargestHalfChroma(lightness);

  return {
    denominator: saturation.denominator * largest.denominator,
    numerator: saturation.numerator * largest.numerator,
  };
}

/**
 * One component of an HSL color given as exact fractions, as an exact fraction of integers: CSS's
 * `f(n)` with `k` counted over `30 ·` the hue's denominator and every fraction over one common
 * denominator, so that a byte that lies on a half is seen exactly (`DERIV-color-spaces`, steps 11
 * and 12; step 6 for `f(n)`). HSL 0°, 75 %, 40 % as 0 / 10, 750 / 1000, 400 / 1000 has the green
 * and blue (120 000 000 − 750 · 400 · 300) / 300 000 000 = 0.1.
 *
 * Formula: with H = h / d, S = s / e, L = l / w and t = 30 d: K = (t n + h) mod 12t,
 * q = clamp(min(K − 3t, 9t − K), −t, t), f(n) = (l e t − s · min(l, w − l) · q) / (w e t), from L − a · q / t
 * (`exactHslRamp`, `exactHalfChroma`).
 *
 * @kind math
 * @param color - the hue in degrees, the saturation and lightness from 0 to 1, as fractions
 * @param n - 0 for red, 8 for green, 4 for blue
 * @returns the component, from 0 to 1, as a fraction of integers
 * @see DERIV-color-spaces
 */
export function exactHslComponent(color: ExactColor, n: number): IntegerRatio {
  const ramp = exactHslRamp(color.hue, n);
  const a = exactHalfChroma(color.saturation, color.level);

  return {
    denominator: a.denominator * ramp.denominator,
    numerator:
      color.level.numerator * color.saturation.denominator * ramp.denominator -
      a.numerator * ramp.numerator,
  };
}
