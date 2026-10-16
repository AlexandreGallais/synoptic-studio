import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rowMajorOffset } from "./row-major-offset";

describe("rowMajorOffset (REF-WIKI-ROW-MAJOR)", () => {
  it.each([
    // The source's 3 × 3 matrix 1 … 9 in row-major order: 1 2 3 4 5 6 7 8 9; the element 6, row
    // 1 column 2, is at 3 · 1 + 2 = 5; the element 7, row 2 column 0, at 6.
    { column: 2, columnCount: 3, expected: 5, row: 1 },
    { column: 0, columnCount: 3, expected: 6, row: 2 },
    { column: 0, columnCount: 3, expected: 0, row: 0 },
    // A pair of glyphs 3 and 7 among 65 536 glyph ids: 3 · 65 536 + 7 = 196 615.
    { column: 7, columnCount: 65_536, expected: 196_615, row: 3 },
  ])("places row $row column $column of $columnCount at $expected", (cell) => {
    expect(rowMajorOffset(cell.row, cell.column, cell.columnCount)).toBe(cell.expected);
  });

  test.prop({
    columnCount: fc.integer({ max: 1000, min: 1 }),
    offset: fc.integer({ max: 10 ** 6, min: 0 }),
  })("gives each offset to exactly one row and column", ({ columnCount, offset }) => {
    // The row is the quotient of the offset by the column count, the column its remainder.
    const row = Math.floor(offset / columnCount);
    const column = offset % columnCount;

    expect(rowMajorOffset(row, column, columnCount)).toBe(offset);
  });
});
