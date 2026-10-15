/** Where a table sits in a font file: its record in the table directory (`REF-OT-OTFF`). */
export type FontTableRecord = {
  /** Offset of the table from the start of the file, in bytes. */
  readonly offset: number;
  /** Length of the table, in bytes. */
  readonly length: number;
};
