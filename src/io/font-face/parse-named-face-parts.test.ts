import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  HELVETICA_COLLECTION_PATH,
  TRUE_TYPE_VERSION,
  WINDOWS_PLATFORM,
  fontCollection,
  fontFile,
  joinedParts,
  nameTable,
  partsDigest,
  systemFont,
  testFontTables,
} from "../../../tests";

import { parseNamedFaceParts } from "./parse-named-face-parts";

/**
 * A face that can be measured, named by its PostScript name alone.
 *
 * @param postScriptName - its name ID 6
 * @returns its file
 */
function namedFace(postScriptName: string): Uint8Array {
  const name = nameTable([
    {
      encoding: 1,
      language: ENGLISH_US,
      nameId: 6,
      platform: WINDOWS_PLATFORM,
      text: postScriptName,
    },
  ]);

  return fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), name });
}

/** Arial's regular face. */
const REGULAR = namedFace("ArialMT");

/** Arial's bold face. */
const BOLD = namedFace("Arial-BoldMT");

/** Helvetica of macOS, a collection of 6 faces, read where the machine has it. */
const HELVETICA = systemFont(HELVETICA_COLLECTION_PATH);

describe("parseNamedFaceParts (REF-OT-OTFF, REF-OT-NAME)", () => {
  it("[F17.AC10] gives the face of that name in a collection, alone", () => {
    const collection = fontCollection([REGULAR, BOLD]);

    expect(joinedParts(parseNamedFaceParts(collection, "Arial-BoldMT") ?? [])).toEqual(BOLD);
    expect(joinedParts(parseNamedFaceParts(collection, "ArialMT") ?? [])).toEqual(REGULAR);
  });

  it("[F17.AC10] gives no face when a collection of several holds none of that name", () => {
    expect(parseNamedFaceParts(fontCollection([REGULAR, BOLD]), "Helvetica")).toBeUndefined();
  });

  it("[F17.AC4] gives the face of a collection of one face, whatever its name", () => {
    const parts = parseNamedFaceParts(fontCollection([BOLD]), "Helvetica") ?? [];

    expect(joinedParts(parts)).toEqual(BOLD);
  });

  it("[F17.AC4] gives a font file back as it is, one part, whatever its name", () => {
    const parts = parseNamedFaceParts(REGULAR, "Helvetica");

    expect(parts).toHaveLength(1);
    expect(parts?.[0]).toBe(REGULAR);
  });

  it.skipIf(HELVETICA === undefined)(
    "[F17.AC10] gives Helvetica-Bold, the second face of Helvetica.ttc, as before US-052",
    () => {
      // The second face's length and SHA-256 as `collectionFaceToFile` wrote it (see
      // parse-collection-faces.test.ts).
      const parts = parseNamedFaceParts(HELVETICA ?? new Uint8Array(), "Helvetica-Bold") ?? [];

      expect(parts.reduce((sum, part) => sum + part.length, 0)).toBe(437_820);

      expect(partsDigest(parts)).toBe(
        "1cce2b1746ac2d78a5e067040d2df303b711dd617dcfdef63b39df63d7c15379",
      );
    },
  );
});
