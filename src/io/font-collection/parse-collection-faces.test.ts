import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  OPEN_TYPE_VERSION,
  SHARED_SANS_TABLES,
  TRUE_TYPE_VERSION,
  fontCollection,
  fontFile,
  joinedParts,
  sharedTableCollection,
  tablesFile,
} from "../../../tests";

import { parseCollectionFaces } from "./parse-collection-faces";

/** A face of one table: 32 bytes. */
const FIRST = fontFile(TRUE_TYPE_VERSION, { abcd: [1, 2, 3] });

/** A face of two tables: 56 bytes. */
const SECOND = fontFile(OPEN_TYPE_VERSION, { cdef: [9, 8, 7, 6, 5], wxyz: [4, 3, 2, 1] });

/** Any table body of up to 12 bytes. */
const TABLE_BODY = fc.array(fc.integer({ max: 255, min: 0 }), { maxLength: 12 });

/** Any face of one to four tables of up to 12 bytes, its sfntVersion TrueType or OpenType. */
const FACE = fc.record({
  tables: fc.dictionary(fc.stringMatching(/^[a-z]{4}$/u), TABLE_BODY, { maxKeys: 4, minKeys: 1 }),
  version: fc.constantFrom(TRUE_TYPE_VERSION, OPEN_TYPE_VERSION),
});

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

  it(
    "[F17.AC10] [F18.AC19] gives each face of a collection sharing Inter's tables as its own file",
    { timeout: 20_000 },
    () => {
      // Inter's tables padded take 411 640 − 284 = 411 356 bytes; its name (1 948) and OS/2 (96)
      // replaced, each face holds 409 312 bytes of shared tables, 96 of OS/2 and its name of 88, 82
      // (padded to 84) or 86 (padded to 88): with its directory of 284, 409 780, 409 776, 409 780.
      const faces = SHARED_SANS_TABLES;
      const read = parseCollectionFaces(sharedTableCollection(TRUE_TYPE_VERSION, faces));

      expect(read.map((parts) => parts.reduce((sum, part) => sum + part.length, 0))).toEqual([
        409_780, 409_776, 409_780,
      ]);

      expect(read.map((parts) => joinedParts(parts))).toEqual(
        faces.map((tables) => tablesFile(TRUE_TYPE_VERSION, tables)),
      );
    },
  );

  test.prop({ faces: fc.array(FACE, { maxLength: 3, minLength: 1 }) })(
    "[F17.AC4] gives back the very faces a collection is made of",
    ({ faces }) => {
      const files = faces.map((face) => fontFile(face.version, face.tables));

      expect(
        parseCollectionFaces(fontCollection(files)).map((parts) => joinedParts(parts)),
      ).toEqual(files);
    },
  );
});
