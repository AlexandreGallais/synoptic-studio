import type { Size } from "./size";

/**
 * Whether a box has an area: the product of its width and height is positive, so that neither is
 * 0 (a box of size 0 cannot be stretched nor converted, F08).
 *
 * Formula: w · h > 0.
 *
 * @kind geometry
 * @param size - width and height, ≥ 0
 * @returns `true` when both are > 0
 * @see REF-WIKI-RECTANGLE
 */
export function hasArea(size: Size): boolean {
  return size.width * size.height > 0;
}
