import { describe, expect, it } from "vitest";

import {
  LARGE_TABLE_LENGTH,
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  joinedParts,
  largeTableCollection,
  sharedSansTables,
  sharedTableCollection,
  uint16,
  uint32,
} from "../../../tests";
import { parseTableDirectory } from "../font-tables";

import { collectionFaceToParts } from "./collection-face-to-parts";
import { parseCollectionOffsets } from "./parse-collection-offsets";

/** A face of one table of 3 bytes: 28 + 4 = 32 bytes. */
const FIRST = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] });

/** A face of two tables, CFF outlines: 12 + 2 × 16 = 44, then 8 and 4 bytes = 56 bytes. */
const SECOND = fontFile(OPEN_TYPE_VERSION, { cdef: [9, 8, 7, 6, 5], wxyz: [4, 3, 2, 1] });

/** The two faces in a collection: header 20 bytes, the first at 20, the second at 52. */
const COLLECTION = fontCollection([FIRST, SECOND]);

/**
 * The collection with some bytes changed.
 *
 * @param at - where the bytes start
 * @param bytes - the bytes written there
 * @returns a copy of the collection
 */
function changedCollection(at: number, bytes: readonly number[]): Uint8Array {
  const copy = [...COLLECTION];

  copy.splice(at, bytes.length, ...bytes);

  return Uint8Array.from(copy);
}

/**
 * A face of a collection, its parts joined into its file.
 *
 * @param file - the collection
 * @param directory - where the face's table directory starts
 * @returns its bytes, the parts joined
 */
function faceFile(file: Readonly<Uint8Array>, directory: number): Uint8Array {
  return joinedParts(collectionFaceToParts(file, directory));
}

/**
 * Where each part of a face comes from: its length, and its offset in the collection when it is a
 * view of it, else `new`.
 *
 * @param parts - the face's parts
 * @returns each part's length and origin
 */
function partPlaces(
  parts: readonly Readonly<Uint8Array>[],
): readonly (readonly [number, number | "new"])[] {
  return parts.map((part) => [
    part.length,
    part.buffer === COLLECTION.buffer ? part.byteOffset : "new",
  ]);
}

/** The large table of a collection's first face, as its directory says and as its part gives it. */
type SbixPlace = {
  /** The table's length, from its record. */
  readonly tableLength: number;
  /** The table's offset in the collection, from its record. */
  readonly tableOffset: number;
  /** Whether the part of that length is a view of the collection's bytes. */
  readonly isView: boolean;
  /** Where that part starts in the collection. */
  readonly partOffset: number;
};

/**
 * Finds the `sbix` table of a collection's first face and the part that gives it.
 *
 * @param file - the collection of `largeTableCollection`
 * @returns the table's record and its part's place
 */
function largeSbix(file: Readonly<Uint8Array>): SbixPlace {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const [directory = 0] = parseCollectionOffsets(view);
  const { sbix = { length: 0, offset: 0 } } = parseTableDirectory(view, directory);
  const part =
    collectionFaceToParts(file, directory).find((each) => each.length === sbix.length) ??
    new Uint8Array();

  return {
    isView: part.buffer === file.buffer,
    partOffset: part.byteOffset - file.byteOffset,
    tableLength: sbix.length,
    tableOffset: sbix.offset,
  };
}

