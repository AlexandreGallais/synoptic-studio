import type { FontTableRecord } from "./font-table-record";

/** Offset of `numTables` in the `cmap` header, after its version. */
const NUM_TABLES_OFFSET = 2;

/** Size of the `cmap` header: version and `numTables`. */
const CMAP_HEADER_SIZE = 4;

/** Size of one encoding record: platform, encoding, then the subtable's offset. */
const ENCODING_RECORD_SIZE = 8;

/** Offset of the subtable's offset in an encoding record. */
const SUBTABLE_OFFSET_FIELD = 4;

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
 * platform, and the Windows ones of encodings 1 and 10. A subtable outside the table is left out.
 *
 * @kind format
 * @param view - the font file
 * @param cmap - where its `cmap` table sits
 * @returns each subtable's offset from the start of the file, in the order of the records
 * @see REF-OT-CMAP
 */
export function parseUnicodeSubtables(view: DataView, cmap: FontTableRecord): readonly number[] {
  return Array.from(
    { length: view.getUint16(cmap.offset + NUM_TABLES_OFFSET) },
    (unused, index) => cmap.offset + CMAP_HEADER_SIZE + ENCODING_RECORD_SIZE * index,
  )
    .filter(
      (record) =>
        view.getUint16(record) === UNICODE_PLATFORM ||
        (view.getUint16(record) === WINDOWS_PLATFORM &&
          WINDOWS_UNICODE_ENCODINGS.has(view.getUint16(record + 2))),
    )
    .map((record) => view.getUint32(record + SUBTABLE_OFFSET_FIELD))
    .filter((offset) => offset < cmap.length)
    .map((offset) => cmap.offset + offset);
}
