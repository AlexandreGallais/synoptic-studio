import { characterAdvance } from "../font";

import type { FontMetrics } from "../font";

/**
 * The advance of a line without kerning nor spacing: the sum of its characters' advances, each
 * glyph starting where the one before ends (`DERIV-text-width`, step 2).
 *
 * Formula: Σ adv(g(cᵢ)).
 *
 * @kind domain
 * @param metrics - the font's metrics
 * @param codePoints - the line's characters
 * @returns the advance in font units; 0 for an empty line
 * @see DERIV-text-width
 */
export function lineAdvance(metrics: FontMetrics, codePoints: readonly number[]): number {
  return codePoints.reduce((sum, codePoint) => sum + characterAdvance(metrics, codePoint), 0);
}
