/** The wheel's zoom for each pixel of its deltas, Penpot's (SP-030's answer 4). */
const WHEEL_ZOOM_PER_PIXEL = 0.0057;

/**
 * The factor the wheel or a pinch multiplies the scale by, from its deltas' sum in pixels, as
 * Penpot (SP-030's answer 4): a delta toward the screen, negative, zooms in.
 *
 * Formula: 1 + 0.0057 · |δ| for δ ≤ 0, 1 / (1 + 0.0057 · δ) for δ > 0.
 *
 * @kind domain
 * @param delta - the deltas' sum, in pixels
 * @returns the factor, > 0
 * @see DERIV-view-zoom
 */
export function wheelZoomFactor(delta: number): number {
  return delta * WHEEL_ZOOM_PER_PIXEL;
}
