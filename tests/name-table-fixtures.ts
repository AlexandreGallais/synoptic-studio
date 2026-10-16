// The `name` table and the `OS/2` fields of a face built byte by byte for the tests of
// `src/io/font-file/` (EN-039), each field at its offset in the OpenType specification. Tests
// only: never part of the library.

import { lineCodePoints } from "../src";

import { TRUE_TYPE_VERSION, fontFile, tableOf, uint16 } from "./font-fixtures";
import { testFontTables } from "./font-table-fixtures";
import { latinTestFontTables } from "./latin-font-table-fixtures";
import { os2Table } from "./metrics-table-fixtures";

import type { Os2Fields } from "./metrics-table-fixtures";

/** Sizes of the `name` table: its header and one name record (`REF-OT-NAME`). */
const NAME_LAYOUT = { headerSize: 6, recordSize: 12 };

/** Platform 3, Windows, whose strings are UTF-16BE (`REF-OT-NAME`). */
export const WINDOWS_PLATFORM = 3;

/** Platform 1, Macintosh. */
export const MACINTOSH_PLATFORM = 1;

/** Language 0x0409 of platform 3, English (United States) (`REF-OT-NAME`). */
export const ENGLISH_US = 0x04_09;

/** One name record of a `name` table and its string. */
export type NameEntry = {
  /** Platform: 0 Unicode, 1 Macintosh, 3 Windows. */
  readonly platform: number;
  /** Encoding of the platform: 1 Unicode BMP for Windows, 0 Roman for Macintosh. */
  readonly encoding: number;
  /** Language of the platform: 0x0409 English (United States) for Windows. */
  readonly language: number;
  /** Name ID: 1 family, 2 subfamily, 16 typographic family. */
  readonly nameId: number;
  /** The string: one byte per character for Macintosh, UTF-16BE otherwise. */
  readonly text: string;
};

/** The first code point past the Basic Multilingual Plane, written as a surrogate pair. */
const SUPPLEMENTARY_START = 0x1_00_00;

/** The span of the low surrogates, 0xDC00 to 0xDFFF. */
const SURROGATE_SPAN = 0x4_00;

/** The first high surrogate. */
const HIGH_SURROGATE = 0xd8_00;

/** The first low surrogate. */
const LOW_SURROGATE = 0xdc_00;

/**
 * The UTF-16 code units of a code point: itself in the Basic Multilingual Plane, else its high
 * and low surrogates (Unicode, §3.9).
 *
 * @param codePoint - from 0 to 0x10FFFF
 * @returns its one or two code units
 */
function utf16Units(codePoint: number): number[] {
  const offset = codePoint - SUPPLEMENTARY_START;

  return offset < 0
    ? [codePoint]
    : [
        HIGH_SURROGATE + Math.floor(offset / SURROGATE_SPAN),
        LOW_SURROGATE + (offset % SURROGATE_SPAN),
      ];
}

/**
 * The bytes of a name's string: one per character for Macintosh; two per UTF-16 code unit,
 * big-endian, for another platform.
 *
 * @param entry - the name record and its string
 * @returns its bytes
 */
function nameBytes(entry: NameEntry): number[] {
  const units = lineCodePoints(entry.text).flatMap((codePoint) => utf16Units(codePoint));

  return entry.platform === MACINTOSH_PLATFORM ? units : units.flatMap((unit) => uint16(unit));
}

/**
 * A `name` table of version 0: its header (`version`, `count`, `storageOffset`), its name records
 * (`platformID`, `encodingID`, `languageID`, `nameID`, `length`, `stringOffset`), then its
 * strings in the same order (`REF-OT-NAME`).
 *
 * @param entries - the names
 * @returns its bytes
 */
export function nameTable(entries: readonly NameEntry[]): number[] {
  const strings = entries.map((entry) => nameBytes(entry));
  const storage = NAME_LAYOUT.headerSize + NAME_LAYOUT.recordSize * entries.length;
  const offsets = strings.map((unused, index) =>
    strings.slice(0, index).reduce((sum, bytes) => sum + bytes.length, 0),
  );

  return [
    ...uint16(0),
    ...uint16(entries.length),
    ...uint16(storage),
    ...entries.flatMap((entry, index) => [
      ...uint16(entry.platform),
      ...uint16(entry.encoding),
      ...uint16(entry.language),
      ...uint16(entry.nameId),
      ...uint16(strings[index]?.length ?? 0),
      ...uint16(offsets[index] ?? 0),
    ]),
    ...strings.flat(),
  ];
}

/**
 * A `name` table naming a Windows family in English (United States): name ID 1 only.
 *
 * @param family - how the table names the family
 * @returns its bytes
 */
