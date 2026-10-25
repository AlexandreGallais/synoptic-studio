import { describe, expect, it } from "vitest";

import { isFlatBox } from "./is-flat-box";

describe("isFlatBox", () => {
  it.each([
    { expected: true, height: 80, width: 0 },
    { expected: true, height: 0, width: 120 },
    { expected: true, height: 0, width: 0 },
    { expected: false, height: 1, width: 1 },
    { expected: false, height: 100, width: 100 },
  ])("is $expected for a box $width × $height", ({ expected, height, width }) => {
    expect(isFlatBox({ height, width })).toBe(expected);
  });
});
