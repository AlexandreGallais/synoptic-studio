/** Coverage format 1: a list of glyphs. */
const GLYPH_LIST = 1;

/** Offset of the first glyph, or range, after the format and the count. */
const ITEMS_OFFSET = 4;

/** Size of one range record: start glyph, end glyph, start coverage index. */
const RANGE_RECORD_SIZE = 6;

/**
 * Reads a Coverage table: the glyphs it covers, in the order of their coverage indexes. Format 1
 * lists them; format 2 gives ranges of consecutive glyphs, in coverage order.
 *
 * @kind format
 * @param view - the font file
 * @param coverage - where the table starts in the file
 * @returns each covered glyph, at its coverage index
 * @see REF-OT-LAYOUT-COMMON
 */
export function parseCoverage(view: DataView, coverage: number): readonly number[] {
  const count = view.getUint16(coverage + 2);

  return view.getUint16(coverage) === GLYPH_LIST
    ? Array.from({ length: count }, (unused, index) =>
        view.getUint16(coverage + ITEMS_OFFSET + 2 * index),
      )
    : Array.from({ length: count }, (unused, index) => {
        const range = coverage + ITEMS_OFFSET + RANGE_RECORD_SIZE * index;
        const start = view.getUint16(range);

        return Array.from(
          { length: view.getUint16(range + 2) - start + 1 },
          (unusedGlyph, offset) => start + offset,
        );
      }).flat();
}