export function familyNameTable(family: string): number[] {
  return nameTable([
    { encoding: 1, language: ENGLISH_US, nameId: 1, platform: WINDOWS_PLATFORM, text: family },
  ]);
}

/** The `OS/2` fields written by `os2FaceTable`. */
export type Os2FaceFields = Os2Fields & {
  /** Weight, `usWeightClass`: 1 to 1000. */
  readonly weightClass: number;
  /** `fsSelection` flags: bit 0 italic, bit 6 regular. */
  readonly fsSelection: number;
};

/**
 * An `OS/2` table 96 bytes long as `os2Table` writes it, with `usWeightClass` at 4 and
 * `fsSelection` at 62 (`REF-OT-OS2`).
 *
 * @param fields - the values to write
 * @returns its bytes
 */
export function os2FaceTable(fields: Os2FaceFields): number[] {
  return tableOf({
    fields: [
      { at: 0, bytes: os2Table(fields) },
      { at: 4, bytes: uint16(fields.weightClass) },
      { at: 62, bytes: uint16(fields.fsSelection) },
    ],
    size: 96,
  });
}

/** `fsSelection` with bit 0 set, italic. */
const ITALIC_SELECTION = 0x00_01;

/** `fsSelection` with bit 6 set, regular. */
const REGULAR_SELECTION = 0x00_40;

/** A face of the test font, as a playground test imports it (US-036). */
export type TestFace = {
  /** How its `name` table names its family. */
  readonly family: string;
  /** Its `usWeightClass`. */
  readonly weightClass: number;
  /** Whether `fsSelection` bit 0, italic, is set. */
  readonly isItalic: boolean;
  /** Its embedding permissions: 0 installable, 2 restricted. */
  readonly fsType: number;
};

/**
 * The `OS/2` table of a face of the test font.
 *
 * @param face - its weight, style and permissions
 * @returns its bytes
 */
function testFaceOs2(face: TestFace): number[] {
  return os2FaceTable({
    capHeight: 700,
    fsSelection: face.isItalic ? ITALIC_SELECTION : REGULAR_SELECTION,
    fsType: face.fsType,
    strikeoutPosition: 300,
    strikeoutSize: 50,
    version: 4,
    weightClass: face.weightClass,
  });
}

/**
 * The test font of `[F05.AC1]` as a whole file, every letter of the ISO basic Latin alphabet
 * mapped (`latinTestFontTables`), named by the `name` table given.
 *
 * @param face - its weight, style and permissions
 * @param name - the bytes of its `name` table
 * @returns the file's bytes
 */
function testFaceNamed(face: TestFace, name: readonly number[]): Uint8Array {
  return fontFile(TRUE_TYPE_VERSION, {
    ...latinTestFontTables(),
    name,
    "OS/2": testFaceOs2(face),
  });
}

/**
 * The test font of `testFaceFile` mapping A, B, C and `a` alone (`testFontTables`), as a font
 * lacking the Latin letters is, such as Apple Color Emoji (US-056).
 *
 * @param face - its family, weight, style and permissions
 * @returns the file's bytes
 */
export function lettersMissingFaceFile(face: TestFace): Uint8Array {
  return fontFile(TRUE_TYPE_VERSION, {
    ...testFontTables(),
    name: familyNameTable(face.family),
    "OS/2": testFaceOs2(face),
  });
}

/**
 * The test font of `[F05.AC1]` as a whole file (1000 units per em, ascender 800, descender −200,
 * line gap 90; A 600 units wide, B, C and a 700, any other character 500, every Latin letter
 * mapped; no kerning), named,
 * weighted and styled by a `name` and an `OS/2` table, for the playground's tests (US-036).
 *
 * @param face - its family, weight, style and permissions
 * @returns the file's bytes
 */
export function testFaceFile(face: TestFace): Uint8Array {
  return testFaceNamed(face, familyNameTable(face.family));
}

/**
 * The test font of `testFaceFile` with a PostScript name too (name ID 6), as an installed face is
 * found in its collection (US-046).
 *
 * @param face - its family, weight, style and permissions
 * @param postScriptName - its name ID 6, such as `TestSans-Bold`
 * @returns the file's bytes
 */
export function postScriptFaceFile(face: TestFace, postScriptName: string): Uint8Array {
  return testFaceNamed(
    face,
    nameTable([
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 1,
        platform: WINDOWS_PLATFORM,
        text: face.family,
      },
      {
        encoding: 1,
        language: ENGLISH_US,
        nameId: 6,
        platform: WINDOWS_PLATFORM,
        text: postScriptName,
      },
    ]),
  );
}
