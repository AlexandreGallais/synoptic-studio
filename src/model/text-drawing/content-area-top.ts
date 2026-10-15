import { ascentAtSize } from "../text-block";

import type { FontMetrics } from "../font";

/**
 * The top of a line's content area: an ascent `A` above its baseline (`REF-CSS-INLINE-3`, §5.3),
 * the area sized from "the maximum ascender and descender of the font" (§6.4), the SVG's
 * ordinates growing downwards.
 *
 * Formula: baseline − A × s / U.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size, the em in user units
 * @param baseline - the line's baseline ordinate
 * @returns the ordinate of the content area's top
 * @see REF-CSS-INLINE-3
 */
export function contentAreaTop(metrics: FontMetrics, size: number, baseline: number): number {
  return baseline - ascentAtSize(metrics, size);
}
