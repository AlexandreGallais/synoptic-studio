import type { FontTableRecord } from "./font-table-record";

/**
 * A view of one table of a font file: its offsets counted from the table's start, which every
 * offset of the table is (`REF-OT-OTFF`), and its length the table's, so that a reader checks what
 * it reads against the table and never reads past it.
 *
 * @kind format
 * @param view - the font file
 * @param table - where the table sits, within the file
 * @returns a view of the table alone
 * @see REF-OT-OTFF
 */
export function tableToView(view: DataView, table: FontTableRecord): DataView {
  return new DataView(view.buffer, view.byteOffset + table.offset, table.length);
}
