import { elementAddress, scan } from "../../math";
import {
  TABLE_DIRECTORY_LAYOUT,
  lengthToFourByteBoundary,
  parseTableRecords,
  unsignedToBigEndian,
} from "../font-tables";

import type { FontTableRecord } from "../font-tables";

/** One table of the face: where its record sits, and where the table sits, in the collection. */
type FaceTable = FontTableRecord & {
  /** Offset of its table record, from the collection's start. */
  readonly record: number;
};

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
 */
function tablesToPlaces(tables: readonly FaceTable[]): readonly PlacedTable[] {
  const { headerSize, recordSize } = TABLE_DIRECTORY_LAYOUT;
  // The directory's size: the address just past its last record.
  const first = elementAddress(headerSize, recordSize, tables.length);

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
 * The zeros that pad a table to the next four-byte boundary: its room in the file
 * (`lengthToFourByteBoundary`) less its length, from 0 to 3.
 *
 * Formula: ⌈length / 4⌉ · 4 − length.
 *
 * @kind format
 * @param length - the table's length, in bytes
 * @returns the bytes of padding after it
 */
function paddingToFourByteBoundary(length: number): number {
  return lengthToFourByteBoundary(length) - length;
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
 */
function placedTableToParts<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
  placed: PlacedTable,
): readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[] {
  const { length, offset } = placed.table;
  const padding = paddingToFourByteBoundary(length);
  const table = file.subarray(offset, offset + length);

  return padding === 0 ? [table] : [table, new Uint8Array(padding)];
}

/**
 * Gives a face of a collection as the parts of a font file of its own, in the layout Skia writes
 * on macOS (`REF-SKIA-MAC-STREAM-CODE`): first its table directory, written anew — its header
 * copied with `numTables` set to the tables kept, its records copied with each offset rewritten
 * from the new file's start —, then each of its tables in the order of its records
 * (`parseTableRecords`), a view of the collection's bytes, never copied, followed by the zeros
 * that pad it to a four-byte boundary. Joined in that order, the parts are the face's file; a
 * table of 191 MB stays a view, where an array of its numbers would pass the longest array an
 * engine allows. Unlike Skia, which computes them again, the checksums and the search
 * fields are copied: each table is copied whole, so its checksum still holds; the search fields
 * are stale when a table is left out, and a parser derives them from `numTables`; the `head`
 * table's `checksumAdjustment` is copied as the collection holds it, "not used for collection
 * files" and stale for the face's file (`REF-OT-OTFF`). A table that does not fit in the
 * collection is left out.
 *
 * @kind format
 * @template Buffer - the buffer the bytes sit in, kept by every view of them
 * @param file - the collection's bytes
 * @param directory - where the face's table directory starts, `tableDirectoryOffsets[i]`
 * @returns the face's parts, its directory first; a directory's header alone for a directory
 * that is not a font's
 */
export function collectionFaceToParts<Buffer extends ArrayBufferLike>(
  file: Readonly<Uint8Array<Buffer>>,
  directory: number,
): readonly Readonly<Uint8Array<ArrayBuffer | Buffer>>[] {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const { headerSize, numTablesOffset } = TABLE_DIRECTORY_LAYOUT;
  const places = tablesToPlaces(parseTableRecords(view, directory));
  const numberTables = directory + numTablesOffset;
  const header = Uint8Array.from([
    ...file.subarray(directory, numberTables),
    ...unsignedToBigEndian(places.length, UINT16_SIZE),
    ...file.subarray(numberTables + UINT16_SIZE, directory + headerSize),
    ...places.flatMap((placed) => placedTableToRecord(file, placed)),
  ]);

  return [header, ...places.flatMap((placed) => placedTableToParts(file, placed))];
}
