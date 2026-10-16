import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ratioValue } from "./ratio-value";

describe("ratioValue (DERIV-color-spaces step 9)", () => {
  it.each([
    // #336699's hue, blue largest, C = 153 − 51 = 102: 60 · (51 − 102 + 4 · 102) / 102
    // = 21 420 / 102 = 210.
    { expected: 210, ratio: { denominator: 102, numerator: 21_420 } },
    { expected: 0.5, ratio: { denominator: 2, numerator: 1 } },
    { expected: 0, ratio: { denominator: 1, numerator: 0 } },
  ])("gives $ratio the value $expected", ({ expected, ratio }) => {
    expect(ratioValue(ratio)).toBe(expected);
  });

  test.prop({
    denominator: fc.integer({ max: 10_000, min: 1 }),
    numerator: fc.integer({ max: 10_000, min: 0 }),
  })(
    "gives back the numerator when multiplied by the denominator, within a rounding error",
    ({ denominator, numerator }) => {
      expect(ratioValue({ denominator, numerator }) * denominator).toBeCloseTo(numerator, 9);
    },
  );
});
