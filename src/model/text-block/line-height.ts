import { fontUnitsAtSize } from "../font";

import type { FontMetrics } from "../font";

/**
 * The height of one line of text: from its top the ascent, then down to the descent, then the
 * line gap before the next line, at the text's size (`DERIV-text-width`, step 4).
 *
 * Formula: (A − D + G) × s / U, the descent D negative below the baseline.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param size - the text's size, the em in user units
 * @returns the line height in user units
 * @see REF-OT-HHEA
 */
export function lineHeight(metrics: FontMetrics, size: number): number {
  return fontUnitsAtSize(
    metrics.ascender - metrics.descender + metrics.lineGap,
    size,
    metrics.unitsPerEm,
  );
}
