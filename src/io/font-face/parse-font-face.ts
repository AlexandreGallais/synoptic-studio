import { MEASURED_BLOCKS, blockCharacters, isEmbeddable } from "../../model";
import {
  parseEmbeddingPermissions,
  parseFontMetrics,
  parseFontTables,
  parseTableDirectory,
  tableToView,
} from "../font-file";
import { parseFamilyName } from "../font-name";

import { parseFontStyle } from "./parse-font-style";

import type { FontFile, FontMetrics, NamedFontFace } from "../../model";

/** Offset of the table's version in the `OS/2` table. */
const OS2_VERSION_OFFSET = 0;

/**
 * Reads the family a font file names, from its `name` table.
 *
 * @kind format
 * @param view - the font file
 * @returns the family's name; undefined when the file has no `name` table or names no family
 * @see REF-OT-NAME
 */
function parseFileFamily(view: DataView): string | undefined {
  const { name } = parseTableDirectory(view);

  return name === undefined ? undefined : parseFamilyName(tableToView(view, name));
}

/**
 * Keeps a font file with whether its `fsType`, read with its `OS/2` table's version, permits
 * embedding it whole (`isEmbeddable`, `DERIV-font-embedding`, step 3).
 *
 * @kind format
 * @param file - the bytes of the font file
 * @param os2 - its `OS/2` table, at least 30 bytes long
 * @returns the file and its permission
 * @see DERIV-font-embedding
 */
function parseFontFile(file: Readonly<Uint8Array>, os2: DataView): FontFile {
  return {
    bytes: file,
    isEmbeddable: isEmbeddable(parseEmbeddingPermissions(os2), os2.getUint16(OS2_VERSION_OFFSET)),
  };
}

/**
 * Reads the face of a font file whose metrics are read: its family, its weight and style, and
 * its file with its permission.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @param os2 - its `OS/2` table, at least 30 bytes long
 * @param metrics - its metrics
 * @returns the face and its family; undefined when the file names no family or its `OS/2`
 * table is too short for its style
 * @see REF-OT-OS2
 */
function parseNamedFace(
  file: Readonly<Uint8Array>,
  os2: DataView,
  metrics: FontMetrics,
): NamedFontFace | undefined {
  const family = parseFileFamily(new DataView(file.buffer, file.byteOffset, file.byteLength));
  const style = parseFontStyle(os2);

  return family === undefined || style === undefined
    ? undefined
    : { face: { ...style, file: parseFontFile(file, os2), metrics }, family };
}

/**
 * Reads a TrueType or OpenType font file — imported, or given by the browser for an installed
 * font — into a face, once: its family from its `name` table (name ID 16, else 1), its weight
 * and style from its `OS/2` table, the metrics F05 reads for the characters of the measured
 * blocks, and its file with whether it may be embedded (ADR-0053). Every read is checked
 * against its table: a file that is not such a font, a collection or a WOFF among them, or a
 * damaged one, gives no face, never an error.
 *
 * @kind format
 * @param file - the bytes of the font file
 * @returns the face and its family; undefined for a file that gives no metrics, names no
 * family, or whose `OS/2` table is shorter than 64 bytes
 * @see REF-OT-OTFF
 */
export function parseFontFace(file: Readonly<Uint8Array>): NamedFontFace | undefined {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const tables = parseFontTables(view);
  const metrics = parseFontMetrics(file, blockCharacters(MEASURED_BLOCKS));

  return tables === undefined || metrics === undefined
    ? undefined
    : parseNamedFace(file, tableToView(view, tables.os2), metrics);
}
