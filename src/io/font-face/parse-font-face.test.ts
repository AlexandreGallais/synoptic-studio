import { describe, expect, it } from "vitest";

import {
  APPLE_TRUE_TYPE_VERSION,
  ENGLISH_US,
  HELLO_TEXT,
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  WINDOWS_PLATFORM,
  familyNameTable,
  fontFile,
  headTable,
  interRegularFile,
  joinedParts,
  nameTable,
  os2FaceTable,
  sharedSansTables,
  sharedTableCollection,
  testFontTables,
  uint32,
} from "../../../tests";
import {
  DEFAULT_FONT,
  MEASURED_BLOCKS,
  blockCharacters,
  fontsWithFace,
  shownLines,
  textBlock,
  textFontMetrics,
} from "../../model";
import { parseCollectionFaces } from "../font-collection";
import { parseFontMetrics } from "../font-file";

import { parseFontFace } from "./parse-font-face";

import type { Os2FaceFields } from "../../../tests";
import type { FontFamily } from "../../model";

/** The faces of a collection of three faces sharing Inter's tables, each joined into its file. */
const SHARED_SANS_FILES = parseCollectionFaces(
  sharedTableCollection(TRUE_TYPE_VERSION, sharedSansTables()),
).map((parts) => joinedParts(parts));

/**
 * A font file tagged `true`, as Chrome gives on macOS a face whose CoreText format is TrueType
 * (`REF-SKIA-MAC-STREAM-CODE`).
 *
 * @param file - the bytes of a font file
 * @returns a copy whose first four bytes are `true`
 */
function appleTagged(file: Readonly<Uint8Array>): Uint8Array {
  return Uint8Array.from([...uint32(APPLE_TRUE_TYPE_VERSION), ...file.subarray(4)]);
}

/** The `OS/2` table of the test font as Arial's: version 3, `fsType` 8, 400, upright. */
const ARIAL_OS2: Os2FaceFields = {
  capHeight: 700,
  fsSelection: 0x00_40,
  fsType: 8,
  strikeoutPosition: 300,
  strikeoutSize: 50,
  version: 3,
  weightClass: 400,
};

/**
 * The test font of `[F05.AC1]` named and styled by a `name` and an `OS/2` table.
 *
 * @param name - the bytes of its `name` table
 * @param os2 - its `OS/2` fields
 * @returns the file's bytes
 */
function namedFont(name: readonly number[], os2: Os2FaceFields): Uint8Array {
  return fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), name, "OS/2": os2FaceTable(os2) });
}

/** The test font named Arial, 400 upright, `fsType` 8. */
const ARIAL_FILE = namedFont(familyNameTable("Arial"), ARIAL_OS2);

