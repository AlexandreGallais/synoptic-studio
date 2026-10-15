import { describe, expect, it } from "vitest";

import { OPEN_TYPE_VERSION, TRUE_TYPE_VERSION, fontCollection, fontFile } from "../../../tests";

import { parseCollectionFaces } from "./parse-collection-faces";

/** A face of one table: 32 bytes. */
const FIRST = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] });

/** A face of two tables: 56 bytes. */
const SECOND = fontFile(OPEN_TYPE_VERSION, { cdef: [9, 8, 7, 6, 5], wxyz: [4, 3, 2, 1] });

describe("parseCollectionFaces (REF-OT-OTFF)", () => {
  it("[F17.AC4] gives each face of a collection as its own file, in the collection's order", () => {
    expect(parseCollectionFaces(fontCollection([FIRST, SECOND]))).toEqual([FIRST, SECOND]);
  });

  it("[F17.AC4] gives a font file back as it is, the very bytes", () => {
    const faces = parseCollectionFaces(FIRST);

    expect(faces).toHaveLength(1);
    expect(faces[0]).toBe(FIRST);
  });

  it("gives back a file that is not a font too, for its reader to refuse", () => {
    const file = Uint8Array.from([1, 2, 3]);

    expect(parseCollectionFaces(file)).toEqual([file]);
  });
});
