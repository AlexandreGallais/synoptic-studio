import { parseTableDirectory } from "./parse-table-directory";

import type { FontTableRecord } from "./font-table-record";
import type { FontTables } from "./font-tables";

/** A font's table directory: where each table sits, by tag. */
type Directory = Readonly<Record<string, FontTableRecord>>;

/**
 * The smallest size of each table read at fixed offsets: up to `unitsPerEm` (18) in `head`,
 * `numberOfHMetrics` (34) in `hhea`, `underlineThickness` (10) in `post`, `yStrikeoutPosition`
 * (28) in `OS/2`, each field two bytes, and the headers of `cmap`, `kern` and `GPOS`. A required
 * table too short makes the font unreadable; a kerning table too short is left out.
 */
const MINIMUM_SIZES: Readonly<Record<string, number>> = {
  GPOS: 10,
  "OS/2": 30,
  cmap: 4,
  head: 20,
  hhea: 36,
  kern: 4,
  post: 12,
};

/**
 * Leaves out of a directory the tables too short for the fields read in them.
 *
 * @kind format
 * @param directory - the font's table directory
 * @returns the tables long enough, by tag
 * @see REF-OT-OTFF
 */
function directoryToReadableTables(directory: Directory): Directory {
  return Object.fromEntries(
    Object.entries(directory).filter(([tag, table]) => table.length >= (MINIMUM_SIZES[tag] ?? 0)),
  );
}

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
 * Finds the tables of the decorations, permissions and names: `post` and `OS/2`, and the kerning
 * tables `GPOS` and `kern` and the naming table `name` if the font has them.
 *
 * @kind format
 * @param directory - the font's table directory
 * @returns the tables; undefined when `post` or `OS/2` is missing
 * @see REF-OT-OS2
 */
function directoryToLayoutTables(
  directory: Directory,
): Pick<FontTables, "gpos" | "kern" | "name" | "os2" | "post"> | undefined {
  const { GPOS: gpos, kern, name, "OS/2": os2, post } = directory;

  return os2 === undefined || post === undefined ? undefined : { gpos, kern, name, os2, post };
}

/**
 * Finds the tables a font file needs to be measured: a TrueType or OpenType file whose directory
 * lists `cmap`, `head`, `hhea`, `hmtx`, `post` and `OS/2` within the file, each long enough for
 * the fields read in it, and `GPOS`, `kern` and `name` when it has them. A face of a collection
 * is read in the collection itself, at its directory, its table offsets counted from the
 * collection's start (`REF-OT-OTFF`), no file of its own written.
 *
 * @kind format
 * @param view - the font file, or a collection
 * @param start - where the face's table directory starts: 0 in a font file,
 * `tableDirectoryOffsets[i]` in a collection
 * @returns where its tables sit; undefined for another file, or a table missing or too short
 * @see REF-OT-OTFF
 */
export function parseFontTables(view: Readonly<DataView>, start: number): FontTables | undefined {
  const directory = directoryToReadableTables(parseTableDirectory(view, start));
  const characters = directoryToCharacterTables(directory);
  const horizontal = directoryToHorizontalTables(directory);
  const layout = directoryToLayoutTables(directory);

  return characters === undefined || horizontal === undefined || layout === undefined
    ? undefined
    : { ...characters, ...horizontal, ...layout };
}
