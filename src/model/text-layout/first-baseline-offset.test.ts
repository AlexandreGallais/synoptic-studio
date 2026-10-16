import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ARIAL_UNKERNED, HELLO_TEXT } from "../../../tests";

import { firstBaselineOffset } from "./first-baseline-offset";

describe("firstBaselineOffset (DERIV-text-box step 7)", () => {
  it("[F14.AC6] puts it the ascent below the top with an automatic line height, as in F05", () => {
    // 1854 × 16 / 2048 = 14.484375.
    expect(firstBaselineOffset(ARIAL_UNKERNED, HELLO_TEXT)).toBe(14.484375);
  });

  it("[F14.AC3] adds half the leading of a line height of 24 above the ascent", () => {
    // (24 − 2288 / 128) / 2 + 14.484375 = 3.0625 + 14.484375 = 17.546875.
    expect(firstBaselineOffset(ARIAL_UNKERNED, { ...HELLO_TEXT, lineHeight: 24 })).toBe(17.546875);
  });

  it.each([
    // 1467 × 16 / 2048 = 11.4609375, whatever the line height.
    { expected: 11.4609375, lineHeight: 0, size: 16 },
    { expected: 11.4609375, lineHeight: 24, size: 16 },
    // 1467 × 32 / 2048 = 22.921875.
    { expected: 22.921875, lineHeight: 0, size: 32 },
  ])(
    "[F14.AC5] puts it the cap height below the top when trimmed: size $size, line height $lineHeight",
    ({ expected, lineHeight, size }) => {
      const text = { ...HELLO_TEXT, isTrimmed: true, lineHeight, size };

      expect(firstBaselineOffset(ARIAL_UNKERNED, text)).toBe(expected);
    },
  );

  test.prop({ size: fc.integer({ max: 100_000, min: 1 }) })(
    "puts the trimmed baseline higher than the untrimmed one, the cap height below the ascent",
    ({ size }) => {
      const text = { ...HELLO_TEXT, size };

      expect(firstBaselineOffset(ARIAL_UNKERNED, { ...text, isTrimmed: true })).toBeLessThan(
        firstBaselineOffset(ARIAL_UNKERNED, text),
      );
    },
  );
});
