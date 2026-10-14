/**
 * The advance of a glyph: its `hmtx` record, or the last record for a glyph past
 * `numberOfHMetrics`.
 *
 * @kind domain
 * @param advances - the advance of each `hmtx` record, by glyph index
 * @param glyph - glyph index, integer ≥ 0
 * @returns the advance in font units; 0 for a font with no record
 * @see REF-OT-HMTX
 */
export function glyphAdvance(advances: readonly number[], glyph: number): number {
  return advances[Math.min(glyph, advances.length - 1)] ?? 0;
}
