import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  SHARED_SANS_TABLES,
  TRUE_TYPE_VERSION,
  WINDOWS_PLATFORM,
  fontCollection,
  fontFile,
  joinedParts,
  nameTable,
  sharedTableCollection,
  tablesFile,
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

  it("[F17.AC10] [F18.AC19] gives SharedSans-Bold, the second face of a collection sharing Inter's tables", () => {
    // The second face: 284 + 409 312 shared + 96 of OS/2 + 84 of name = 409 776 bytes.
    const faces = SHARED_SANS_TABLES;
    const collection = sharedTableCollection(TRUE_TYPE_VERSION, faces);
    const parts = parseNamedFaceParts(collection, "SharedSans-Bold") ?? [];

    expect(parts.reduce((sum, part) => sum + part.length, 0)).toBe(409_776);
    expect(joinedParts(parts)).toEqual(tablesFile(TRUE_TYPE_VERSION, faces[1] ?? {}));
  });
});
