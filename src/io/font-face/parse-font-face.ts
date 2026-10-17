import { MEASURED_BLOCKS, blockCharacters, isEmbeddable } from "../../model";
import { fontTablesToMetrics, parseEmbeddingPermissions, parseOs2Version } from "../font-file";
import { parseFamilyName, parseFontName } from "../font-name";
import { parseFontTables, tableToView } from "../font-tables";

import { parseFontStyle } from "./parse-font-style";

import type { FontFile, FontMetrics, NamedFontFace } from "../../model";
import type { FontTables } from "../font-tables";

/** Name ID 5, the version string of the font. */
const VERSION_NAME = 5;

/** A font file, its view and its tables, found once in its table directory. */
type ReadFont = {
  /** The bytes of the file. */
  readonly file: Readonly<Uint8Array>;
  /** A view of them. */
  readonly view: Readonly<DataView>;
  /** Where its tables sit. */
  readonly tables: FontTables;
};

/**
 * Keeps a font file with its version and whether its `fsType`, read with its `OS/2` table's
 * version, permits embedding it whole (`isEmbeddable`, `DERIV-font-embedding`, step 3).
 *
 * @kind format
 * @param font - the file and its tables
 * @param name - its `name` table
 * @returns the file, its version (name ID 5, empty without one) and its permission
 * @see DERIV-font-embedding
 */
function parseFontFile(font: ReadFont, name: Readonly<DataView>): FontFile {
  const os2 = tableToView(font.view, font.tables.os2);

  return {
    bytes: font.file,
    isEmbeddable: isEmbeddable(parseEmbeddingPermissions(os2), parseOs2Version(os2)),
    version: parseFontName(name, VERSION_NAME) ?? "",
  };
}

/**
 * Reads the face of a font file whose metrics are read: its family, its weight and style, and
 * its file with its version and its permission.
 *
 * @kind format
 * @param font - the file and its tables
 * @param name - its `name` table
 * @param metrics - its metrics
 * @returns the face and its family; undefined when the file names no family or its `OS/2`
 * table is too short for its style
 * @see REF-OT-OS2
 */
function parseNamedFace(
  font: ReadFont,
  name: Readonly<DataView>,
  metrics: FontMetrics,
): NamedFontFace | undefined {
  const family = parseFamilyName(name);
  const style = parseFontStyle(tableToView(font.view, font.tables.os2));

  return family === undefined || style === undefined
    ? undefined
    : { face: { ...style, file: parseFontFile(font, name), metrics }, family };
}

/**
 * Reads the face of a font file whose tables are found: none without a `name` table or
 * without metrics.
 *
 * @kind format
 * @param font - the file and its tables
 * @returns the face and its family; undefined when it has no `name` table, gives no metrics,
 * names no family or has an `OS/2` table too short for its style
 * @see REF-OT-OTFF
 */
function fontTablesToFace(font: ReadFont): NamedFontFace | undefined {
  const metrics = fontTablesToMetrics(font.view, font.tables, blockCharacters(MEASURED_BLOCKS));
  const { name } = font.tables;

  return name === undefined || metrics === undefined
    ? undefined
    : parseNamedFace(font, tableToView(font.view, name), metrics);
}

/**
 * Reads a TrueType or OpenType font file — imported, or given by the browser for an installed
 * font — into a face, once, its table directory read once: its family from its `name` table
 * (name ID 16, else 1), its weight and style from its `OS/2` table, the metrics F05 reads for
 * the characters of the measured blocks, and its file with its version and whether it may be
 * embedded (ADR-0053). Every read is checked against its table: a file that is not such a font,
 * a collection or a WOFF among them, or a damaged one, gives no face, never an error.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @returns the face and its family; undefined for a file that gives no metrics, names no
 * family, or whose `OS/2` table is shorter than 64 bytes
 * @see REF-OT-OTFF
 */
export function parseFontFace(file: Readonly<Uint8Array>): NamedFontFace | undefined {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const tables = parseFontTables(view, 0);

  return tables === undefined ? undefined : fontTablesToFace({ file, tables, view });
}
