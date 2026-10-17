import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { APPLE_COLOR_EMOJI_PATH, partsDigest, systemFont } from "../../../tests";

import { collectionFaceToParts } from "./collection-face-to-parts";
import { facePartsToFile } from "./face-parts-to-file";
import { parseCollectionOffsets } from "./parse-collection-offsets";

/** Apple Color Emoji of macOS, 192 120 460 bytes, read where the machine has it. */
const EMOJI = systemFont(APPLE_COLOR_EMOJI_PATH);

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

  it.skipIf(EMOJI === undefined)(
    "[F18.AC13] writes Apple Color Emoji's first face, its 191 MB sbix table included",
    { timeout: 20_000 },
    () => {
      // The face's parts hold the sbix table of 191 134 508 bytes as a view: joined into one
      // buffer, the file has the parts' total length and the digest of the parts read in order.
      const collection = EMOJI ?? new Uint8Array();
      const [directory = 0] = parseCollectionOffsets(
        new DataView(collection.buffer, collection.byteOffset, collection.byteLength),
      );
      const parts = collectionFaceToParts(collection, directory);
      const file = facePartsToFile(parts);

      expect(file.length).toBe(parts.reduce((total, part) => total + part.length, 0));
      expect(file.length).toBeGreaterThan(191_134_508);
      expect(partsDigest([file])).toBe(partsDigest(parts));
    },
  );
});
