import { scan } from "../../math";

import { lengthToFourByteBoundary } from "./length-to-four-byte-boundary";
import { parseRecordPlaces } from "./parse-record-places";
import { TABLE_DIRECTORY_LAYOUT } from "./table-directory-layout";
import { unsignedToBigEndian } from "./unsigned-to-big-endian";

/** One table of the face: where its record sits, and where the table sits, in the collection. */
type FaceTable = {
  /** Offset of its table record, from the collection's start. */
  readonly record: number;
  /** Offset of the table, from the collection's start. */
  readonly offset: number;
  /** Length of the table, in bytes. */
  readonly length: number;
};

/**
 * Reads the tables of a face of a collection that fit in the file, each with its record.
 *
 * @kind format
 * @param view - the collection
 * @param directory - where the face's table directory starts
 * @returns its tables, in the order of its records
 * @see REF-OT-OTFF
 */
function parseFaceTables(view: DataView, directory: number): readonly FaceTable[] {
  const { lengthField, offsetField } = TABLE_DIRECTORY_LAYOUT;

  return parseRecordPlaces(view, directory)
    .map((record) => ({
      length: view.getUint32(record + lengthField),
      offset: view.getUint32(record + offsetField),
      record,
    }))
    .filter((table) => table.offset + table.length <= view.byteLength);
}

/** A table of the face placed in its own file: where it starts and where the next one does. */
type PlacedTable = {
  /** The table, as the collection holds it. */
  readonly table: FaceTable;
  /** Offset of the table in the face's file. */
  readonly start: number;
  /** Offset of the next table: past this one, padded to a four-byte boundary. */
  readonly end: number;
};

/**
 * Places the tables of a face one after the other, from the end of its directory, each padded
 * to a four-byte boundary (`lengthToFourByteBoundary`).
 *
 * @kind format
 * @param tables - the face's tables, in the order of their records
 * @returns each table and where it starts in the face's file
 * @see REF-OT-OTFF
 */
function tablesToPlaces(tables: readonly FaceTable[]): readonly PlacedTable[] {
  const { headerSize, recordSize } = TABLE_DIRECTORY_LAYOUT;
  const first = headerSize + recordSize * tables.length;

  return scan<PlacedTable, FaceTable>(
    tables,
    (before, table) => ({
      end: before.end + lengthToFourByteBoundary(table.length),
      start: before.end,
      table,
    }),
    { end: first, start: first, table: { length: 0, offset: 0, record: 0 } },
  );
}

/** Size of an `Offset32` and of a `uint32`, in bytes. */
const UINT32_SIZE = 4;

/** Size of a `uint16`, in bytes. */
const UINT16_SIZE = 2;

/**
 * Writes the record of a table placed in the face's file: its tag and checksum copied, its
 * offset rewritten from the file's start, its length copied.
 *
 * @kind format
 * @param file - the collection's bytes
 * @param placed - the table and where it starts in the face's file
 * @returns the record's 16 bytes
 * @see REF-OT-OTFF
 */
function placedTableToRecord(file: Readonly<Uint8Array>, placed: PlacedTable): readonly number[] {
  const { lengthField, offsetField, recordSize } = TABLE_DIRECTORY_LAYOUT;
  const { record } = placed.table;

  return [
    ...file.subarray(record, record + offsetField),
    ...unsignedToBigEndian(placed.start, UINT32_SIZE),
    ...file.subarray(record + lengthField, record + recordSize),
  ];
}

/**
 * Gives a table placed in the face's file as its parts: a view of its bytes in the collection,
 * never copied, then the zeros up to the next table, when it needs any.
 *
 * @kind format
 * @template Buffer - the buffer the bytes sit in, kept by every view of them
 * @param file - the collection's bytes
 * @param placed - the table and where it starts and ends in the face's file
 * @returns the table's view, then its padding
 * @see REF-OT-OTFF
 */
function placedTableToParts<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
  placed: PlacedTable,
): readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[] {
  const { length, offset } = placed.table;
  const padding = placed.end - placed.start - length;
  const table = file.subarray(offset, offset + length);

  return padding === 0 ? [table] : [table, new Uint8Array(padding)];
}

/**
 * Gives a face of a collection as the parts of a font file of its own, as Skia assembles one on
 * macOS (`REF-SKIA-MAC-STREAM-CODE`): first its table directory, written anew — its header copied
 * with `numTables` set to the tables kept, its records copied with each offset rewritten from the
 * new file's start —, then each of its tables in the order of its records, a view of the
 * collection's bytes, never copied, followed by the zeros that pad it to a four-byte boundary.
 * Joined in that order, the parts are the face's file; a table of 191 MB stays a view, where an
 * array of its numbers would pass the longest array an engine allows (US-052). The search fields
 * are copied, which a parser derives from `numTables` (`REF-OT-OTFF`); a table that does not fit
 * in the collection is left out.
 *
 * @kind format
 * @template Buffer - the buffer the bytes sit in, kept by every view of them
 * @param file - the collection's bytes
 * @param directory - where the face's table directory starts, `tableDirectoryOffsets[i]`
 * @returns the face's parts, its directory first; a directory's header alone for a directory
 * that is not a font's
 * @see REF-OT-OTFF
 */
export function collectionFaceToParts<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
  directory: number,
): readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[] {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const { headerSize, numTablesOffset } = TABLE_DIRECTORY_LAYOUT;
  const places = tablesToPlaces(parseFaceTables(view, directory));
  const numberTables = directory + numTablesOffset;
  const header = Uint8Array.from([
    ...file.subarray(directory, numberTables),
    ...unsignedToBigEndian(places.length, UINT16_SIZE),
    ...file.subarray(numberTables + UINT16_SIZE, directory + headerSize),
    ...places.flatMap((placed) => placedTableToRecord(file, placed)),
  ]);

  return [header, ...places.flatMap((placed) => placedTableToParts(file, placed))];
}
