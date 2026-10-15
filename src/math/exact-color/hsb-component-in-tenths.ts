import { clamp, modulo } from "../arithmetic";

import type { IntegerRatio } from "./integer-ratio";
import type { TenthsColor } from "./tenths-color";

/** A sixth of a turn, 60°, in tenths of a degree: the step of HSB's `k`. */
const SIXTH = 600;

/** A whole turn, in tenths of a degree. */
const TURN = 3600;

/** Where the component is back at its highest, `4 − k`, in tenths of a degree: 240°. */
const FALL_END = 2400;

/** The denominator: 1000 tenths of a percent for V, times 1000 for S, times 600 for `k`. */
const DENOMINATOR = 600_000_000;

/** A whole saturation times a whole step of `k`: 1000 · 600. */
const WHOLE = 600_000;

/**
 * One component of an HSB color given to one decimal, as an exact fraction of integers: HSB's
 * `f(n)` with each value in tenths, so that a byte that lies on a half is seen exactly
 * (`DERIV-color-spaces`, step 11; step 6 for `f(n)`). HSB 30.1°, 90 %, 100 % has the blue
 * 60 000 000 / 600 000 000 = 0.1.
 *
 * Formula: K = (600 n + h) mod 3600, p = clamp(min(K, 2400 − K), 0, 600),
 * f(n) = v · (600 000 − s · p) / (6 · 10⁸).
 *
 * @kind math
 * @param color - the hue, saturation and brightness in tenths, integers
 * @param n - 5 for red, 3 for green, 1 for blue
 * @returns the component as a numerator over 6 · 10⁸
 * @see DERIV-color-spaces
 */
export function hsbComponentInTenths(color: TenthsColor, n: number): IntegerRatio {
  const k = modulo(SIXTH * n + color.hue, TURN);
  const p = clamp(Math.min(k, FALL_END - k), 0, SIXTH);

  return { denominator: DENOMINATOR, numerator: color.level * (WHOLE - color.saturation * p) };
}