describe("parseFontFace (REF-OT-NAME, REF-OT-OS2)", () => {
  it("[F15.AC1] reads a face named Arial, 400, upright, with its metrics and its file", () => {
    expect(parseFontFace(ARIAL_FILE)).toEqual({
      face: {
        // The test font has no name ID 5: no version.
        file: { bytes: ARIAL_FILE, isEmbeddable: true, version: "" },
        isItalic: false,
        // The metrics F05 reads, for the characters of the measured blocks.
        metrics: parseFontMetrics(ARIAL_FILE, blockCharacters(MEASURED_BLOCKS)),
        weight: 400,
      },
      family: "Arial",
    });
  });

  it("[F15.AC1] keeps the very bytes it read as the face's file", () => {
    expect(parseFontFace(ARIAL_FILE)?.face.file?.bytes).toBe(ARIAL_FILE);
  });

  it("[F15.AC1] names Inter by its name ID 16, its weight 300, italic from fsSelection 1", () => {
    const name = nameTable([
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 1,
        platform: WINDOWS_PLATFORM,
        text: "Inter Light",
      },
      { encoding: 1, language: ENGLISH_US, nameId: 16, platform: WINDOWS_PLATFORM, text: "Inter" },
    ]);
    const file = namedFont(name, { ...ARIAL_OS2, fsSelection: 1, fsType: 0, weightClass: 300 });

    expect(parseFontFace(file)).toMatchObject({
      face: { file: { isEmbeddable: true }, isItalic: true, weight: 300 },
      family: "Inter",
    });
  });

  it("[F15.AC1] keeps the version its file names, name ID 5, read once with the family", () => {
    const name = nameTable([
      { encoding: 1, language: ENGLISH_US, nameId: 1, platform: WINDOWS_PLATFORM, text: "Arial" },
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 5,
        platform: WINDOWS_PLATFORM,
        text: "Version 5.01x",
      },
    ]);

    expect(parseFontFace(namedFont(name, ARIAL_OS2))?.face.file?.version).toBe("Version 5.01x");
  });

  it.each([
    { case: "2", fsType: 2, isEmbeddable: false, version: 3 },
    { case: "0x0208, version 4", fsType: 0x02_08, isEmbeddable: false, version: 4 },
    { case: "0x0208, version 1", fsType: 0x02_08, isEmbeddable: true, version: 1 },
  ])("[F15.AC2] reads fsType $case: embeddable $isEmbeddable", (row) => {
    const file = namedFont(familyNameTable("Arial"), { ...ARIAL_OS2, ...row });

    expect(parseFontFace(file)?.face.file?.isEmbeddable).toBe(row.isEmbeddable);
  });

  it.each([
    // The tags of a TrueType collection, read whole (its faces are read with
    // parseCollectionFaces), a WOFF and a WOFF2 file.
    {
      case: "ttcf",
      file: fontFile(0x74_74_63_66, { ...testFontTables(), name: familyNameTable("A") }),
    },
    {
      case: "wOFF",
      file: fontFile(0x77_4f_46_46, { ...testFontTables(), name: familyNameTable("A") }),
    },
    {
      case: "wOF2",
      file: fontFile(0x77_4f_46_32, { ...testFontTables(), name: familyNameTable("A") }),
    },
    { case: "an empty file", file: new Uint8Array() },
  ])("[F15.AC1] gives no face, never an error, for $case", ({ file }) => {
    expect(parseFontFace(file)).toBeUndefined();
  });

  it("gives no face for a font that F05 cannot measure: units per em 0", () => {
    const file = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      head: headTable(0),
      name: familyNameTable("Arial"),
      "OS/2": os2FaceTable(ARIAL_OS2),
    });

    expect(parseFontFace(file)).toBeUndefined();
  });

  it("gives no face for a font without a name table, or naming no family", () => {
    const unnamed = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      "OS/2": os2FaceTable(ARIAL_OS2),
    });
    const subfamilyOnly = nameTable([
      { encoding: 1, language: ENGLISH_US, nameId: 2, platform: WINDOWS_PLATFORM, text: "Regular" },
    ]);

    expect(parseFontFace(unnamed)).toBeUndefined();
    expect(parseFontFace(namedFont(subfamilyOnly, ARIAL_OS2))).toBeUndefined();
  });

  it("gives no face for an OS/2 table too short to hold fsSelection", () => {
    // 40 bytes: enough for F05's fields (30), not for fsSelection at 62.
    const file = fontFile(OPEN_TYPE_VERSION, {
      ...testFontTables(),
      name: familyNameTable("Arial"),
      "OS/2": os2FaceTable(ARIAL_OS2).slice(0, 40),
    });

    expect(parseFontFace(file)).toBeUndefined();
  });

  it("[F15.AC1] measures a text in arial 700 with the face of Arial 700 read from its file", () => {
    // Arial 400 and Arial 700, told apart by their units per em, 1000 and 2000.
    const bold = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      head: headTable(2000),
      name: familyNameTable("Arial"),
      "OS/2": os2FaceTable({ ...ARIAL_OS2, weightClass: 700 }),
    });
    const faces = [parseFontFace(ARIAL_FILE), parseFontFace(bold)];
    const fonts = faces.reduce<readonly FontFamily[]>(
      (kept, named) => (named === undefined ? kept : fontsWithFace(kept, named)),
      [DEFAULT_FONT],
    );
    const metrics = textFontMetrics({ ...HELLO_TEXT, font: "arial", weight: 700 }, fonts);

    expect(metrics).toBe(faces[1]?.face.metrics);
    expect(metrics?.unitsPerEm).toBe(2000);
  });

  it("[F15.AC1] reads Inter Regular 4.1: Inter, 400, upright, embeddable", () => {
    // REF-OT-NAME: no name ID 16, name ID 1 Inter; REF-OT-OS2: 400, fsSelection 0x00c0 (bit 6
    // regular), fsType 0, installable; read again with fontTools 4.60.2
    // (`REF-INTER-REGULAR-FILE`).
    const named = parseFontFace(interRegularFile());

    expect(named).toMatchObject({
      face: { file: { isEmbeddable: true }, isItalic: false, weight: 400 },
      family: "Inter",
    });

    expect(named?.face.metrics.unitsPerEm).toBe(2048);
  });
});

