import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  LARGE_TABLE_LENGTH,
  TRUE_TYPE_VERSION,
  largeTableCollection,
  partsDigest,
} from "../../../tests";
import { unlistedFaceReason } from "../../model";
import { parseFontFace } from "../font-face";

import { collectionFaceToParts } from "./collection-face-to-parts";
import { facePartsToFile } from "./face-parts-to-file";
import { parseCollectionOffsets } from "./parse-collection-offsets";

/**
 * The parts of the first face of `largeTableCollection`, its large table among them as a view.
 *
 * @returns its directory written anew, then its tables as views of the collection
 */
function largeFaceParts(): readonly Uint8Array[] {
  const collection = largeTableCollection(TRUE_TYPE_VERSION);
  const [directory = 0] = parseCollectionOffsets(new DataView(collection.buffer));

  return collectionFaceToParts(collection, directory);
}

describe("facePartsToFile (REF-ECMA-262-TYPEDARRAY-SET)", () => {
  it("[F18.AC13] writes each part after the one before it, in one buffer of their total size", () => {
    // Lengths 3, 1, 0 and 2: 6 bytes; the parts start at 0, 3, 4 and 4.
    const file = facePartsToFile([
      Uint8Array.of(1, 2, 3),
      Uint8Array.of(4),
      new Uint8Array(),
      Uint8Array.of(5, 6),
    ]);

    expect([...file]).toEqual([1, 2, 3, 4, 5, 6]);
    expect(file.buffer.byteLength).toBe(6);
  });

  it("[F18.AC13] copies a view from its own place in its buffer, into a buffer of its own", () => {
    // A view of 2 bytes at 3 in [0, 1, 2, 3, 4, 5]: the bytes 3 and 4, not the buffer's first.
    const buffer = Uint8Array.of(0, 1, 2, 3, 4, 5);
    const file = facePartsToFile([buffer.subarray(3, 5), Uint8Array.of(9)]);

    expect([...file]).toEqual([3, 4, 9]);
    expect(file.buffer).not.toBe(buffer.buffer);
  });

  it(
    "[F18.AC13] writes a part of 200 MB on any machine, the CI's Linux included",
    // alone: 29 ms, the median of 5 runs of its file (EN-129); 20 s holds a slower machine's
    // allocation of 200 MB.
    { timeout: 20_000 },
    () => {
      // A view of 199 999 999 bytes from 1 in a buffer of 200 000 000, its last byte 5, then one
      // byte 7: 200 000 000 bytes, the view's last at 199 999 998, the 7 at 199 999 999.
      const buffer = new Uint8Array(200_000_000);
      const large = buffer.fill(5, 199_999_999).subarray(1);
      const file = facePartsToFile([large, Uint8Array.of(7)]);

      expect(file.length).toBe(200_000_000);
      expect(file[0]).toBe(0);
      expect(file[199_999_998]).toBe(5);
      expect(file[199_999_999]).toBe(7);
    },
  );

  it("gives an empty file for no part", () => {
    expect(facePartsToFile([])).toEqual(new Uint8Array());
  });

  test.prop({ parts: fc.array(fc.uint8Array({ maxLength: 20 }), { maxLength: 10 }) })(
    "gives the parts' bytes one after the other",
    ({ parts }) => {
      expect([...facePartsToFile(parts)]).toEqual(parts.flatMap((part) => [...part]));
    },
  );

  it(
    "[F18.AC13] [F18.AC19] writes the first face of a collection holding a table of 191 134 509 bytes",
    // alone: 184 ms, the median of 5 runs of its file (EN-129); 20 s is over 100 times it.
    { timeout: 20_000 },
    () => {
      // The face's parts hold the large table as a view: joined into one buffer, the file has the
      // parts' total length and the digest of the parts read in order.
      const parts = largeFaceParts();
      const file = facePartsToFile(parts);

      expect(file.length).toBe(parts.reduce((total, part) => total + part.length, 0));
      expect(file.length).toBeGreaterThan(LARGE_TABLE_LENGTH);
      expect(partsDigest([file])).toBe(partsDigest(parts));
    },
  );

  it(
    "[F17.AC14] [F18.AC19] reads the face of that large file, then refuses it: no basic Latin letters",
    // alone: 14 ms, the median of 5 runs of its file (EN-129); 20 s, as for the file it reads.
    { timeout: 20_000 },
    () => {
      // The face maps A, B, C and a alone: read, Test Symbols 400, but lacking 48 of the 52 letters.
      const named = parseFontFace(facePartsToFile(largeFaceParts()));

      expect(named).toMatchObject({ face: { weight: 400 }, family: "Test Symbols" });

      expect(named === undefined ? undefined : unlistedFaceReason(named.face)).toBe(
        "no-latin-letters",
      );
    },
  );
});
