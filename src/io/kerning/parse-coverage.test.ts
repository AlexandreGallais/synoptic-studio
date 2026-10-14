import { describe, expect, it } from "vitest";

import { coverageList, rangeTable, uint16, viewOf, viewOfFirst } from "../../../tests";

import { parseCoverage } from "./parse-coverage";

describe("parseCoverage (REF-OT-LAYOUT-COMMON)", () => {
  it("reads a list of glyphs (format 1), the glyphs asked at their coverage indexes", () => {
    const table = coverageList([3, 7, 9]);

    // 3 at index 0, 9 at index 2; 7 was not asked, 10 is not covered.
    expect(parseCoverage(viewOf(table), 0, new Set([3, 9, 10]))).toEqual({ 3: 0, 9: 2 });
  });

  it("looks the glyphs asked up in ranges (format 2), indexes running on from each range's start", () => {
    // 4–6 at indexes 0 to 2, then 10–11 at 3 and 4: 5 → 1, 11 → 3 + 11 − 10 = 4; 7 is in no range.
    const table = rangeTable([
      { end: 6, start: 4, value: 0 },
      { end: 11, start: 10, value: 3 },
    ]);

    expect(parseCoverage(viewOf(table), 0, new Set([5, 7, 11]))).toEqual({ 5: 1, 11: 4 });
  });

  it("reads a range of every glyph, 0 to 65 535, without spreading it out", () => {
    const table = rangeTable([{ end: 0xff_ff, start: 0, value: 0 }]);

    expect(parseCoverage(viewOf(table), 0, new Set([0, 0xff_ff]))).toEqual({
      0: 0,
      65_535: 65_535,
    });
  });

  it("reads a table lying at an offset", () => {
    const file = [9, 9, ...coverageList([5])];

    expect(parseCoverage(viewOf(file), 2, new Set([5]))).toEqual({ 5: 0 });
  });

  it("gives undefined for a table passing the end, or of another format", () => {
    // A list of 3 glyphs cut after 2; ranges cut in their record; a header cut; format 3.
    const list = coverageList([3, 7, 9]);
    const ranges = rangeTable([{ end: 6, start: 4, value: 0 }]);
    const otherFormat = [...uint16(3), ...uint16(0)];

    expect(parseCoverage(viewOfFirst(list, 8), 0, new Set([3]))).toBeUndefined();
    expect(parseCoverage(viewOfFirst(ranges, 8), 0, new Set([5]))).toBeUndefined();
    expect(parseCoverage(viewOf([0, 1, 0]), 0, new Set([5]))).toBeUndefined();
    expect(parseCoverage(viewOf(otherFormat), 0, new Set([5]))).toBeUndefined();
  });
});
