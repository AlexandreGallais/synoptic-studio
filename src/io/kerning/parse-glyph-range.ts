/**
 * Finds the range record holding a glyph among the range records of a Coverage or ClassDef table
 * of format 2: its start glyph at or before the glyph, its end glyph at or after it.
 *
 * @kind format
 * @param table - the layout table holding the records, each within it
 * @param ranges - where each range record starts in the table: start glyph, then end glyph
 * @param glyph - the glyph to look for
 * @returns where the range holding it starts; undefined when no range holds it
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseGlyphRange(
  table: DataView,
  ranges: readonly number[],
  glyph: number,
): number | undefined {
  return ranges.find(
    (range) => table.getUint16(range) <= glyph && glyph <= table.getUint16(range + 2),
  );
}
