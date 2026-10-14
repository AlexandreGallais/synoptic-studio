/** Offset of `ascender` in the `hhea` table. */
const ASCENDER_OFFSET = 4;

/** Offset of `descender` in the `hhea` table. */
const DESCENDER_OFFSET = 6;

/** Offset of `lineGap` in the `hhea` table. */
const LINE_GAP_OFFSET = 8;

/** Offset of `numberOfHMetrics` in the `hhea` table. */
const NUMBER_OF_H_METRICS_OFFSET = 34;

/** What the `hhea` table gives: the vertical metrics and the number of `hmtx` records. */
export type HorizontalHeader = {
  /** Typographic ascent, font units. */
  readonly ascender: number;
  /** Typographic descent, font units, negative below the baseline. */
  readonly descender: number;
  /** Typographic line gap, font units. */
  readonly lineGap: number;
  /** Number of records of the `hmtx` table. */
  readonly numberOfHMetrics: number;
};

/**
 * Reads the horizontal header of a font: its typographic ascent, descent and line gap, and the
 * number of advance records.
 *
 * @kind format
 * @param hhea - the font's `hhea` table, at least 36 bytes long
 * @returns the header's fields
 * @see REF-OT-HHEA
 */
export function parseHorizontalHeader(hhea: DataView): HorizontalHeader {
  return {
    ascender: hhea.getInt16(ASCENDER_OFFSET),
    descender: hhea.getInt16(DESCENDER_OFFSET),
    lineGap: hhea.getInt16(LINE_GAP_OFFSET),
    numberOfHMetrics: hhea.getUint16(NUMBER_OF_H_METRICS_OFFSET),
  };
}
