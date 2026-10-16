import { describe, expect, it } from "vitest";

import {
  ENGLISH_US,
  HELVETICA_COLLECTION_PATH,
  TRUE_TYPE_VERSION,
  WINDOWS_PLATFORM,
  familyNameTable,
  fontFile,
  nameTable,
  systemFont,
  testFontTables,
} from "../../../tests";
import { parseCollectionOffsets } from "../font-tables";

import { parsePostScriptName } from "./parse-post-script-name";

/** Helvetica of macOS, a collection of 6 faces, read where the machine has it. */
const HELVETICA = systemFont(HELVETICA_COLLECTION_PATH);

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

  it.skipIf(HELVETICA === undefined)(
    "[F17.AC10] names each face of Helvetica.ttc, read in the collection",
    () => {
      // Each face read at its directory, 48, 380, 696, 1028, 1344 and 1644 (US-046), no face's
      // file written (US-052).
      const file = HELVETICA ?? new Uint8Array();
      const offsets = parseCollectionOffsets(
        new DataView(file.buffer, file.byteOffset, file.byteLength),
      );

      expect(offsets).toEqual([48, 380, 696, 1028, 1344, 1644]);

      expect(offsets.map((offset) => parsePostScriptName(file, offset))).toEqual([
        "Helvetica",
        "Helvetica-Bold",
        "Helvetica-Oblique",
        "Helvetica-BoldOblique",
        "Helvetica-Light",
        "Helvetica-LightOblique",
      ]);
    },
  );

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
