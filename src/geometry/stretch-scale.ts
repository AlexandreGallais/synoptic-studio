import { bisection } from "../math";

import { boxAspect } from "./box-aspect";
import { isSoundStretch } from "./is-sound-stretch";
import { STRETCH_LOG_BOUND } from "./stretch-log-bound";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

import type { Point } from "../math";

/**
 * Vertical scale `λ` of a stretched polygon for a box ratio `h / w` (rule C,
 * `DERIV-stretched-polygon` step 5): the sound scales `[λ_lo, λ_hi]` found by bisection on
 * `ln λ`, then the scale whose aspect is the ratio, `λ_lo` or `λ_hi` when the ratio is beyond
 * their aspects. A ratio of `NaN` (a box of 0 × 0) gives `λ_lo`, as every comparison with it is
 * false. The bounds are found within a rounding error: a clamped polygon's guard corner may turn
 * `π/n` less about 1e-16.
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer from 3 to 12
 * @param ratio - height of the box over its width, `≥ 0`, `+∞` or `NaN`
 * @returns `λ > 0`
 * @see DERIV-stretched-polygon
 */
export function stretchScale(corners: number, ratio: number): number {
  /**
   * Unit stretched polygon for a logarithm of the scale.
   *
   * @param log - `ln λ`
   * @returns its vertices
   */
  function polygon(log: number): readonly Point[] {
    return unitStretchedPolygon(corners, Math.exp(log));
  }

  const low = bisection((log) => !isSoundStretch(polygon(log)), -STRETCH_LOG_BOUND, 0);
  const high = bisection((log) => isSoundStretch(polygon(log)), 0, STRETCH_LOG_BOUND);

  // A ratio below the lowest sound aspect is false everywhere and ends at `low`, one above the
  // highest true everywhere and ends at `high`: the clamp of rule C.
  return Math.exp(bisection((log) => boxAspect(polygon(log)) < ratio, low, high));
}
