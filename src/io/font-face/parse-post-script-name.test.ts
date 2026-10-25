import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  TRUE_TYPE_VERSION,
  WINDOWS_PLATFORM,
  familyNameTable,
  fontFile,
  nameTable,
  sharedSansTables,
  sharedTableCollection,
  testFontTables,
} from "../../../tests";
import { parseCollectionOffsets } from "../font-collection";

import { parsePostScriptName } from "./parse-post-script-name";

describe("parsePostScriptName (REF-OT-NAME)", () => {
  it("[F17.AC4] reads name ID 6, not the family", () => {
    const name = nameTable([
      { encoding: 1, language: ENGLISH_US, nameId: 1, platform: WINDOWS_PLATFORM, text: "Arial" },
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 6,
        platform: WINDOWS_PLATFORM,
        text: "Arial-BoldMT",
      },
    ]);

    const file = fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), name });

    expect(parsePostScriptName(file, 0)).toBe("Arial-BoldMT");
  });

  it("[F17.AC10] [F18.AC19] names each face of a collection sharing Inter's tables, read in the collection", () => {
    // Each face read at its directory, 24, 308 and 592 (12 + 3 × 4, then 284 bytes each), no
    // face's file written.
    const file = sharedTableCollection(TRUE_TYPE_VERSION, sharedSansTables());
    const offsets = parseCollectionOffsets(new DataView(file.buffer));

    expect(offsets).toEqual([24, 308, 592]);

    expect(offsets.map((offset) => parsePostScriptName(file, offset))).toEqual([
      "SharedSans-Regular",
      "SharedSans-Bold",
      "SharedSans-Italic",
    ]);
  });

  it.each([
    {
      case: "a name table without name ID 6",
      file: fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), name: familyNameTable("Arial") }),
    },
    { case: "no name table", file: fontFile(TRUE_TYPE_VERSION, testFontTables()) },
    { case: "a file that is not a font", file: Uint8Array.from([1, 2, 3]) },
  ])("reads no name for $case", ({ file }) => {
    expect(parsePostScriptName(file, 0)).toBeUndefined();
  });
});
