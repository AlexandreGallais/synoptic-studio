import { describe, expect, it } from "vitest";

import {
  ARIAL_PATH,
  HELLO_TEXT,
  NO_KERNING,
  OPEN_TYPE_VERSION,
  TRUE_TYPE_VERSION,
  X_ADVANCE_ONLY,
  fontFile,
  gposTable,
  headTable,
  hheaTable,
  kernTable,
  os2Table,
  pairPosFormat1,
  pairPosFormat2,
  systemFont,
  testFontTables,
} from "../../../tests";
import { pairKerning, shownLines, textBlock } from "../../model";

import { parseFontMetrics } from "./parse-font-metrics";

/** Arial 5.01 of macOS, read where the machine has it (`DERIV-text-width`, cross-check). */
const ARIAL = systemFont(ARIAL_PATH);

/** The characters of "Hello World" and "AVATAR": H e l o space W r d, A V T R. */
const ARIAL_CHARACTERS = [72, 101, 108, 111, 32, 87, 114, 100, 65, 86, 84, 82];

/** A, B, C, a, and Z, which the test font does not map. */
const CHARACTERS = [65, 66, 67, 97, 90];

/**
 * A `GPOS` table: a `kern` feature of lookups 1 and 0, listed in that order, and a `cpsp` feature
 * of lookup 2. Lookup 0, pair adjustment: A B (glyphs 1, 2) −50. Lookup 1, an extension of a
 * pair adjustment by classes: A and B in first class 1, C (glyph 3) in second class 1, −30.
 * Lookup 2: A B +200, which is not kerning.
 */
const GPOS = gposTable(
  [
    { lookups: [2], tag: "cpsp" },
    { lookups: [1, 0], tag: "kern" },
  ],
  [
    {
      subtables: [pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }])],
      type: 2,
    },
    {
      subtables: [
        pairPosFormat2({
          covered: [1, 2],
          firstClasses: [{ end: 2, start: 1, value: 1 }],
          secondClasses: [{ end: 3, start: 3, value: 1 }],
          values: [
            [0, 0],
            [0, -30],
          ],
        }),
      ],
      type: 9,
    },
    {
      subtables: [pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, 200]] }])],
      type: 2,
    },
  ],
);

/** A `kern` table: A B −999, A C −20. */
const KERN = kernTable([
  {
    coverage: 1,
    pairs: [
      { left: 1, right: 2, value: -999 },
      { left: 1, right: 3, value: -20 },
    ],
  },
]);

