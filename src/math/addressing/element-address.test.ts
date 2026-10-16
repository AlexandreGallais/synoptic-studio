import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { elementAddress } from "./element-address";

describe("elementAddress (REF-WIKI-ARRAY)", () => {
  it.each([
    // Table records of 16 bytes after a 12-byte header: record 0 at 12, record 2 at 12 + 32 = 44.
    { base: 12, expected: 12, index: 0, stride: 16 },
    { base: 12, expected: 44, index: 2, stride: 16 },
    // Past the last of 3 records: 12 + 48 = 60, the directory's size.
    { base: 12, expected: 60, index: 3, stride: 16 },
    // uint16 glyph ids at 6: the fourth, index 3, at 6 + 6 = 12.
    { base: 6, expected: 12, index: 3, stride: 2 },
    // An array of no stride stays at its base.
    { base: 7, expected: 7, index: 5, stride: 0 },
  ])("places element $index of stride $stride from $base at $expected", (row) => {
    expect(elementAddress(row.base, row.stride, row.index)).toBe(row.expected);
  });

  test.prop({
    base: fc.integer({ max: 2 ** 32, min: 0 }),
    index: fc.integer({ max: 2 ** 16, min: 0 }),
    stride: fc.integer({ max: 64, min: 0 }),
  })("places each next element one stride further", ({ base, index, stride }) => {
    expect(elementAddress(base, stride, index + 1) - elementAddress(base, stride, index)).toBe(
      stride,
    );

    expect(elementAddress(base, stride, 0)).toBe(base);
  });
});
