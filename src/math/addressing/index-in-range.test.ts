import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { indexInRange } from "./index-in-range";

describe("indexInRange (REF-OT-LAYOUT-COMMON)", () => {
  it.each([
    // Glyphs 40 to 45 with startCoverageIndex 7: glyph 42 is 7 + (42 − 40) = 9.
    { expected: 9, rangeStart: 40, startIndex: 7, value: 42 },
    // The range's first value has its start index.
    { expected: 7, rangeStart: 40, startIndex: 7, value: 40 },
    // A cmap group from U+0041 to glyph 36: U+0043 maps to 36 + 2 = 38.
    { expected: 38, rangeStart: 0x41, startIndex: 36, value: 0x43 },
    // A range indexed from 0: the offset from its first value.
    { expected: 3, rangeStart: 10, startIndex: 0, value: 13 },
  ])("gives $value index $expected", ({ expected, rangeStart, startIndex, value }) => {
    expect(indexInRange(value, rangeStart, startIndex)).toBe(expected);
  });

  test.prop({
    rangeStart: fc.integer({ max: 65_535, min: 0 }),
    startIndex: fc.integer({ max: 65_535, min: 0 }),
    step: fc.integer({ max: 1000, min: 0 }),
  })("increases one by one along the range", ({ rangeStart, startIndex, step }) => {
    const value = rangeStart + step;

    expect(indexInRange(value + 1, rangeStart, startIndex)).toBe(
      indexInRange(value, rangeStart, startIndex) + 1,
    );

    expect(indexInRange(rangeStart, rangeStart, startIndex)).toBe(startIndex);
  });
});
