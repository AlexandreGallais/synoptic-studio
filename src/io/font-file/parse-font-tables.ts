import { parseTableDirectory } from "./parse-table-directory";

import type { FontTableRecord } from "./font-table-record";
import type { FontTables } from "./font-tables";

/** The horizontal tables: the header and the metrics. */
type HorizontalTables = Pick<FontTables, "hhea" | "hmtx">;

/**
 * Finds the horizontal tables of a font in its directory.
 *
 * @kind format
 * @param directory - where each table sits, by tag
 * @returns its `hhea` and `hmtx` tables; undefined when one is missing
 * @see REF-OT-HHEA
 */
function directoryToHorizontalTables(
  directory: Readonly<Record<string, FontTableRecord>>,
): HorizontalTables | undefined {
  const { hhea, hmtx } = directory;

  return hhea === undefined || hmtx === undefined ? undefined : { hhea, hmtx };
}

/**
 * Finds the tables a font file needs to be measured: a TrueType or OpenType file whose directory
 * lists `cmap`, `head`, `hhea` and `hmtx` within the file.
 *
 * @kind format
 * @param view - the font file
 * @returns where its tables sit; undefined for another file, or a table missing
 * @see REF-OT-OTFF
 */
export function parseFontTables(view: DataView): FontTables | undefined {
  const directory = parseTableDirectory(view);
  const horizontal = directoryToHorizontalTables(directory);
  const { cmap, head } = directory;

  return cmap === undefined || head === undefined || horizontal === undefined
    ? undefined
    : { cmap, head, ...horizontal };
}
