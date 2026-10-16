import { clamp, modulo } from "../arithmetic";

import type { ExactColor } from "./exact-color";
import type { IntegerRatio } from "./integer-ratio";

/** The degrees of one step of HSB's `k`, a sixth of a turn. */
const SIXTH = 60;

/** The steps of `k` in a turn. */
const STEPS_PER_TURN = 6;

/** Where the component is back at its highest, `4 − k`, in steps of `k`. */
const FALL_END = 4;

/**
 * HSB's ramp `clamp(min(k, 4 − k), 0, 1)` as an exact fraction, `k` counted over `60 ·` the hue's
 * denominator (REF-WIKI-HSL-HSV, HSV to RGB alternative; `DERIV-color-spaces`, steps 6 and 11).
 *
 * Formula: t = 60 d, K = (t n + h) mod 6t, p = clamp(min(K, 4t − K), 0, t), ramp = p / t.
 *
 * @kind math
 * @param hue - the hue in degrees, as a fraction `h / d`
 * @param n - 5 for red, 3 for green, 1 for blue
 * @returns the ramp, from 0 to 1, over `60 d`
 * @see REF-WIKI-HSL-HSV
 */
function exactHsbRamp(hue: IntegerRatio, n: number): IntegerRatio {
  const step = SIXTH * hue.denominator;
  const k = modulo(step * n + hue.numerator, STEPS_PER_TURN * step);

  return { denominator: step, numerator: clamp(Math.min(k, FALL_END * step - k), 0, step) };
}

/**
 * One component of an HSB color given as exact fractions, as an exact fraction of integers: HSB's
 * `f(n)` with `k` counted over `60 ·` the hue's denominator and every fraction over one common
 * denominator, so that a byte that lies on a half is seen exactly (`DERIV-color-spaces`, steps 11
 * and 12; step 6 for `f(n)`). HSB 30.1°, 90 %, 100 % as 301 / 10, 900 / 1000, 1000 / 1000 has the
 * blue 60 000 000 / 600 000 000 = 0.1; `#336699`'s exact 210°, 102 / 153 with the brightness
 * 61 / 100 has the red 61 · 51 / 100 · 153 = 0.20333.
 *
 * Formula: with H = h / d, S = s / e, V = v / w and t = 60 d: K = (t n + h) mod 6t,
 * p = clamp(min(K, 4t − K), 0, t), f(n) = v · (e t − s p) / (w e t), from V · (1 − S · p / t) (`exactHsbRamp`).
 *
 * @kind math
 * @param color - the hue in degrees, the saturation and brightness from 0 to 1, as fractions
 * @param n - 5 for red, 3 for green, 1 for blue
 * @returns the component, from 0 to 1, as a fraction of integers
 * @see DERIV-color-spaces
 */
export function exactHsbComponent(color: ExactColor, n: number): IntegerRatio {
  const { hue, level, saturation } = color;
  const ramp = exactHsbRamp(hue, n);

  return {
    denominator: level.denominator * saturation.denominator * ramp.denominator,
    numerator:
      level.numerator *
      (saturation.denominator * ramp.denominator - saturation.numerator * ramp.numerator),
  };
}
