import { fontUnitsAtSize } from "../font";

import type { FontMetrics } from "../font";

/**
 * The height of a line's content area, sized "to fit … text from its first available font", here
 * from "the maximum ascender and descender of the font" (`REF-CSS-INLINE-3`, §6.4): the ascent
 * `A` plus the depth `D` of CSS's leading (§5.3), at the text's size.
 *
 * Formula: (A − D) × s / U, the descender D negative below the baseline.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size, the em in user units
 * @returns the content area's height, in user units
 * @see REF-CSS-INLINE-3
 */
export function contentAreaHeight(metrics: FontMetrics, size: number): number {
  return fontUnitsAtSize(metrics.ascender - metrics.descender, size, metrics.unitsPerEm);
}
