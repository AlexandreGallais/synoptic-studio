import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { toOneDecimal } from "./to-one-decimal";

describe("toOneDecimal (DERIV-color-spaces step 9)", () => {
  it.each([
    // round(301.1765) / 10.
    { expected: 30.1, value: 30.117647 },
    // round(666.6667) / 10.
    { expected: 66.7, value: 66.666667 },
    // round(501.9608) / 10.
    { expected: 50.2, value: 50.196078 },
    // round(1.9608) / 10: #000001's lightness, 0.19608 %, shown 0.2 %.
    { expected: 0.2, value: 0.196078 },
    // A tie: 10 · 0.05 is 0.5 in binary64, rounded up, so 0.1.
    { expected: 0.1, value: 0.05 },
    { expected: 100, value: 100 },
    { expected: 0, value: 0 },
  ])("[F16.AC3] shows $value as $expected", ({ expected, value }) => {
    expect(toOneDecimal(value)).toBe(expected);
  });

  test.prop({ value: fc.double({ max: 360, min: 0, noNaN: true }) })(
    "is within a twentieth of the value, and is read back as itself",
    ({ value }) => {
      const shown = toOneDecimal(value);

      expect(Math.abs(shown - value)).toBeLessThanOrEqual(0.05 + 1e-12);
      expect(Number(shown.toFixed(1))).toBe(shown);
    },
  );
});
