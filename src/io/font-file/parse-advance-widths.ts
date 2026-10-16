import { elementAddress, integerQuotient } from "../../math";

/** Size of one `hmtx` record: an advance width, then a left side bearing, two bytes each. */
const METRIC_RECORD_SIZE = 4;

/**
 * Reads the advance width of each `hmtx` record, by glyph index; a record past the table's end is
 * left out.
 *
 * @kind format
 * @param hmtx - the font's `hmtx` table
 * @param numberOfHMetrics - number of records, from `hhea`
 * @returns the advance of each record, font units
 * @see REF-OT-HMTX
 */
export function parseAdvanceWidths(hmtx: DataView, numberOfHMetrics: number): readonly number[] {
  return Array.from(
    { length: Math.min(numberOfHMetrics, integerQuotient(hmtx.byteLength, METRIC_RECORD_SIZE)) },
    (unused, glyph) => hmtx.getUint16(elementAddress(0, METRIC_RECORD_SIZE, glyph)),
  );
}
