import type { FontTableRecord } from "./font-table-record";

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
 * @param view - the font file
 * @param hhea - where its `hhea` table sits
 * @returns the header's fields
 * @see REF-OT-HHEA
 */
export function parseHorizontalHeader(view: DataView, hhea: FontTableRecord): HorizontalHeader {
  return {
    ascender: view.getInt16(hhea.offset + ASCENDER_OFFSET),
    descender: view.getInt16(hhea.offset + DESCENDER_OFFSET),
    lineGap: view.getInt16(hhea.offset + LINE_GAP_OFFSET),
    numberOfHMetrics: view.getUint16(hhea.offset + NUMBER_OF_H_METRICS_OFFSET),
  };
}
