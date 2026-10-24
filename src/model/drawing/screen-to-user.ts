/**
 * A length in screen pixels, in user units of the drawing: divided by the view's scale, so that a
 * reach or an outline given in screen pixels does not change with the zoom (`interaction.md` §2,
 * as snapping's tolerance, §4).
 *
 * Formula: pixels / scale.
 *
 * @kind domain
 * @param pixels - the length in screen pixels
 * @param scale - the view's scale, screen pixels per user unit, > 0
 * @returns the length in user units
 * @see docs/domain/interaction.md#_2-selector
 */
export function screenToUser(pixels: number, scale: number): number {
  return pixels / scale;
}
