import type { FontMetrics } from "./font-metrics";

/**
 * The advance of a character: the advance of its glyph, or that of glyph 0, `.notdef`, when the
 * font maps no glyph to it.
 *
 * @kind domain
 * @param metrics - the font's metrics
 * @param codePoint - the character's Unicode code point
 * @returns the advance in font units
 * @see REF-OT-CMAP
 */
export function characterAdvance(metrics: FontMetrics, codePoint: number): number {
  return metrics.advances[codePoint] ?? metrics.missingAdvance;
}
