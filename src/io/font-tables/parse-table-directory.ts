import { parseRecordPlaces } from "./parse-record-places";
import { parseTag } from "./parse-tag";
import { TABLE_DIRECTORY_LAYOUT } from "./table-directory-layout";

import type { FontTableRecord } from "./font-table-record";

/**
 * Reads a table directory of a TrueType or OpenType font file, or of one face of a collection:
 * where each table sits, by tag, its offset counted from the start of the file, as in a
 * collection (`REF-OT-OTFF`). A table that does not fit in the file is left out; another file
 * has no table.
 *
 * @kind format
 * @param view - the file
 * @param directory - where the table directory starts: 0 in a font file
 * @returns each table's offset and length, by tag
 * @see REF-OT-OTFF
 */
export function parseTableDirectory(
  view: DataView,
  directory: number,
): Readonly<Record<string, FontTableRecord>> {
  const { lengthField, offsetField } = TABLE_DIRECTORY_LAYOUT;

  return Object.fromEntries(
    parseRecordPlaces(view, directory)
      .map((record): [string, FontTableRecord] => [
        parseTag(view, record),
        {
          length: view.getUint32(record + lengthField),
          offset: view.getUint32(record + offsetField),
        },
      ])
      .filter(([, table]) => table.offset + table.length <= view.byteLength),
  );
}
