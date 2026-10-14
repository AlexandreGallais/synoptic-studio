import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { populationCount } from "./population-count";

describe("populationCount (REF-WIKI-HAMMING-WEIGHT)", () => {
  it.each([
    // 0b11101 holds four 1s (the source's first example); 0b11101000 too.
    { expected: 4, value: 0b1_1101 },
    { expected: 4, value: 0b1110_1000 },
    { expected: 0, value: 0 },
    // A ValueFormat of X_PLACEMENT and X_ADVANCE, 0x0005: two fields.
    { expected: 2, value: 0x00_05 },
    { expected: 8, value: 0xff },
    // 2³² + 1: past the 32 bits of the bitwise operators.
    { expected: 2, value: 2 ** 32 + 1 },
  ])("counts $expected bits set in $value", ({ expected, value }) => {
    expect(populationCount(value)).toBe(expected);
  });

  test.prop({ value: fc.integer({ max: 2 ** 40, min: 0 }) })(
    "counts the last bit, then the bits of the rest",
    ({ value }) => {
      // n = 2 · ⌊n / 2⌋ + (n mod 2): its bits are those of ⌊n / 2⌋, then n mod 2.
      expect(populationCount(value)).toBe(populationCount(Math.floor(value / 2)) + (value % 2));
    },
  );
});
