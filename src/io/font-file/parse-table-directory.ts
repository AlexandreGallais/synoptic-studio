import { parseTag } from "./parse-tag";

import type { FontTableRecord } from "./font-table-record";

/** Size of the table directory before its records: version, `numTables` and three search fields. */
const DIRECTORY_HEADER_SIZE = 12;

/** Offset of `numTables` in the table directory. */
const NUM_TABLES_OFFSET = 4;

/** Size of one table record: tag, checksum, offset and length, four bytes each. */
const TABLE_RECORD_SIZE = 16;

/** Offset of a table's offset in its record. */
const RECORD_OFFSET_FIELD = 8;

/** Offset of a table's length in its record. */
const RECORD_LENGTH_FIELD = 12;

/** `sfntVersion` of a font with TrueType outlines. */
const TRUE_TYPE_OUTLINES = 0x00_01_00_00;

/** `sfntVersion` of a font with CFF outlines: the tag `OTTO`. */
const CFF_OUTLINES = 0x4f_54_54_4f;

/** The `sfntVersion` values of a TrueType or OpenType font file. */
const SFNT_VERSIONS: ReadonlySet<number> = new Set([TRUE_TYPE_OUTLINES, CFF_OUTLINES]);

/**
 * Where the table records of a font file sit: none for a file that is not a TrueType or OpenType
 * font, or whose directory does not fit in it.
 *
 * @kind format
 * @param view - the file
 * @returns the offset of each table record
 * @see REF-OT-OTFF
 */
function parseRecordPlaces(view: DataView): readonly number[] {
  const isFont = view.byteLength >= DIRECTORY_HEADER_SIZE && SFNT_VERSIONS.has(view.getUint32(0));
  const count = isFont ? view.getUint16(NUM_TABLES_OFFSET) : 0;

  return DIRECTORY_HEADER_SIZE + TABLE_RECORD_SIZE * count > view.byteLength
    ? []
    : Array.from(
        { length: count },
        (unused, index) => DIRECTORY_HEADER_SIZE + TABLE_RECORD_SIZE * index,
      );
}

/**
 * Reads the table directory of a TrueType or OpenType font file: where each table sits, by tag.
 * A table that does not fit in the file is left out; another file has no table.
 *
 * @kind format
 * @param view - the file
 * @returns each table's offset and length, by tag
 * @see REF-OT-OTFF
 */
export function parseTableDirectory(view: DataView): Readonly<Record<string, FontTableRecord>> {
  return Object.fromEntries(
    parseRecordPlaces(view)
      .map((record): [string, FontTableRecord] => [
        parseTag(view, record),
        {
          length: view.getUint32(record + RECORD_LENGTH_FIELD),
          offset: view.getUint32(record + RECORD_OFFSET_FIELD),
        },
      ])
      .filter(([, table]) => table.offset + table.length <= view.byteLength),
  );
}
