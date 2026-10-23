import { boundedScale } from "./bounded-scale";

/** A key's step of the zoom, √2: 100, 141, 200, 283, 400 % (SP-030's answer 4). */
const ZOOM_STEP = Math.SQRT2;

/**
 * A scale stepped up or down by a key's step, × √2 or ÷ √2, Inkscape's, within the bounds
 * (SP-030's answer 4).
 *
 * Formula: s · √2 or s / √2, then min(max(·, 0.01), 256).
 *
 * @kind domain
 * @param scale - CSS pixels per user unit before the step
 * @param direction - 1 to zoom in, −1 to zoom out
 * @returns the scale stepped, within the bounds
 * @see DERIV-view-zoom
 */
export function steppedScale(scale: number, direction: -1 | 1): number {
  return boundedScale(direction > 0 ? scale * ZOOM_STEP : scale / ZOOM_STEP);
}
