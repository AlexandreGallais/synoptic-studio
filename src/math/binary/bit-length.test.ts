import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bitLength } from "./bit-length";

describe("bitLength (REF-WIKI-BIT-LENGTH)", () => {
  it.each([
    // The source's table: 0 is the empty string, 1 is 1, 2 is 10, 3 is 11, 4 is 100, 7 is 111,
    // 8 is 1000, 10 is 1010.
    { expected: 0, value: 0 },
    { expected: 1, value: 1 },
    { expected: 2, value: 2 },
    { expected: 2, value: 3 },
    { expected: 3, value: 4 },
    { expected: 3, value: 7 },
    { expected: 4, value: 8 },
    { expected: 4, value: 10 },
    // 2³²: past the 32 bits of the bitwise operators, 33 digits.
    { expected: 33, value: 2 ** 32 },
  ])("writes $value with $expected binary digits", ({ expected, value }) => {
    expect(bitLength(value)).toBe(expected);
  });

  test.prop({ value: fc.integer({ max: 2 ** 40, min: 1 }) })(
    "places the integer from 2^(ℓ − 1) up to 2^ℓ",
    ({ value }) => {
      const length = bitLength(value);

      expect(2 ** (length - 1)).toBeLessThanOrEqual(value);
      expect(value).toBeLessThan(2 ** length);
    },
  );
});
