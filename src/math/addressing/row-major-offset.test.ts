import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rowMajorOffset } from "./row-major-offset";

describe("rowMajorOffset (REF-WIKI-ROW-MAJOR)", () => {
  it.each([
    // The source's 2 × 3 array a₁₁ … a₂₃, stored a₁₁ a₁₂ a₁₃ a₂₁ a₂₂ a₂₃: a₂₃, row 1 column 2 from
    // 0, is at 3 · 1 + 2 = 5; a row 2 column 0 would follow at 6.
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
