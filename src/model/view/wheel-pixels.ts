/** Pixels in a line of the wheel, Penpot's and Inkscape's (`REF-PENPOT-NORMALIZE-WHEEL`). */
const LINE_PIXELS = 40;

/** Pixels in a page of the wheel, Penpot's (`REF-PENPOT-NORMALIZE-WHEEL`). */
const PAGE_PIXELS = 800;

/**
 * A delta of the wheel in pixels, from its unit (`deltaMode`, `REF-MDN-WHEEL-DELTAMODE`): pixels
 * kept, a line 40 pixels, a page 800, as Penpot (`REF-PENPOT-NORMALIZE-WHEEL`).
 *
 * Formula: δ for a mode of 0, 40 · δ for 1, 800 · δ for any other.
 *
 * @kind domain
 * @param delta - the delta, in the unit of its mode
 * @param mode - `deltaMode`: 0 pixels, 1 lines, 2 pages
 * @returns the delta, in pixels
 * @see REF-PENPOT-NORMALIZE-WHEEL
 */
export function wheelPixels(delta: number, mode: number): number {
  if (mode === 0) {
    return delta;
  }

  return delta * (mode === 1 ? LINE_PIXELS : PAGE_PIXELS);
}
