import { describe, expect, it } from "vitest";

import {
  HELVETICA_COLLECTION_PATH,
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  joinedParts,
  partsDigest,
  systemFont,
} from "../../../tests";

import { parseCollectionFaces } from "./parse-collection-faces";

/** A face of one table: 32 bytes. */
const FIRST = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] });

/** A face of two tables: 56 bytes. */
const SECOND = fontFile(OPEN_TYPE_VERSION, { cdef: [9, 8, 7, 6, 5], wxyz: [4, 3, 2, 1] });

/** Helvetica of macOS, a collection of 6 faces, read where the machine has it. */
const HELVETICA = systemFont(HELVETICA_COLLECTION_PATH);

describe("parseCollectionFaces (REF-OT-OTFF)", () => {
  it("[F17.AC4] gives each face of a collection as its own file, in the collection's order", () => {
    expect(
      parseCollectionFaces(fontCollection([FIRST, SECOND])).map((parts) => joinedParts(parts)),
    ).toEqual([FIRST, SECOND]);
  });

  it("[F17.AC4] gives the face of a collection of one face as its own file, without the collection's header", () => {
    expect(
      parseCollectionFaces(fontCollection([FIRST])).map((parts) => joinedParts(parts)),
    ).toEqual([FIRST]);
  });

  it("[F17.AC4] gives a font file back as it is, the very bytes, one part", () => {
    const faces = parseCollectionFaces(FIRST);

    expect(faces).toHaveLength(1);
    expect(faces[0]).toHaveLength(1);
    expect(faces[0]?.[0]).toBe(FIRST);
  });

  it("gives back a file that is not a font too, for its reader to refuse", () => {
    const file = Uint8Array.from([1, 2, 3]);

    expect(parseCollectionFaces(file)).toEqual([[file]]);
  });

  it.skipIf(HELVETICA === undefined)(
    "[F17.AC10] gives Helvetica.ttc's 6 faces byte for byte as collectionFaceToFile wrote them",
    () => {
      // Length and SHA-256 of each face as `collectionFaceToFile` wrote it before US-052, read on
      // macOS 27's Helvetica.ttc (2 338 064 bytes) on 2026-10-16.
      const faces = parseCollectionFaces(HELVETICA ?? new Uint8Array());

      expect(
        faces.map((parts) => [
          parts.reduce((sum, part) => sum + part.length, 0),
          partsDigest(parts),
        ]),
      ).toEqual([
        [416_420, "46be68e81b108459a4e5358eddc5caefca2aac7b10b5f7da710520dab4f4486a"],
        [437_820, "1cce2b1746ac2d78a5e067040d2df303b711dd617dcfdef63b39df63d7c15379"],
        [660_564, "9d2d4d1a5d517bbe16a4006cbae77def00e94c076aed5211bc5cdfb27d372f53"],
        [355_044, "67563bf396bd908d9606535f933bd852a6c3762aea70d2b7433338fed55ceb49"],
        [319_036, "b6d030e869c822f8df4083d4c32082349465bdd095f79956a696874477557f59"],
        [325_764, "6e443176752ddf7760cf94daea14cd35f5433d948db8c7078fff497f128354f7"],
      ]);
    },
  );
});