describe("parseFontMetrics (REF-OT-OTFF)", () => {
  it("[F05.AC1] reads the units per em, the vertical metrics and the advances as written", () => {
    const metrics = parseFontMetrics(fontFile(TRUE_TYPE_VERSION, testFontTables()), CHARACTERS);

    expect(metrics).toEqual({
      // A → glyph 1 (600), B → glyph 2 (700); C → glyph 3 and a → glyph 4, past the three
      // hmtx records: the last record, 700. Z is not mapped: left out, drawn by glyph 0 (500).
      advances: { 65: 600, 66: 700, 67: 700, 97: 700 },
      ascender: 800,
      capHeight: 700,
      descender: -200,
      embeddingPermissions: 4,
      // No kerning table: every character in class 0.
      kerning: { leftClasses: {}, rightClasses: {}, values: [[0]] },
      lineGap: 90,
      missingAdvance: 500,
      strikeoutPosition: 300,
      strikeoutSize: 50,
      underlinePosition: -100,
      underlineThickness: 40,
      unitsPerEm: 1000,
    });
  });

  it("[F05.AC1] reads an OpenType file's kerning from the lookups of its GPOS kern feature", () => {
    const file = fontFile(OPEN_TYPE_VERSION, { ...testFontTables(), GPOS, kern: KERN });
    const kerning = parseFontMetrics(file, CHARACTERS)?.kerning ?? NO_KERNING;

    // A B: −50 in lookup 0, and 0 in lookup 1 (B in second class 0); the kern table and the
    // cpsp feature are not read.
    expect(pairKerning(kerning, 65, 66)).toBe(-50);
    // A C and B C: −30 in lookup 1, by classes; C A: nothing.
    expect(pairKerning(kerning, 65, 67)).toBe(-30);
    expect(pairKerning(kerning, 66, 67)).toBe(-30);
    expect(pairKerning(kerning, 67, 65)).toBe(0);
  });

  it("[F05.AC1] reads a TrueType file's kerning from its kern table when it has no GPOS", () => {
    const file = fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), kern: KERN });
    const kerning = parseFontMetrics(file, CHARACTERS)?.kerning ?? NO_KERNING;

    expect(pairKerning(kerning, 65, 66)).toBe(-999);
    expect(pairKerning(kerning, 65, 67)).toBe(-20);
  });

  it.skipIf(ARIAL === undefined)(
    "[F05.AC1] reads Arial's advances and kerning pairs as the derivation's cross-check",
    () => {
      const metrics = parseFontMetrics(ARIAL ?? new Uint8Array(), ARIAL_CHARACTERS);
      const kerning = metrics?.kerning ?? NO_KERNING;

      // DERIV-text-width, cross-check: read from the file's tables by the spike's script.
      expect(metrics).toMatchObject({
        advances: {
          32: 569,
          72: 1479,
          87: 1933,
          100: 1139,
          101: 1139,
          108: 455,
          111: 1139,
          114: 682,
        },
        unitsPerEm: 2048,
      });

      expect(pairKerning(kerning, 87, 111)).toBe(-37);

      // A V, V A, A T, T A: −152 each.
      expect([
        pairKerning(kerning, 65, 86),
        pairKerning(kerning, 86, 65),
        pairKerning(kerning, 65, 84),
        pairKerning(kerning, 84, 65),
      ]).toEqual([-152, -152, -152, -152]);
    },
  );

  it.skipIf(ARIAL === undefined)(
    "[F14.AC5] reads Arial's cap height from its OS/2 table of version 3",
    () => {
      // DERIV-text-box, cross-check: sCapHeight 1467.
      expect(parseFontMetrics(ARIAL ?? new Uint8Array(), [72])?.capHeight).toBe(1467);
    },
  );

  it.skipIf(ARIAL === undefined)(
    "[F05.AC3] measures 'Hello World' and 'Hello' / 'World' in Arial 16 read from its file",
    () => {
      const metrics = parseFontMetrics(ARIAL ?? new Uint8Array(), ARIAL_CHARACTERS);
      const blocks = ["Hello World", "Hello\nWorld"].map((content) =>
        metrics === undefined
          ? undefined
          : textBlock(
              metrics,
              { ...HELLO_TEXT, content },
              shownLines(metrics, { ...HELLO_TEXT, content }),
            ),
      );

      // DERIV-text-width, checks: (10 584 − 37) × 16 / 2048 = 82.3984375, one line of 18.3984375;
      // kerned "World", W o −37: 5311 × 16 / 2048 = 41.4921875, two lines.
      expect(blocks).toEqual([
        { height: 18.3984375, width: 82.3984375 },
        { height: 36.796875, width: 41.4921875 },
      ]);
    },
  );

  it("gives undefined, never an error, when a kerning table passes its end", () => {
    // A GPOS of one kern lookup, cut by one byte: its subtable's coverage passes the table's end.
    const gpos = gposTable(
      [{ lookups: [0], tag: "kern" }],
      [
        {
          subtables: [pairPosFormat1([X_ADVANCE_ONLY, 0], [{ first: 1, pairs: [[2, -50]] }])],
          type: 2,
        },
      ],
    );
    const file = fontFile(OPEN_TYPE_VERSION, { ...testFontTables(), GPOS: gpos.slice(0, -1) });

    expect(parseFontMetrics(file, CHARACTERS)).toBeUndefined();
  });

  it("reads a damaged file without an error, whatever byte is changed", () => {
    const file = fontFile(OPEN_TYPE_VERSION, { ...testFontTables(), GPOS, kern: KERN });

    // Each byte set in turn to 0x00, 0x80 and 0xFF: the reader gives metrics or undefined.
    for (const [index] of file.entries()) {
      for (const value of [0x00, 0x80, 0xff]) {
        const damaged = Uint8Array.from(file);

        damaged[index] = value;

        expect(() => parseFontMetrics(damaged, CHARACTERS)).not.toThrow();
      }
    }
  });

  it("reads the signed fields as signed: a line gap of −1, a strike-out at −20", () => {
    const file = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      hhea: hheaTable({ ascender: 800, descender: -200, lineGap: -1, numberOfHMetrics: 3 }),
      "OS/2": os2Table({
        capHeight: 700,
        fsType: 0,
        strikeoutPosition: -20,
        strikeoutSize: 50,
        version: 4,
      }),
    });

    expect(parseFontMetrics(file, [65])).toMatchObject({ lineGap: -1, strikeoutPosition: -20 });
  });

  it("gives undefined for units per em outside 16 to 16 384 (REF-OT-HEAD)", () => {
    // 0 would measure every text as Infinity, or NaN for an empty line: 10 584 × 16 / 0.
    const metrics = [0, 15, 16, 16_384, 16_385].map((unitsPerEm) =>
      parseFontMetrics(
        fontFile(TRUE_TYPE_VERSION, { ...testFontTables(), head: headTable(unitsPerEm) }),
        [65],
      ),
    );

    expect(metrics.map((each) => each?.unitsPerEm)).toEqual([
      undefined,
      undefined,
      16,
      16_384,
      undefined,
    ]);
  });

  it("gives undefined for a font without any advance record (REF-OT-HMTX)", () => {
    // numberOfHMetrics 0: no record whose advance the remaining glyphs take, every text 0 wide.
    const file = fontFile(TRUE_TYPE_VERSION, {
      ...testFontTables(),
      hhea: hheaTable({ ascender: 800, descender: -200, lineGap: 90, numberOfHMetrics: 0 }),
    });

    expect(parseFontMetrics(file, [65])).toBeUndefined();
  });

  it("reads a file lying inside a larger buffer", () => {
    const file = fontFile(TRUE_TYPE_VERSION, testFontTables());
    const buffer = new Uint8Array(file.length + 8);

    buffer.set(file, 8);

    expect(parseFontMetrics(buffer.subarray(8), [66])?.advances).toEqual({ 66: 700 });
  });

  it("gives undefined for a file that is not a TrueType or OpenType font", () => {
    // A TrueType collection starts with the tag `ttcf`, 0x74746366.
    const collection = fontFile(0x74_74_63_66, testFontTables());

    expect(parseFontMetrics(collection, [65])).toBeUndefined();
    expect(parseFontMetrics(new Uint8Array([0, 1, 0, 0]), [65])).toBeUndefined();
  });
});