describe("collectionFaceToParts (REF-OT-OTFF, REF-SKIA-MAC-STREAM-CODE)", () => {
  it("[F17.AC4] gives each face alone, its parts joined into the face's own file", () => {
    // Each face's table offsets, counted from the collection's start (48 for abcd = 20 + 28),
    // are rewritten from the face's start: the face's own file comes back byte for byte.
    expect(faceFile(COLLECTION, 20)).toEqual(FIRST);
    expect(faceFile(COLLECTION, 52)).toEqual(SECOND);
  });

  it("[F17.AC10] gives the directory, then each table as a view of the collection, then its padding", () => {
    // First face: its directory 12 + 16 = 28 bytes, written anew; abcd's 3 bytes, a view at
    // 20 + 28 = 48; 1 zero of padding, written anew.
    expect(partPlaces(collectionFaceToParts(COLLECTION, 20))).toEqual([
      [28, "new"],
      [3, 48],
      [1, "new"],
    ]);

    // Second face: its directory 12 + 32 = 44 bytes; cdef's 5 bytes at 52 + 44 = 96, padded by
    // 3 zeros; wxyz's 4 bytes at 96 + 8 = 104, already on a four-byte boundary, no padding.
    expect(partPlaces(collectionFaceToParts(COLLECTION, 52))).toEqual([
      [44, "new"],
      [5, 96],
      [3, "new"],
      [4, 104],
    ]);
  });

  it("[F17.AC4] copies a table the face shares with another face", () => {
    // The second face's first record (cdef) at 52 + 12 = 64: its offset (at 72) set to abcd's,
    // 48, its length (at 76) to 3; abcd's 3 bytes then padded to 4 bytes.
    const shared = changedCollection(72, [...uint32(48), ...uint32(3)]);
    const face = faceFile(shared, 52);

    // 44 bytes of directory, then cdef (now [1, 2, 3], padded to 4) at 44, wxyz at 48.
    expect(parseTableDirectory(new DataView(face.buffer), 0)).toEqual({
      cdef: { length: 3, offset: 44 },
      wxyz: { length: 4, offset: 48 },
    });

    expect([...face.subarray(44, 52)]).toEqual([1, 2, 3, 0, 4, 3, 2, 1]);
  });

  it("[F17.AC4] keeps each record's tag and checksum, and the directory's search fields", () => {
    // The first face's record at 20 + 12 = 32: its checksum at 36; its search fields at 26.
    const marked = changedCollection(26, [...uint16(16), ...uint16(0), ...uint16(0)]);
    const face = faceFile(Uint8Array.from(marked).fill(7, 36, 40), 20);

    expect([...face.subarray(6, 12)]).toEqual([0, 16, 0, 0, 0, 0]);
    expect([...face.subarray(12, 20)]).toEqual([97, 98, 99, 100, 7, 7, 7, 7]);
  });

  it("leaves out a table that ends past the collection, numTables counting those kept", () => {
    // The second face's cdef length (at 76) set to 1000: past the 108 bytes of the collection.
    const face = faceFile(changedCollection(76, uint32(1000)), 52);

    // One record left: wxyz, 12 + 16 = 28, its 4 bytes there.
    expect([...face.subarray(4, 6)]).toEqual([0, 1]);

    expect(parseTableDirectory(new DataView(face.buffer), 0)).toEqual({
      wxyz: { length: 4, offset: 28 },
    });
  });

  it("writes a header of no table for a directory that is not a font's", () => {
    // At 0 the collection's own header, `ttcf` 1.0 with 2 faces: copied, numTables 0.
    expect(collectionFaceToParts(COLLECTION, 0).map((part) => [...part])).toEqual([
      [116, 116, 99, 102, 0, 0, 0, 0, 0, 0, 0, 2],
    ]);
  });

  it("[F17.AC4] [F18.AC19] gives a table two faces share as the same view of the collection for each", () => {
    // Three faces of Inter's tables, their directories of 284 bytes at 24, 308 and 592: the first
    // table, GDEF, written once at 24 + 3 × 284 = 876, its 1 036 bytes; the name tables differ.
    const collection = sharedTableCollection(TRUE_TYPE_VERSION, sharedSansTables());
    const faces = [24, 308, 592].map((directory) =>
      collectionFaceToParts(collection, directory).map((part) => [
        part.length,
        part.buffer === collection.buffer ? part.byteOffset : "new",
      ]),
    );

    expect(faces.map((parts) => parts[1])).toEqual([
      [1036, 876],
      [1036, 876],
      [1036, 876],
    ]);

    expect(faces[0]).not.toEqual(faces[1]);
  });

  it(
    "[F17.AC10] [F18.AC19] gives a table of 191 134 509 bytes as a view, no array of its bytes",
    // alone: 1 ms, the median of 5 runs of its file (EN-129); 20 s, as for the other tests of a
    // table this large.
    { timeout: 20_000 },
    () => {
      // Before US-052 the face was spread into an array of numbers, past the longest array V8
      // allows (about 134 million): "invalid array length", the process ended.
      const sbix = largeSbix(largeTableCollection(TRUE_TYPE_VERSION));

      expect(sbix.tableLength).toBe(LARGE_TABLE_LENGTH);
      expect(sbix.isView).toBe(true);
      expect(sbix.partOffset).toBe(sbix.tableOffset);
    },
  );
});
