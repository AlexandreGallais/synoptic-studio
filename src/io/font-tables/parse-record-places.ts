import { elementAddress } from "../../math";

import { SFNT_VERSION } from "./sfnt-version";
import { TABLE_DIRECTORY_LAYOUT } from "./table-directory-layout";

/**
 * The `sfntVersion` values of a TrueType or OpenType font: `0x00010000` and `OTTO`
 * (`REF-OT-OTFF`), and `true`, which Apple's systems read as TrueType (`REF-APPLE-TT-SCALER`).
 */
const SFNT_VERSIONS: ReadonlySet<number> = new Set([
  SFNT_VERSION.appleTrueType,
  SFNT_VERSION.cff,
  SFNT_VERSION.trueType,
]);

/**
 * Where the table records of a table directory sit: none for a directory that is not a TrueType
 * or OpenType font's, or that does not fit in the file. A font file has its directory at byte 0;
 * a collection has one per face, at its `tableDirectoryOffsets[i]` (`REF-OT-OTFF`).
 *
 * @kind format
 * @param view - the file
 * @param directory - where the table directory starts, in bytes from the file's start
 * @returns the offset of each table record, from the file's start
 */
export function parseRecordPlaces(view: Readonly<DataView>, directory: number): readonly number[] {
  const { headerSize, numTablesOffset, recordSize } = TABLE_DIRECTORY_LAYOUT;
  const records = directory + headerSize;
  const isFont = records <= view.byteLength && SFNT_VERSIONS.has(view.getUint32(directory));
  const count = isFont ? view.getUint16(directory + numTablesOffset) : 0;

  // The records end at the address of the one just past the last.
  return elementAddress(records, recordSize, count) > view.byteLength
    ? []
    : Array.from({ length: count }, (unused, index) => elementAddress(records, recordSize, index));
}
