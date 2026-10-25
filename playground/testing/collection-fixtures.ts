// Font files whose tables are written in one buffer, for the tests that once read the fonts of
// macOS (EN-067): a very large file, the import of a second family, never a font a machine
// happens to have. The playground's copy of the root's `tests/collection-fixtures.ts`, the part
// its tests use, so that they run on the playground's install alone (EN-176). Tests only.

import { tagBytes } from "./font-fixtures";

/** A table of zeros given by its length alone, too large to hold as numbers. */
type ZeroTable = {
  /** Its length, in bytes, each zero. */
  readonly zeros: number;
};

/** A table's bytes: as numbers, or zeros given by their count. */
type TableBody = readonly number[] | ZeroTable;

/** The tables of a font, by tag. */
export type FontTables = Readonly<Record<string, TableBody>>;

/**
 * The length of a table's bytes.
 *
 * @param body - its bytes, or its zeros
 * @returns its length
 */
function bodyLength(body: TableBody): number {
  return "zeros" in body ? body.zeros : body.length;
}

/** Sizes and offsets of the table directory (`REF-OT-OTFF`). */
const LAYOUT = {
  directoryHeaderSize: 12,
  recordSize: 16,
  tableAlignment: 4,
};

/** Where a table directory holds `sfntVersion` and `numTables` (`REF-OT-OTFF`). */
const DIRECTORY_AT = { numTables: 4, sfntVersion: 0 };

/** Where a table record holds its tag, offset and length (`REF-OT-OTFF`). */
const RECORD_AT = { length: 12, offset: 8, tag: 0, tagSize: 4 };

/**
 * The tables of a font file, read from its table directory (`REF-OT-OTFF`) apart from the
 * library's reader, to check it.
 *
 * @param file - a font file, not a collection
 * @returns each table's bytes, by tag, in the directory's order
 */
export function fileTables(file: Readonly<Uint8Array>): Record<string, readonly number[]> {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const records = Array.from(
    { length: view.getUint16(DIRECTORY_AT.numTables) },
    (unused, index): [string, readonly number[]] => {
      const at = LAYOUT.directoryHeaderSize + LAYOUT.recordSize * index;
      const offset = view.getUint32(at + RECORD_AT.offset);

      return [
        String.fromCodePoint(...file.subarray(at, at + RECORD_AT.tagSize)),
        [...file.subarray(offset, offset + view.getUint32(at + RECORD_AT.length))],
      ];
    },
  );

  return Object.fromEntries(records);
}

/**
 * The length of a table padded to a four-byte boundary (`REF-OT-OTFF`).
 *
 * @param length - its bytes
 * @returns the bytes it takes in the file
 */
function paddedLength(length: number): number {
  return Math.ceil(length / LAYOUT.tableAlignment) * LAYOUT.tableAlignment;
}

/**
 * The size of a face's table directory: its header, then one record per table (`REF-OT-OTFF`).
 *
 * @param tables - the face's tables, by tag
 * @returns its bytes
 */
function directorySize(tables: FontTables): number {
  return LAYOUT.directoryHeaderSize + LAYOUT.recordSize * Object.keys(tables).length;
}

/**
 * Where each face's table directory starts, one after the other after a header.
 *
 * @param faces - the tables of each face, by tag
 * @param headerSize - the bytes before the first directory
 * @returns each directory's start, then where the tables start
 */
function directoryStarts(faces: readonly FontTables[], headerSize: number): readonly number[] {
  const sizes = faces.map((tables) => directorySize(tables));

  return [...sizes, 0].map(
    (unused, index) => headerSize + sizes.slice(0, index).reduce((sum, size) => sum + size, 0),
  );
}

/**
 * Where each table starts: each once, a table given to several faces as the same array written
 * once, each on a four-byte boundary (`REF-OT-OTFF`).
 *
 * @param faces - the tables of each face, by tag
 * @param tablesStart - where the first table starts
 * @returns each table's start, by its bytes, and the end of the last
 */
