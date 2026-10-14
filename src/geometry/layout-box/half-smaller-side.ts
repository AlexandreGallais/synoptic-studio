import type { Size } from "./size";

/**
 * Half the smaller side of a box: the radius that rounds it into a circle or a capsule
 * (`DERIV-round-shape` step 1, m = min(w, h), r = m/2).
 *
 * Formula: min(w, h) / 2.
 *
 * @kind geometry
 * @param size - width and height
 * @returns half the smaller of the two
 * @see DERIV-round-shape
 */
export function halfSmallerSide(size: Size): number {
  return Math.min(size.width, size.height) / 2;
}
