import { fontUnitsAtSize } from "../font";

import type { FontMetrics } from "../font";

/**
 * The ascent `A` of CSS's leading at a text's size: how far the font's ascender rises above the
 * baseline (`REF-CSS-INLINE-3`, §5.3, from "the maximum ascender … of the font", §6.4).
 *
 * Formula: A × s / U.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size, the em in user units
 * @returns the ascent, in user units
 * @see REF-CSS-INLINE-3
 */
export function ascentAtSize(metrics: FontMetrics, size: number): number {
  return fontUnitsAtSize(metrics.ascender, size, metrics.unitsPerEm);
}
