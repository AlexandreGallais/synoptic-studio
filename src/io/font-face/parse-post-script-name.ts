import { parseFontName } from "../font-name";
import { parseFontTables, tableToView } from "../font-tables";

/** Name ID 6, the PostScript name of the face, unique to it, such as `Helvetica-Bold`. */
const POST_SCRIPT_NAME = 6;

/**
 * Reads the PostScript name of a font file's face (name ID 6 of its `name` table), the name a
 * browser gives an installed face by (`FontData.postscriptName`, `REF-WICG-LOCAL-FONT-ACCESS`),
 * so that a face given within its whole collection is found among the collection's faces. A face
 * of a collection is read in the collection, at its directory, no file of its own written.
 *
 * @kind format
 * @param file - the bytes of a font file, or of a collection
 * @param directory - where the face's table directory starts: 0 in a font file,
 * `tableDirectoryOffsets[i]` in a collection
 * @returns its PostScript name; undefined for a file that is not a font that can be measured, or
 * without a `name` table or a name ID 6
 */
export function parsePostScriptName(
  file: Readonly<Uint8Array>,
  directory: number,
): string | undefined {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const name = parseFontTables(view, directory)?.name;

  return name === undefined ? undefined : parseFontName(tableToView(view, name), POST_SCRIPT_NAME);
}
