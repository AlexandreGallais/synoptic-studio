import { elementAddress, integerQuotient, isSpanWithin } from "../../math";

/** Offset of `numTables` in the `cmap` header, after its version. */
const NUM_TABLES_OFFSET = 2;

/** Size of the `cmap` header: version and `numTables`. */
const CMAP_HEADER_SIZE = 4;

/** Size of one encoding record: platform, encoding, then the subtable's offset. */
const ENCODING_RECORD_SIZE = 8;

/** Offset of the subtable's offset in an encoding record. */
const SUBTABLE_OFFSET_FIELD = 4;

/** Size of a subtable's format field, read to choose a subtable. */
const FORMAT_SIZE = 2;

/** Platform 0, Unicode: every encoding maps Unicode code points. */
const UNICODE_PLATFORM = 0;

/** Platform 3, Windows. */
const WINDOWS_PLATFORM = 3;

/** Windows encoding 1: Unicode, the Basic Multilingual Plane. */
const WINDOWS_BMP = 1;

/** Windows encoding 10: Unicode, the full repertoire. */
const WINDOWS_FULL_REPERTOIRE = 10;

/** The Windows encodings of Unicode. */
const WINDOWS_UNICODE_ENCODINGS: ReadonlySet<number> = new Set([
  WINDOWS_BMP,
  WINDOWS_FULL_REPERTOIRE,
]);

/**
 * Reads where the `cmap` subtables that map Unicode code points sit: those of the Unicode
 * platform, and the Windows ones of encodings 1 and 10. A record past the table's end, or a
 * subtable whose format does not fit in the table, is left out.
 *
 * @kind format
 * @param cmap - the font's `cmap` table, at least its 4-byte header
 * @returns each subtable's offset from the start of the table, in the order of the records
 * @see REF-OT-CMAP
 */
export function parseUnicodeSubtables(cmap: DataView): readonly number[] {
  const count = Math.min(
    cmap.getUint16(NUM_TABLES_OFFSET),
    integerQuotient(cmap.byteLength - CMAP_HEADER_SIZE, ENCODING_RECORD_SIZE),
  );

  return Array.from({ length: count }, (unused, index) =>
    elementAddress(CMAP_HEADER_SIZE, ENCODING_RECORD_SIZE, index),
  )
    .filter(
      (record) =>
        cmap.getUint16(record) === UNICODE_PLATFORM ||
        (cmap.getUint16(record) === WINDOWS_PLATFORM &&
          WINDOWS_UNICODE_ENCODINGS.has(cmap.getUint16(record + 2))),
    )
    .map((record) => cmap.getUint32(record + SUBTABLE_OFFSET_FIELD))
    .filter((offset) => isSpanWithin(offset, FORMAT_SIZE, cmap.byteLength));
}
