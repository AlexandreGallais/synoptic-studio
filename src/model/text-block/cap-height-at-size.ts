import { fontUnitsAtSize } from "../font";

import type { FontMetrics } from "../font";

/**
 * The height of a text's capitals above its baseline, at its size: the font's cap height
 * (`sCapHeight`, or 0.66 em when the font gives none), where a trimmed block starts
 * (`DERIV-text-box`, step 7).
 *
 * Formula: C × s / U.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size, the em in user units
 * @returns the cap height, in user units
 * @see DERIV-text-box
 */
export function capHeightAtSize(metrics: FontMetrics, size: number): number {
  return fontUnitsAtSize(metrics.capHeight, size, metrics.unitsPerEm);
}
