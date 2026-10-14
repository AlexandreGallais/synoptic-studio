import { describe, expect, it } from "vitest";

import { coverageList, rangeTable, viewOf } from "../../../tests";

import { parseCoverage } from "./parse-coverage";

describe("parseCoverage (REF-OT-LAYOUT-COMMON)", () => {
  it("reads a list of glyphs (format 1), at their coverage indexes", () => {
    const table = coverageList([3, 7, 9]);

    expect(parseCoverage(viewOf(table), 0)).toEqual([3, 7, 9]);
  });

  it("reads ranges of glyphs (format 2), each glyph of a range in turn", () => {
    // 4–6 at indexes 0 to 2, then 10–11 at 3 and 4.
    const table = rangeTable([
      { end: 6, start: 4, value: 0 },
      { end: 11, start: 10, value: 3 },
    ]);

    expect(parseCoverage(viewOf(table), 0)).toEqual([4, 5, 6, 10, 11]);
  });

  it("reads a table lying at an offset", () => {
    const file = [9, 9, ...coverageList([5])];

    expect(parseCoverage(viewOf(file), 2)).toEqual([5]);
  });
});
