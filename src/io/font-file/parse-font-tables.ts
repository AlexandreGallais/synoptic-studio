import { parseTableDirectory } from "./parse-table-directory";

import type { FontTableRecord } from "./font-table-record";
import type { FontTables } from "./font-tables";

/** A font's table directory: where each table sits, by tag. */
type Directory = Readonly<Record<string, FontTableRecord>>;

/**
 * Finds the tables that map characters to glyphs and give the em: `cmap` and `head`.
 *
 * @kind format
 * @param directory - the font's table directory
 * @returns the two tables; undefined when one is missing
 * @see REF-OT-CMAP
 */
function directoryToCharacterTables(
  directory: Directory,
): Pick<FontTables, "cmap" | "head"> | undefined {
  const { cmap, head } = directory;

  return cmap === undefined || head === undefined ? undefined : { cmap, head };
}

/**
 * Finds the horizontal tables: `hhea` and `hmtx`.
 *
 * @kind format
 * @param directory - the font's table directory
 * @returns the two tables; undefined when one is missing
 * @see REF-OT-HHEA
 */
function directoryToHorizontalTables(
  directory: Directory,
): Pick<FontTables, "hhea" | "hmtx"> | undefined {
  const { hhea, hmtx } = directory;

  return hhea === undefined || hmtx === undefined ? undefined : { hhea, hmtx };
}

/**
 * Finds the tables of the decorations and permissions: `post` and `OS/2`, and the kerning tables
 * `GPOS` and `kern` if the font has them.
 *
 * @kind format
 * @param directory - the font's table directory
 * @returns the tables; undefined when `post` or `OS/2` is missing
 * @see REF-OT-OS2
 */
function directoryToLayoutTables(
  directory: Directory,
): Pick<FontTables, "gpos" | "kern" | "os2" | "post"> | undefined {
  const { GPOS: gpos, kern, "OS/2": os2, post } = directory;

  return os2 === undefined || post === undefined ? undefined : { gpos, kern, os2, post };
}

/**
 * Finds the tables a font file needs to be measured: a TrueType or OpenType file whose directory
 * lists `cmap`, `head`, `hhea`, `hmtx`, `post` and `OS/2` within the file, and `GPOS` and `kern`
 * when it has them.
 *
 * @kind format
 * @param view - the font file
 * @returns where its tables sit; undefined for another file, or a table missing
 * @see REF-OT-OTFF
 */
export function parseFontTables(view: DataView): FontTables | undefined {
  const directory = parseTableDirectory(view);
  const characters = directoryToCharacterTables(directory);
  const horizontal = directoryToHorizontalTables(directory);
  const layout = directoryToLayoutTables(directory);

  return characters === undefined || horizontal === undefined || layout === undefined
    ? undefined
    : { ...characters, ...horizontal, ...layout };
}
