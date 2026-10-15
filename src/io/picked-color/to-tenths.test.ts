import { describe, expect, it } from "vitest";

import { toTenths } from "./to-tenths";

describe("toTenths (DERIV-color-spaces step 11)", () => {
  it.each([
    // 10 · 30.1 is 301 exactly in binary64, as 10 · (i / 10) is i for every i from 0 to 3600.
    { expected: 301, value: 30.1 },
    // 0.7 + 0.1 = 0.7999999999999999 in binary64, 10 times it 7.999999999999999, rounded to 8.
    { expected: 8, value: 0.7 + 0.1 },
    { expected: 667, value: 66.7 },
    { expected: 3599, value: 359.9 },
    { expected: 0, value: 0 },
    { expected: 1000, value: 100 },
  ])("[F16.AC3] counts $value in tenths: $expected", ({ expected, value }) => {
    expect(toTenths(value)).toBe(expected);
  });

  it("[F16.AC3] counts every tenth from 0 to 360 as its own integer", () => {
    const tenths = Array.from({ length: 3601 }).keys().toArray();

    expect(tenths.map((index) => toTenths(index / 10))).toEqual(tenths);
  });
});
