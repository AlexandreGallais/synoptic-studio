import type { FontTableRecord } from "./font-table-record";

/** Size of one `hmtx` record: an advance width, then a left side bearing, two bytes each. */
const METRIC_RECORD_SIZE = 4;

/**
 * Reads the advance width of each `hmtx` record, by glyph index; a record past the table's end is
 * left out.
 *
 * @kind format
 * @param view - the font file
 * @param hmtx - where its `hmtx` table sits
 * @param numberOfHMetrics - number of records, from `hhea`
 * @returns the advance of each record, font units
 * @see REF-OT-HMTX
 */
export function parseAdvanceWidths(
  view: DataView,
  hmtx: FontTableRecord,
  numberOfHMetrics: number,
): readonly number[] {
  return Array.from(
    { length: Math.min(numberOfHMetrics, Math.floor(hmtx.length / METRIC_RECORD_SIZE)) },
    (unused, glyph) => view.getUint16(hmtx.offset + METRIC_RECORD_SIZE * glyph),
  );
}
