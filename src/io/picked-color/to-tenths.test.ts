import { describe, expect, it } from "vitest";

import { toTenths } from "./to-tenths";

describe("toTenths (DERIV-color-spaces step 11)", () => {
  it.each([
    // 10 · 30.1 = 301.00000000000006 in binary64, rounded to 301.
    { expected: 301, value: 30.1 },
    { expected: 667, value: 66.7 },
    { expected: 3599, value: 359.9 },
    { expected: 0, value: 0 },
    { expected: 1000, value: 100 },
  ])("[F16.AC3] counts $value in tenths: $expected", ({ expected, value }) => {
    expect(toTenths(value)).toBe(expected);
  });
});