describe("parseFontFace, faces of macOS and of collections (F17.AC4)", () => {
  it("[F17.AC4] reads a face whose sfntVersion is true as TrueType (REF-APPLE-TT-SCALER)", () => {
    const tagged = appleTagged(ARIAL_FILE);

    expect(parseFontFace(tagged)).toEqual({
      face: {
        file: { bytes: tagged, isEmbeddable: true, version: "" },
        isItalic: false,
        metrics: parseFontMetrics(ARIAL_FILE, blockCharacters(MEASURED_BLOCKS)),
        weight: 400,
      },
      family: "Arial",
    });
  });

  it(
    "[F17.AC4] [F18.AC19] reads the 3 faces of a collection sharing Inter's tables: 400, 700, 400 italic",
    // Inter's metrics read three times, about 2.3 s alone: 20 s holds a loaded pre-push run.
    { timeout: 20_000 },
    () => {
      // usWeightClass 400, 700, 400; fsSelection 0x00c0, 0x00a0, 0x0081: bit 0, italic, in the third.
      expect(
        SHARED_SANS_FILES.map((file) => {
          const named = parseFontFace(file);

          return [named?.family, named?.face.weight, named?.face.isItalic];
        }),
      ).toEqual([
        ["Shared Sans", 400, false],
        ["Shared Sans", 700, false],
        ["Shared Sans", 400, true],
      ]);
    },
  );

  it(
    "[F17.AC4] [F18.AC19] reads each face of that collection tagged true, as Chrome gives it on macOS",
    // Inter's metrics read three times, about 2.3 s alone: 20 s holds a loaded pre-push run.
    { timeout: 20_000 },
    () => {
      const faces = SHARED_SANS_FILES.map((file) => parseFontFace(appleTagged(file)));

      expect(faces.map((named) => named?.family)).toEqual([
        "Shared Sans",
        "Shared Sans",
        "Shared Sans",
      ]);
    },
  );
});

describe("parseFontFace, measuring with its face (F15.AC4)", () => {
  /** The face read from the file, once, before the measure is timed. */
  const metrics = parseFontFace(ARIAL_FILE)?.face.metrics;

  /** A thousand texts of two lines, each its own content. */
  const texts = Array.from({ length: 1000 }, (unused, index) => ({
    ...HELLO_TEXT,
    content: `ABC ${String(index)}\naBC`,
  }));

  // This synthetic file's face measures them in about 7 ms alone (EN-054, 2026-10-16); a loaded
  // pre-push run was seen at 157 ms (RET-003): 400 ms, Vitest's timeout, holds over that load and
  // is over four times the measure alone. No clock is read in the test.
  it(
    "[F15.AC4] [F18.AC10] measures the blocks of 1 000 texts of two lines in under 0.4 s",
    { timeout: 400 },
    () => {
      const blocks = texts.map((text) =>
        metrics === undefined ? undefined : textBlock(metrics, text, shownLines(metrics, text)),
      );

      // Two lines of the automatic line height: (800 + 200 + 90) / 1000 × 16 = 17.44 each, 34.88.
      expect(blocks).toHaveLength(1000);

      expect(
        blocks.every((block) => block !== undefined && Math.abs(block.height - 34.88) < 1e-9),
      ).toBe(true);
    },
  );
});
