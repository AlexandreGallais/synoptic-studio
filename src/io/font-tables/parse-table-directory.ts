import { parseTableRecords } from "./parse-table-records";
import { parseTag } from "./parse-tag";

import type { FontTableRecord } from "./font-table-record";

/**
 * Reads a table directory of a TrueType or OpenType font file, or of one face of a collection:
 * where each table sits, by tag, its offset counted from the start of the file, as in a
 * collection (`REF-OT-OTFF`). A table that does not fit in the file is left out; another file
 * has no table (`parseTableRecords`).
 *
 * @kind format
 * @param view - the file
 * @param directory - where the table directory starts: 0 in a font file
 * @returns each table's offset and length, by tag
 * @see REF-OT-OTFF
 */
export function parseTableDirectory(
  view: Readonly<DataView>,
  directory: number,
): Readonly<Record<string, FontTableRecord>> {
  return Object.fromEntries(
    parseTableRecords(view, directory).map(({ length, offset, record }) => [
      parseTag(view, record),
      { length, offset },
    ]),
  );
}
