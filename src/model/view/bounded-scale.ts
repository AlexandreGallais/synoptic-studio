import { clamp } from "../../math";

/** The smallest scale, 1 % (SP-030's answer 5). */
const MIN_SCALE = 0.01;

/** The largest scale, 25 600 % (SP-030's answer 5). */
const MAX_SCALE = 256;

/**
 * A scale within the view's bounds, 1 % to 25 600 %, Inkscape's (SP-030's answer 5): a scale
 * beyond them taken at the bound.
 *
 * Formula: min(max(s, 0.01), 256).
 *
 * @kind domain
 * @param scale - the scale asked
 * @returns the scale within the bounds
 * @see DERIV-view-zoom
 */
export function boundedScale(scale: number): number {
  return clamp(scale, MIN_SCALE, MAX_SCALE);
}
