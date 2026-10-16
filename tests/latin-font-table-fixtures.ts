// The test font of `testFontTables` with every letter of the ISO basic Latin alphabet mapped (US-056).

import {
  TEST_FONT,
  cmapFormat4,
  cmapTable,
  hheaTable,
  hmtxTable,
  maxpTable,
  testFontTables,
} from "./font-table-fixtures";

import type { CmapSegment } from "./font-table-fixtures";

/** Glyph 5 of the Latin test font: 500 units wide, as `.notdef`, so that the letters it maps measure as before. */
const LATIN_REST_GLYPH = 5;

/** D, the first uppercase letter past A, B and C, which map to glyphs of their own. */
const UPPERCASE_REST_START = 0x44;

/** Z, the last uppercase letter (`REF-WIKI-ISO-BASIC-LATIN`). */
const UPPERCASE_END = 0x5a;

/** b, the first lowercase letter past `a`, which maps to a glyph of its own. */
const LOWERCASE_REST_START = 0x62;

/** z, the last lowercase letter (`REF-WIKI-ISO-BASIC-LATIN`). */
const LOWERCASE_END = 0x7a;

/**
 * A `cmap` format 4 segment mapping every character from one code to another to glyph 5.
 *
 * @param start - first character code
 * @param end - last character code
 * @returns a segment of delta 0 whose glyph array names glyph 5 once per character
 */
function restSegment(start: number, end: number): CmapSegment {
  return {
    delta: 0,
    end,
    glyphs: Array.from({ length: end - start + 1 }, () => LATIN_REST_GLYPH),
    start,
  };
}

/**
 * The tables of the test font of `testFontTables` with every letter of the ISO basic Latin
 * alphabet mapped (U+0041 to U+005A, U+0061 to U+007A, `REF-WIKI-ISO-BASIC-LATIN`), as an installed
 * font must map them to be listed (US-056): A, B, C and `a` as before, every other letter to a
 * sixth glyph 500 units wide, the advance of `.notdef`, so that every text measures as with
 * `testFontTables`.
 *
 * @returns the tables by tag
 */
export function latinTestFontTables(): Record<string, readonly number[]> {
  const advances = [
    TEST_FONT.notdefAdvance,
    TEST_FONT.firstAdvance,
    TEST_FONT.lastAdvance,
    TEST_FONT.lastAdvance,
    TEST_FONT.lastAdvance,
    TEST_FONT.notdefAdvance,
  ];

  return {
    ...testFontTables(),
    cmap: cmapTable([
      {
        encoding: 1,
        platform: 3,
        subtable: cmapFormat4([
          { delta: -64, end: 67, glyphs: [], start: 65 },
          restSegment(UPPERCASE_REST_START, UPPERCASE_END),
          { delta: 0, end: 97, glyphs: [TEST_FONT.glyphCount - 1], start: 97 },
          restSegment(LOWERCASE_REST_START, LOWERCASE_END),
          { delta: 1, end: 0xff_ff, glyphs: [], start: 0xff_ff },
        ]),
      },
    ]),
    hhea: hheaTable({ ...TEST_FONT, numberOfHMetrics: advances.length }),
    hmtx: hmtxTable(advances, 0),
    maxp: maxpTable(advances.length),
  };
}
