import { describe, expect, it } from "vitest";

import { rangeTable, uint16, viewOf, viewOfFirst } from "../../../tests";

import { parseClassDefinition } from "./parse-class-definition";

/** A class array: start glyph 10, three classes, 10 → 2, 11 → 0, 12 → 1. */
const CLASS_ARRAY = [
  ...uint16(1),
  ...uint16(10),
  ...uint16(3),
  ...uint16(2),
  ...uint16(0),
  ...uint16(1),
];

describe("parseClassDefinition (REF-OT-LAYOUT-COMMON)", () => {
  it("reads a class array (format 1): the class of each glyph asked from the start glyph", () => {
    // 9 and 13 lie outside the array: left out, in class 0.
    expect(parseClassDefinition(viewOf(CLASS_ARRAY), 0, new Set([9, 10, 12, 13]))).toEqual({
      10: 2,
      12: 1,
    });
  });

  it("looks the glyphs asked up in class ranges (format 2)", () => {
    const table = rangeTable([
      { end: 5, start: 4, value: 1 },
      { end: 8, start: 8, value: 3 },
    ]);

    expect(parseClassDefinition(viewOf(table), 0, new Set([5, 6, 8]))).toEqual({ 5: 1, 8: 3 });
  });

  it("reads a range of every glyph, 0 to 65 535, without spreading it out", () => {
    const table = rangeTable([{ end: 0xff_ff, start: 0, value: 7 }]);

    expect(parseClassDefinition(viewOf(table), 0, new Set([0, 0xff_ff]))).toEqual({
      0: 7,
      65_535: 7,
    });
  });

  it("gives undefined for a table passing the end, or of another format", () => {
    // The array cut in its values; its header cut; ranges cut; format 3.
    expect(parseClassDefinition(viewOfFirst(CLASS_ARRAY, 10), 0, new Set([10]))).toBe(undefined);

    expect(parseClassDefinition(viewOfFirst(CLASS_ARRAY, 5), 0, new Set([10]))).toBe(undefined);

    const ranges = rangeTable([{ end: 5, start: 4, value: 1 }]);
    const otherFormat = [...uint16(3), ...uint16(0)];

    expect(parseClassDefinition(viewOfFirst(ranges, 8), 0, new Set([4]))).toBeUndefined();
    expect(parseClassDefinition(viewOf(otherFormat), 0, new Set([4]))).toBeUndefined();
  });
});
