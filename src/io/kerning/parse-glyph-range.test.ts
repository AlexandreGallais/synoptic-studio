import { describe, expect, it } from "vitest";

import { rangeTable, viewOf } from "../../../tests";

import { parseGlyphRange } from "./parse-glyph-range";

/** Ranges 4–6 and 10–11, records at 4 and 10. */
const TABLE = viewOf(
  rangeTable([
    { end: 6, start: 4, value: 0 },
    { end: 11, start: 10, value: 3 },
  ]),
);

describe("parseGlyphRange (REF-OT-LAYOUT-COMMON)", () => {
  it.each([
    // Each range's start and end are in it.
    { expected: 4, glyph: 4 },
    { expected: 4, glyph: 6 },
    { expected: 10, glyph: 10 },
    { expected: 10, glyph: 11 },
    // Before, between and after the ranges.
    { expected: undefined, glyph: 3 },
    { expected: undefined, glyph: 7 },
    { expected: undefined, glyph: 12 },
  ])("finds the range of glyph $glyph at $expected", ({ expected, glyph }) => {
    expect(parseGlyphRange(TABLE, [4, 10], glyph)).toBe(expected);
  });
});
