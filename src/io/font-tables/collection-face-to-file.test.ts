import { describe, expect, it } from "vitest";

import {
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  uint16,
  uint32,
} from "../../../tests";

import { collectionFaceToFile } from "./collection-face-to-file";
import { parseTableDirectory } from "./parse-table-directory";

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

describe("collectionFaceToFile (REF-OT-OTFF, REF-SKIA-MAC-STREAM-CODE)", () => {
  it("[F17.AC4] writes each face alone, as the face's own file", () => {
    // Each face's table offsets, counted from the collection's start (48 for abcd = 20 + 28),
    // are rewritten from the face's start: the face's own file comes back byte for byte.
    expect(collectionFaceToFile(COLLECTION, 20)).toEqual(FIRST);
    expect(collectionFaceToFile(COLLECTION, 52)).toEqual(SECOND);
  });

  it("[F17.AC4] copies a table the face shares with another face", () => {
    // The second face's first record (cdef) at 52 + 12 = 64: its offset (at 72) set to abcd's,
    // 48, its length (at 76) to 3; abcd's 3 bytes then padded to 4 bytes.
    const shared = changedCollection(72, [...uint32(48), ...uint32(3)]);
    const face = collectionFaceToFile(shared, 52);

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
    const face = collectionFaceToFile(Uint8Array.from(marked).fill(7, 36, 40), 20);

    expect([...face.subarray(6, 12)]).toEqual([0, 16, 0, 0, 0, 0]);
    expect([...face.subarray(12, 20)]).toEqual([97, 98, 99, 100, 7, 7, 7, 7]);
  });

  it("leaves out a table that ends past the collection, numTables counting those kept", () => {
    // The second face's cdef length (at 76) set to 1000: past the 108 bytes of the collection.
    const face = collectionFaceToFile(changedCollection(76, uint32(1000)), 52);

    // One record left: wxyz, 12 + 16 = 28, its 4 bytes there.
    expect([...face.subarray(4, 6)]).toEqual([0, 1]);

    expect(parseTableDirectory(new DataView(face.buffer), 0)).toEqual({
      wxyz: { length: 4, offset: 28 },
    });
  });

  it("writes a header of no table for a directory that is not a font's", () => {
    // At 0 the collection's own header, `ttcf` 1.0 with 2 faces: copied, numTables 0.
    expect([...collectionFaceToFile(COLLECTION, 0)]).toEqual([
      116, 116, 99, 102, 0, 0, 0, 0, 0, 0, 0, 2,
    ]);
  });
});
