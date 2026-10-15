import { describe, expect, it } from "vitest";

import { integerRange } from "../../math";

import { alphaToPercent } from "./alpha-to-percent";
import { percentToAlpha } from "./percent-to-alpha";

describe("percentToAlpha (DERIV-color-spaces, step 2)", () => {
  it.each([
    // ⌊(255 · 50 + 50) / 100⌋ = ⌊128⌋: 127.5 rounded up, not 127 as 50 × 2.55 in binary64.
    { expected: 128, percent: 50 },
    // ⌊(255 · 80 + 50) / 100⌋ = ⌊204.5⌋ = 204, `cc`.
    { expected: 204, percent: 80 },
    // ⌊(255 · 90 + 50) / 100⌋ = ⌊230⌋: 229.5 rounded up, not 229 as 90 × 2.55 in binary64.
    { expected: 230, percent: 90 },
    // ⌊(255 · 93 + 50) / 100⌋ = ⌊237.65⌋ = 237, `ed`.
    { expected: 237, percent: 93 },
    { expected: 0, percent: 0 },
    { expected: 255, percent: 100 },
  ])("[F16.AC1] gives the byte $expected for $percent %", ({ expected, percent }) => {
    expect(percentToAlpha(percent)).toBe(expected);
  });

  it("[F16.AC1] gives each percent from 0 to 100 back from its byte", () => {
    const percents = integerRange(0, 100);

    expect(percents.map((percent) => alphaToPercent(percentToAlpha(percent)))).toEqual(percents);
  });
});
