/** Sizes and offsets of a table directory and its records, in bytes (`REF-OT-OTFF`). */
export const TABLE_DIRECTORY_LAYOUT: {
  /** Size of the directory before its records: version, `numTables` and three search fields. */
  readonly headerSize: number;
  /** Offset of `numTables` in the directory. */
  readonly numTablesOffset: number;
  /** Size of one table record: tag, checksum, offset and length, four bytes each. */
  readonly recordSize: number;
  /** Offset of a table's offset in its record, past its tag and checksum. */
  readonly offsetField: number;
  /** Offset of a table's length in its record. */
  readonly lengthField: number;
} = { headerSize: 12, lengthField: 12, numTablesOffset: 4, offsetField: 8, recordSize: 16 };
