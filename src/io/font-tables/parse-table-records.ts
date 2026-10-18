import { isSpanWithin } from "../../math";

import { parseRecordPlaces } from "./parse-record-places";
import { TABLE_DIRECTORY_LAYOUT } from "./table-directory-layout";

import type { FontTableRecord } from "./font-table-record";

/**
 * Reads the table records of a table directory, of a font file or of one face of a collection:
 * where each record sits, and the offset and length of its table, counted from the start of the
 * file (`REF-OT-OTFF`). A table that does not fit in the file is left out; another file has no
 * table. What `parseTableDirectory` reads by tag and a collection's face rewrites
 * (`collectionFaceToParts`).
 *
 * @kind format
 * @param view - the file
 * @param directory - where the table directory starts: 0 in a font file,
 * `tableDirectoryOffsets[i]` in a collection
 * @returns each record's offset, and its table's offset and length, in the order of the records
 * @see REF-OT-OTFF
 */
export function parseTableRecords(
  view: Readonly<DataView>,
  directory: number,
): readonly (FontTableRecord & {
  /** Offset of the table's record, from the file's start. */
  readonly record: number;
})[] {
  const { lengthField, offsetField } = TABLE_DIRECTORY_LAYOUT;

  return parseRecordPlaces(view, directory)
    .map((record) => ({
      length: view.getUint32(record + lengthField),
      offset: view.getUint32(record + offsetField),
      record,
    }))
    .filter((table) => isSpanWithin(table.offset, table.length, view.byteLength));
}
