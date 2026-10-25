/**
 * A length in screen pixels, in user units of the drawing: divided by the view's scale, so that a
 * reach or an outline given in screen pixels does not change with the zoom
 * (`RULE-interaction-the-outline-screen-pixels`, as snapping's tolerance,
 * `RULE-interaction-4-snapping-tolerance-in-screen-pixels`).
 *
 * Formula: pixels / scale.
 *
 * @kind domain
 * @param pixels - the length in screen pixels
 * @param scale - the view's scale, screen pixels per user unit, > 0
 * @returns the length in user units
 * @see RULE-interaction-2-selector
 */
export function screenToUser(pixels: number, scale: number): number {
  return pixels / scale;
}