function tableStarts(
  faces: readonly FontTables[],
  tablesStart: number,
): { readonly starts: ReadonlyMap<TableBody, number>; readonly end: number } {
  const bodies = [...new Set(faces.flatMap((tables) => Object.values(tables)))];
  const ends = [...bodies, []].map(
    (unused, index) =>
      tablesStart +
      bodies.slice(0, index).reduce((sum, body) => sum + paddedLength(bodyLength(body)), 0),
  );

  return {
    end: ends.at(-1) ?? tablesStart,
    starts: new Map(bodies.map((body, index) => [body, ends[index] ?? 0])),
  };
}

/** Where a face's table directory goes, and where each of its tables starts. */
type DirectoryPlace = {
  /** The bytes written into. */
  readonly bytes: Uint8Array;
  /** Where the directory starts. */
  readonly start: number;
  /** Each table's start, by its bytes. */
  readonly starts: ReadonlyMap<TableBody, number>;
};

/**
 * Writes a face's table directory: `sfntVersion`, `numTables`, then each record's tag, offset and
 * length, its checksum and the search fields left zero (`REF-OT-OTFF`).
 *
 * @param place - the bytes, where the directory starts, where each table starts
 * @param sfntVersion - the face's `sfntVersion`
 * @param tables - the face's tables, by tag
 */
function writeDirectory(place: DirectoryPlace, sfntVersion: number, tables: FontTables): void {
  const view = new DataView(place.bytes.buffer);
  const records = Object.entries(tables).entries();

  view.setUint32(place.start + DIRECTORY_AT.sfntVersion, sfntVersion);
  view.setUint16(place.start + DIRECTORY_AT.numTables, Object.keys(tables).length);

  for (const [record, [tag, body]] of records) {
    const at = place.start + LAYOUT.directoryHeaderSize + LAYOUT.recordSize * record;

    place.bytes.set(tagBytes(tag), at + RECORD_AT.tag);
    view.setUint32(at + RECORD_AT.offset, place.starts.get(body) ?? 0);
    view.setUint32(at + RECORD_AT.length, bodyLength(body));
  }
}

/**
 * Writes each table at its start; a table of zeros is already written, the buffer starting as
 * zeros.
 *
 * @param bytes - the buffer written into
 * @param starts - each table's start, by its bytes
 */
function writeBodies(bytes: Uint8Array, starts: ReadonlyMap<TableBody, number>): void {
  for (const [body, start] of starts) {
    bytes.set("zeros" in body ? [] : body, start);
  }
}

/**
 * Faces written in one buffer: a header of `headerSize` bytes left as zeros, then each face's
 * table directory, then each table once (`tableStarts`), every offset from the buffer's start
 * (`REF-OT-OTFF`). Records keep each face's order of tags, their checksums and the search fields
 * zero, as `fontFile` writes them.
 *
 * @param sfntVersion - each face's `sfntVersion`
 * @param faces - the tables of each face, by tag
 * @param headerSize - the bytes left before the first directory, for a collection's header
 * @returns the bytes
 */
function writtenFaces(
  sfntVersion: number,
  faces: readonly FontTables[],
  headerSize: number,
): Uint8Array {
  const directories = directoryStarts(faces, headerSize);
  const tables = tableStarts(faces, directories.at(-1) ?? headerSize);
  const bytes = new Uint8Array(tables.end);

  for (const [face, faceTables] of faces.entries()) {
    writeDirectory(
      { bytes, start: directories[face] ?? 0, starts: tables.starts },
      sfntVersion,
      faceTables,
    );
  }

  writeBodies(bytes, tables.starts);

  return bytes;
}

/**
 * A font file of large tables, written as `fontFile` writes one, its tables copied into one
 * buffer rather than spread: the table directory, then each table on a four-byte boundary.
 *
 * @param sfntVersion - `TRUE_TYPE_VERSION` or `OPEN_TYPE_VERSION`
 * @param tables - its tables, by tag
 * @returns the file's bytes
 */
export function tablesFile(sfntVersion: number, tables: FontTables): Uint8Array {
  return writtenFaces(sfntVersion, [tables], 0);
}

/** The length of a large table: over 191 MB, not a multiple of 4. */
export const LARGE_TABLE_LENGTH = 191_134_509;
